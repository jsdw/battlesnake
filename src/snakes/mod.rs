mod down_snake;

use crate::api::{ request, response };

pub use down_snake::DownSnake;

/// All snakes implement this trait
pub trait Snake: Send {
    /// Called when we make a move. Returns an iterator, which gives the
    /// snake the chance to continue improving on the suggested move until the
    /// server decides that it no longer has time, and responds with whatever
    /// the last move it's made it to is. This shouldn't take too long to respond in
    /// each iteration; aim for <10ms, so that we can yield a result and don't
    /// overrun the allowed time.
    fn iter_moves(&mut self, turn: request::Turn) -> Box<dyn Iterator<Item=response::Move>>;
}