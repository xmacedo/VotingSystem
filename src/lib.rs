use std::sync::{Arc, atomic::AtomicU32};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

// Type Definitions
pub type PollId = u32;
pub type OptionId = u32;
pub type PollStore = Arc<RwLock<HashMap<PollId, Poll>>>;

// Models Structure
#[derive(Clone)]
pub struct AppState {
    pub polls: PollStore,
    pub ws_tx: tokio::sync::broadcast::Sender<Poll>,
    pub next_poll_id: Arc<AtomicU32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct OptionItem {
    pub id: OptionId,
    pub label: String,
    pub votes: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct Poll {
    pub id: PollId,
    pub question: String,
    pub is_open: bool,
    pub options: Vec<OptionItem>,
    pub voters: HashSet<Uuid>, // Set of voter IDs who have voted in this poll
}

#[derive(Debug, Deserialize)]
pub struct VoteRequest {
    pub poll_id: PollId, // ID of the poll being voted in
    pub option_id: OptionId, // ID of the option being voted for
    pub voter_id: Uuid, // unique ID for each voter
}

#[derive(Serialize)]
pub struct ApiError {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct CreatePollRequest {
    pub question: String,
    pub options: Vec<String>, // labels of the options
}

#[derive(Debug, Deserialize)]
pub struct CreateOptionRequest {
    pub label: String,
}