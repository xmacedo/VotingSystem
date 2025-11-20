use std::sync::Arc;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use tokio::sync::RwLock;
use serde::Deserialize;

pub type PollStore = Arc<RwLock<HashMap<String, Poll>>>;

#[derive(Clone)]
pub struct AppState {
    pub polls: PollStore,
    pub ws_tx: tokio::sync::broadcast::Sender<Poll>,
}

#[derive(Debug, Clone)]
pub struct OptionItem {
    pub id: String,
    pub label: String,
    pub votes: u64,
}

#[derive(Debug, Clone)]
pub struct Poll {
    pub id: String,
    pub question: String,
    pub is_open: bool,
    pub options: Vec<OptionItem>,
    pub voters: HashSet<Uuid>, // quem já votou nessa poll
}

#[derive(Deserialize)]
pub struct VoteRequest {
    pub poll_id: String, // ID of the poll being voted in
    pub option_id: String, // ID of the option being voted for
    pub voter_id: Uuid, // unique ID for each voter
}