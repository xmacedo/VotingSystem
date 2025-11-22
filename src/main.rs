//
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

use std::{
    collections::{HashMap},
    net::SocketAddr,
    sync::{Arc, atomic::AtomicU32},
};

use tokio::sync::{broadcast, RwLock};

use voting_system::{ AppState, VoteRequest, Poll, PollId, ApiError };

// ENDPOINTS

// POST /vote -> cast a vote
pub async fn vote(
    State(state): State<AppState>, 
    Json(payload): Json<VoteRequest>
) -> (StatusCode, Json<ApiError>) {

    let mut polls = state.polls.write().await;

    let Some(poll) = polls.get_mut(&payload.poll_id) else {
        // poll NOT FOUND
        return (
            StatusCode::NOT_FOUND,
            Json(ApiError { message: "Poll não encontrada".into() })
        );
    };

    if !poll.is_open {
        // poll closed
        return (
            StatusCode::FORBIDDEN,
            Json(ApiError { message: "A votação está encerrada".into() })
        );
    }

    // has this voter already voted in this poll?
    if poll.voters.contains(&payload.voter_id) {
        // User has already voted
        return (
            StatusCode::CONFLICT,
            Json(ApiError { message: "Usuário já votou nessa poll".into() })
        );
    }

    // Find the option and increment its vote count
    if let Some(option) = poll.options.iter_mut().find(|opt| opt.id == payload.option_id) {
        option.votes += 1;
        poll.voters.insert(payload.voter_id);

        // Notify via WebSocket
        let _ = state.ws_tx.send(poll.clone());

        return (
            StatusCode::OK,
            Json(ApiError { message: "Voto registrado com sucesso".into() })
        );
    }
        // option not found
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiError { message: "Opção não encontrada nessa poll".into() })
        );
    
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
    println!("Starting server......");

    // Initialize polls store
    let polls_map: HashMap<PollId, Poll> = HashMap::new();

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
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("🚀 Server running on http://{}", addr);

    // create TCP listener
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    
}
