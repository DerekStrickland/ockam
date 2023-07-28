use clap::Args;
use colorful::Colorful;
use indoc::formatdoc;
use std::net::SocketAddr;

use crate::run::ConfigRunner;
use crate::util::local_cmd;
use crate::util::parsers::socket_addr_parser;
use crate::{docs, fmt_info, CommandGlobalOpts};

const LONG_ABOUT: &str = include_str!("./static/secure_relay/long_about.txt");
const AFTER_LONG_HELP: &str = include_str!("./static/secure_relay/after_long_help.txt");

/// Create and setup a new relay node, idempotent
#[derive(Clone, Debug, Args)]
#[command(
long_about = docs::about(LONG_ABOUT),
after_long_help = docs::after_help(AFTER_LONG_HELP)
)]
pub struct SecureRelayOutlet {
    /// The name of the service
    #[arg(value_name = "SERVICE NAME")]
    pub service_name: String,

    /// TCP address to send raw tcp traffic.
    #[arg(long, display_order = 902, id = "SOCKET_ADDRESS", value_parser = socket_addr_parser)]
    to: SocketAddr,

    /// If using Okta enrollment
    #[arg(long = "okta", group = "authentication_method")]
    pub okta: bool,

    /// Enrollment ticket to use
    #[arg(
        long,
        value_name = "ENROLLMENT TICKET PATH",
        group = "authentication_method"
    )]
    pub enroll_ticket: Option<String>,
}

impl SecureRelayOutlet {
    pub fn run(self, opts: CommandGlobalOpts) {
        local_cmd(self.create_config_and_start(opts));
    }
}

impl SecureRelayOutlet {
    pub fn create_config_and_start(self, opts: CommandGlobalOpts) -> miette::Result<()> {
        let stdout = opts.terminal.stdout();
        let enroll: String = if let Some(enroll_ticket) = self.enroll_ticket.as_ref() {
            format! {
                "enroll-ticket: {enroll_ticket}",
                enroll_ticket = enroll_ticket
            }
        } else {
            "okta: true".to_string()
        };

        let config: String = formatdoc! {
            r#"
            nodes:
              secure_relay_outlet:
                {enroll}
                tcp-outlets:
                  {service_name}:
                    from: '/service/outlet_{service_name}'
                    to: {to}
                    access_control: '(= subject.component "{service_name}")'
                relays:
                  {service_name}:
                    at: /project/default
            "#,
            enroll = enroll,
            to = self.to.to_string(),
            service_name = self.service_name,
        };

        stdout
            .plain(fmt_info!(
                r#"Creating new outlet relay node using this configuration:
```
{}```
       You can copy and customize the above configuration and launch it with `ockam run`.
"#,
                config.as_str().dark_gray()
            ))
            .write_line()?;

        ConfigRunner::go_inline(&opts.state, &config)
    }
}
