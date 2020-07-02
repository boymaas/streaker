use serde::{Deserialize, Serialize};

type VisitorId = String;
type Token = String;

/// This type is used as a request which sent to websocket connection.
#[derive(Serialize, Deserialize, Debug)]
pub enum WsRequest {}

// Member state
#[derive(Serialize, Deserialize, Debug)]
pub struct MemberState {
    pub visitorid: String,
    pub bucket: i32,
    pub streak_total: i32,
    pub streak_bucket: i32,
    pub balance: f64,
    pub email: Option<String>,
}

// This is the response after
// successfull connecion
#[derive(Serialize, Deserialize, Debug)]
pub enum WsResponse {
    Connected,
    Attribution(Token),
    MemberState(MemberState),
    DoubleConnection,
    Error(String),
}
