use super::{
    add::AddRequest, setup::SetupRequest, status::StatusRequest, sync::SyncRequest,
    CommandOutcome, GoldenPathRoute,
};
use sxmc::error::Result;

pub(crate) struct GoldenPathApp {
    route: GoldenPathRoute,
}

impl GoldenPathApp {
    pub(crate) fn current() -> Self {
        Self {
            route: GoldenPathRoute::current(),
        }
    }

    pub(crate) fn run_add(&self, request: AddRequest) -> Result<CommandOutcome> {
        super::add::AddService::new().run(self.route, request)
    }

    pub(crate) fn run_setup(&self, request: SetupRequest) -> Result<CommandOutcome> {
        super::setup::SetupService::new().run(self.route, request)
    }

    pub(crate) async fn run_status(&self, request: StatusRequest) -> Result<CommandOutcome> {
        super::status::StatusService::new()
            .run(self.route, request)
            .await
    }

    pub(crate) fn run_sync(&self, request: SyncRequest) -> Result<CommandOutcome> {
        super::sync::SyncService::new().run(self.route, request)
    }
}
