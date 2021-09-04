mod http_server;
mod api;
mod error;
mod snakes;

use structopt::StructOpt;
use hyper::{Body, Method, Request, Response};
use api::{ request, response };
use error::Error;
use std::collections::HashMap;
use snakes::{ Snake, DownSnake };
use std::sync::{ Arc, Mutex };

#[derive(StructOpt,Debug)]
struct Opts {
    /// Address that the battlesnake server will listen on
    #[structopt(short = "l", long = "listen", default_value = "0.0.0.0:8888")]
    addr: std::net::SocketAddr,
}

/// This much latency is allowed between moves. For now, we just take the max of 500
/// and remove some amount to account for latency.
const ALLOWED_LATENCY: std::time::Duration = std::time::Duration::from_millis(400);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    let opts = Opts::from_args();

    // Our game state. Snakes are taken out of this when they
    // are moving, and put back when they are done, so that multiple
    // snakes can be moved simultaneously without long term locking.
    let games: Arc<Mutex<HashMap<String, Option<Box<dyn Snake>>>>>
        = Arc::new(Mutex::new(HashMap::new()));

    http_server::start_server(http_server::Opts {
        addr: opts.addr,
        on_error: log_internal_errors,
        handler: move |req: Request<Body>, _addr| {
            let games = games.clone();
            async move {
                let method = req.method();
                let path = req.uri().path().trim_matches('/');

                match (method, path) {
                    // Return some basic battlesnake info:
                    (&Method::GET, "") => {
                        Ok::<_, Error>(json_response(&response::Info {
                            apiversion: "1".to_string(),
                            author: Some("jsdw".to_string()),
                            color: Some("#ff69b4".to_string()),
                            head: None,
                            tail: None,
                            version: Some("0.0.1".to_string()),
                        }))
                    },
                    // Battlesnake server has asked to start a new game!
                    (&Method::POST, "start") => {
                        let turn: request::Turn = body_into_json(req).await?;
                        let game_id = turn.game.id;

                        // Put a new snake into play:
                        games.lock().unwrap().insert(game_id.clone(), Some(Box::new(DownSnake)));
                        // Clean up after ourselves after a while to limit the impact of malicious calls:
                        tokio::spawn({
                            let games = games.clone();
                            async move {
                                tokio::time::sleep(tokio::time::Duration::from_secs(600)).await;
                                games.lock().unwrap().remove(&game_id);
                            }
                        });

                        Ok(Response::default())
                    },
                    // Make a move!
                    (&Method::POST, "move") => {
                        let turn: request::Turn = body_into_json(req).await?;
                        let game_id = turn.game.id.clone();

                        // Pull out the snake associated with this game:
                        let mut snake = match games.lock().unwrap().get_mut(&game_id) {
                            Some(snake) => {
                                match snake.take() {
                                    Some(snake) => snake,
                                    None => return Err(Error::Them(400, "This snake is busy moving".to_string()))
                                }
                            },
                            None => return Err(Error::Them(400, "This snake doesn't exist".to_string()))
                        };
                        // Find the best move we can in roughly the time allowed:
                        let (best_move, snake) = tokio::task::spawn_blocking(move || {
                            let start_time = std::time::Instant::now();
                            let mut moves = snake.iter_moves(turn);
                            let mut best_move = None;
                            while let Some(curr_move) = moves.next() {
                                best_move = Some(curr_move);
                                if std::time::Instant::now().duration_since(start_time) > ALLOWED_LATENCY {
                                    break;
                                }
                            };
                            (best_move, snake)
                        }).await?;
                        // Put our snake back, ready for the next turn. If the snake "holder" was removed (proabably
                        // because the game ended or lasted too long), we do nothing and let our snake be thrown away.
                        if let Some(res) = games.lock().unwrap().get_mut(&game_id) {
                            *res = Some(snake);
                        }

                        // Respond with our best move, or a default "up" move if we didn't get a best move in time:
                        Ok(json_response(&best_move.unwrap_or_default()))
                    },
                    // End the game!
                    (&Method::POST, "end") => {
                        let turn: request::Turn = body_into_json(req).await?;

                        // Remove our snake; the game is over
                        games.lock().unwrap().remove(&turn.game.id);

                        Ok(Response::default())
                    },
                    // All other requests are unknown and handled with a 404.
                    _ => {
                        Ok(Response::builder()
                            .status(404)
                            .body(Body::from("Woa, that ain't a path I recognise"))
                            .unwrap())
                    }
                }
            }
        },
    }).await?;

    Ok(())
}

/// If we did something wrong, log it:
fn log_internal_errors(e: &Error) {
    if let Error::Us(e) = e {
        log::error!("Internal error: {}", e)
    }
}

/// Extract the JSON body from the request, 400 if not valid
async fn body_into_json<T: serde::de::DeserializeOwned>(req: Request<Body>) -> Result<T, Error> {
    let bytes = hyper::body::to_bytes(req.into_body()).await?;
    match serde_json::from_slice(&bytes) {
        Ok(val) => Ok(val),
        Err(e) => Err(Error::Them(400, e.to_string()))
    }
}

/// Hand back a response containing the JSON of the value provided
fn json_response<T: serde::Serialize>(val: &T) -> Response<Body> {
    let bytes = serde_json::to_vec(val).expect("value can be serialized");
    Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::from(bytes))
        .unwrap()
}

