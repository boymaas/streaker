use load_dotenv::load_dotenv;

try_load_dotenv!();

pub const API_ENDPOINT: &'static str = env!("STREAKER_API_ENDPOINT");
pub const WSS_ENDPOINT: &'static str = env!("STREAKER_WSS_ENDPOINT");
