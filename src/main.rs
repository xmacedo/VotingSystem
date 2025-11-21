//
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
    sync::{Arc, atomic::AtomicU32},
};

use tokio::sync::{broadcast, RwLock};

use voting_system::{ AppState, VoteRequest, Poll, OptionItem, PollId };

// ENDPOINTS

// POST /vote -> cast a vote
pub async fn vote(
    State(state): State<AppState>, 
    Json(payload): Json<VoteRequest>
) -> StatusCode {

    let mut polls = state.polls.write().await;

    let Some(poll) = polls.get_mut(&payload.poll_id) else {
        // poll NOT FOUND
        return StatusCode::NOT_FOUND;
    };

    if !poll.is_open {
        // poll closed
        return StatusCode::FORBIDDEN;
    }

    // has this voter already voted in this poll?
    if poll.voters.contains(&payload.voter_id) {
        // User has already voted
        return StatusCode::CONFLICT; // 409
    }

    // Find the option and increment its vote count
    if let Some(option) = poll.options.iter_mut().find(|opt| opt.id == payload.option_id) {
        option.votes += 1;
        poll.voters.insert(payload.voter_id);

        // Notify via WebSocket
        let _ = state.ws_tx.send(poll.clone());

        StatusCode::OK
    } else {
        // option not found
        StatusCode::BAD_REQUEST
    }
}

// GET /polls -> list all polls
async fn list_polls(State(state): State<AppState>) -> Json<HashMap<PollId, Poll>> {
    let polls = state.polls.read().await;
    Json(polls.clone())
}

// GET /polls/:poll_id -> details of a specific poll
async fn get_poll(
    State(state): State<AppState>,
    Path(poll_id): Path<PollId>,
) -> Result<Json<Poll>, StatusCode> {
    let polls = state.polls.read().await;
    if let Some(poll) = polls.get(&poll_id) {
        Ok(Json(poll.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

// MAIN

#[tokio::main]
async fn main() {
    println!("STARTING SERVER...");

    // Initialize polls store
    let mut polls_map: HashMap<PollId, Poll> = HashMap::new();


    /*polls_map.insert("poll_1".to_string(), Poll {
        id: "poll_1".into(),
        question: "Which is the best programming Language?".into(),
        is_open: true,
        voters: HashSet::new(),
        options: vec![
            OptionItem { id: "rust".into(), label: "Rust".into(), votes: 0 },
            OptionItem { id: "go".into(), label: "Go".into(), votes: 0 },
            OptionItem { id: "java".into(), label: "Java".into(), votes: 0 }
        ],
    });*/

    //println!("{:#?}", polls_map);

    // shared polls store
    let polls = Arc::new(RwLock::new(polls_map));

    // WebSocket broadcast channel
    let (ws_tx, _ws_rx) = broadcast::channel(100);

    //
    let next_poll_id = Arc::new(AtomicU32::new(2));
    
    // application state
    let state = AppState { polls, ws_tx, next_poll_id };

    // build app with routes
    let app = Router::new()
        .route("/vote", post(vote))
        .route("/polls", get(list_polls))
        .route("/polls/:poll_id", get(get_poll))
        .with_state(state);

    // start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running on http://{}", addr);

    // create TCP listener
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    
}
