use chrono::{DateTime, Duration, Utc};
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
pub struct AccessNode {
    pub label: String,
    pub url: String,
    pub weight: f64,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScanSessionState {
    pub uuid: Uuid,
    pub count: u16,
    pub total: u16,
    pub next_anode: Option<AccessNode>,
    pub begin: DateTime<Utc>,
}

impl ScanSessionState {
    pub fn end(&self) -> DateTime<Utc> {
        self.begin + Duration::hours(24)
    }
}

impl Default for ScanSessionState {
    fn default() -> ScanSessionState {
        ScanSessionState {
            uuid: Uuid::nil(),
            count: 0,
            total: 0,
            next_anode: None,
            begin: Utc::now().date().and_hms(0, 0, 0),
        }
    }
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
