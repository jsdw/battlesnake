use crate::api::{ request, response };

/// A Stupid snake that just goes down, screaming all the way to the wall.
#[derive(Clone)]
pub struct DownSnake;

impl super::Snake for DownSnake {
    fn iter_moves(&mut self, _turn: request::Turn) -> Box<dyn Iterator<Item=response::Move>> {
        Box::new(std::iter::once(response::Move {
            movement: response::Movement::Down,
            shout: Some("Aaaahhhhh!".to_string())
        }))
    }
}
