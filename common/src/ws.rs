use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use crate::streak_logic::StreakState;

type VisitorId = String;
type Token = String;

/// This type is used as a request which sent to websocket connection.
#[derive(Serialize, Deserialize, Debug)]
pub enum WsRequest {}

// Member state
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemberState {
    pub visitorid: String,
    pub streak_current: i32,
    pub streak_bucket: i32,
    pub balance: f64,
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScanSessionState {
    pub uuid: Uuid,
    pub count: u16,
    pub total: u16,
    pub next_anode: Option<String>,
    pub begin: DateTime<Utc>,
}

// This is the response after
// successfull connecion
#[derive(Serialize, Deserialize, Debug)]
pub enum WsResponse {
    Connected,
    Attribution(Token),
    // when tring to connect to wss
    // with a bad token, we give back
    // a valid auth token so frontend
    // can reconnect
    BadToken(Token),
    MemberState(MemberState),
    ScanSessionState(ScanSessionState),
    StreakState(StreakState),
    DoubleConnection,
    Error(String),
}
