pub mod dashboard;
pub mod index;
pub mod login;

use yew_router::prelude::*;

#[derive(Switch, Debug, Clone)]
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
