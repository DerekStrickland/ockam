use ockam_api::cloud::project::Project;

pub(crate) mod commands;
mod error;
pub(crate) mod events;
pub(super) mod plugin;

pub(crate) type State = Vec<Project>;
