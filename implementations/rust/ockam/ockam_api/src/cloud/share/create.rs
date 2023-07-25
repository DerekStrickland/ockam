use minicbor::{Decode, Encode};
use serde::Serialize;

use ockam_identity::IdentityIdentifier;

use super::SentInvitation;

#[derive(Clone, Debug, Decode, Encode, Serialize)]
#[cbor(map)]
#[rustfmt::skip]
pub struct CreateServiceInvite {
    #[n(1)] pub expires_at: Option<String>,
    #[n(2)] pub project_id: String,
    #[n(3)] pub recipient_email: String,

    // TODO: Should route be a MultiAddr?
    #[n(4)] pub project_identity: IdentityIdentifier,
    #[n(5)] pub project_route: String,
    #[n(6)] pub project_authority_identity: IdentityIdentifier,
    #[n(7)] pub project_authority_route: String,
    #[n(8)] pub shared_node_identity: IdentityIdentifier,
    #[n(9)] pub shared_node_route: String,
}

mod node {
    use ockam_core::api::{Request, Response};
    use ockam_core::{self, Result};
    use ockam_multiaddr::MultiAddr;
    use ockam_node::Context;
    use tracing::trace;

    use crate::cloud::CloudRequestWrapper;
    use crate::nodes::{NodeManager, NodeManagerWorker};

    use super::*;

    const API_SERVICE: &str = "users";

    impl NodeManager {
        // TODO: Should identity_name not be an Option for this use-case?
        //       This is an authenticated-only request
        pub async fn create_service_invite(
            &self,
            ctx: &Context,
            req: CreateServiceInvite,
            route: &MultiAddr,
            identity_name: Option<String>,
        ) -> Result<SentInvitation> {
            Response::parse_response_body(
                self.create_service_invite_response(
                    ctx,
                    CloudRequestWrapper::new(req, route, identity_name),
                )
                .await?
                .as_slice(),
            )
        }

        pub(crate) async fn create_service_invite_response(
            &self,
            ctx: &Context,
            req_wrapper: CloudRequestWrapper<CreateServiceInvite>,
        ) -> Result<Vec<u8>> {
            let cloud_multiaddr = req_wrapper.multiaddr()?;
            let req_body = req_wrapper.req;

            let label = "create_service_invite";
            trace!(project_id = %req_body.project_id, "creating service invite");

            let req_builder = Request::post("/v0/invites/service").body(req_body);

            self.request_controller(
                ctx,
                label,
                "create_service_invite",
                &cloud_multiaddr,
                API_SERVICE,
                req_builder,
                None,
            )
            .await
        }
    }

    impl NodeManagerWorker {
        pub async fn create_service_invite(
            &self,
            ctx: &Context,
            req: CreateServiceInvite,
            route: &MultiAddr,
            identity_name: Option<String>,
        ) -> Result<SentInvitation> {
            let node_manager = self.inner().read().await;
            node_manager.create_service_invite(ctx, req, route, identity_name).await
        }

        pub(crate) async fn create_service_invite_response(
            &self,
            ctx: &Context,
            req_wrapper: CloudRequestWrapper<CreateServiceInvite>,
        ) -> Result<Vec<u8>> {
            let node_manager = self.inner().read().await;
            node_manager
                .create_service_invite_response(ctx, req_wrapper)
                .await
        }
    }
}
