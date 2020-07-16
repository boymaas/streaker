pub mod dashboard;
pub mod index;
pub mod login;
pub mod scan;
pub mod scan_complete;

use yew_router::prelude::*;

#[derive(Switch, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[to = "/scans"]
    Scans,
    #[to = "/cashouts"]
    CashOuts,
    #[to = "/dashboard"]
    DashBoard,
    #[to = "/login"]
    Login,
    #[to = "/"]
    Index,
}
