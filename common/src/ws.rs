use serde::{Deserialize, Serialize};

type VisitorId = String;

/// This type is used as a request which sent to websocket connection.
#[derive(Serialize, Deserialize, Debug)]
pub enum WsRequest {}

// This is the response after
// successfull connecion
#[derive(Serialize, Deserialize, Debug)]
pub enum WsResponse {
    Connected,
    Authenticated(VisitorId),
    DoubleConnection,
}