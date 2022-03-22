use std::default;

use serde_derive::Deserialize;


#[derive(Debug, Deserialize)]
#[warn(non_camel_case_types)]
enum State {
    new, done, error
}

impl Default for State {
    fn default() -> Self {
        State::new
    }
}
#[derive(Debug, Deserialize)]
enum Size {
    small, medium, large, huge
}

#[derive(Debug, Deserialize)]
enum MazeType {
    rectangular, circular, triangular, hexagonal
}

#[derive(Debug, Deserialize)]
pub struct Job
{
    #[serde(default)]
    state: State,
    size: Size,
    #[serde(rename="type")]
    maze_type: MazeType,
    guaranteed: bool,
    payment: String 
}

impl Job {

}