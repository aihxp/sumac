pub(crate) mod golden_path;
pub(crate) mod status;

pub(crate) const GOLDEN_PATH_ROUTE_ENV: &str = "SXMC_GOLDEN_PATH_ROUTE";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum GoldenPathRoute {
    Core,
    Legacy,
}

impl GoldenPathRoute {
    pub(crate) fn current() -> Self {
        match std::env::var(GOLDEN_PATH_ROUTE_ENV)
            .unwrap_or_else(|_| "core".to_string())
            .to_ascii_lowercase()
            .as_str()
        {
            "legacy" => Self::Legacy,
            _ => Self::Core,
        }
    }
}

#[derive(Default)]
pub(crate) struct CommandOutcome {
    pub(crate) exit_code: Option<i32>,
}
