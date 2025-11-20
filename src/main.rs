use axum::{ extract::State, http::StatusCode, Json };
// use tokio::sync::RwLock;
// use std::sync::Arc;
use std::collections::{HashSet, HashMap};
use voting_system::{ AppState, VoteRequest, Poll, OptionItem };

pub async fn vote(State(state): State<AppState>, Json(payload): Json<VoteRequest>) -> StatusCode {
    let mut polls = state.polls.write().await;

    let Some(poll) = polls.get_mut(&payload.poll_id) else {
        // poll não encontrada
        return StatusCode::NOT_FOUND;
    };

    if !poll.is_open {
        // poll closed
        return StatusCode::FORBIDDEN;
    }

    // Is voter has already voted in this poll?
    if poll.voters.contains(&payload.voter_id) {
        // usuário já votou nessa poll
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

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let mut polls_map = HashMap::new();
    // let polls = Arc::new(RwLock::new(polls_map));

    polls_map.insert("poll_1".to_string(), Poll {
        id: "poll_1".into(),
        question: "Which is the best programming Language?".into(),
        is_open: true,
        voters: HashSet::new(),
        options: vec![
            OptionItem { id: "rust".into(), label: "Rust".into(), votes: 0 },
            OptionItem { id: "go".into(), label: "Go".into(), votes: 0 },
            OptionItem { id: "java".into(), label: "Java".into(), votes: 0 }
        ],
    });

    println!("{:#?}", polls_map);
    
}
