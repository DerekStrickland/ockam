use ockam_api::cloud::share::{InviteList, ReceivedInvitation, SentInvitation};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct State<R, S> {
    pub(crate) sent: Vec<S>,
    pub(crate) received: Vec<R>,
}

pub type InviteState = State<ReceivedInvitation, SentInvitation>;

impl Default for State<ReceivedInvitation, SentInvitation> {
    fn default() -> Self {
        Self {
            sent: vec![],
            received: vec![],
        }
    }
}

impl From<InviteList> for InviteState {
    fn from(val: InviteList) -> Self {
        let InviteList { sent, received } = val;
        Self {
            sent: sent.unwrap_or_default(),
            received: received.unwrap_or_default(),
        }
    }
}
