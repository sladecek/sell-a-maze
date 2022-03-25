use serde::Serialize;
use serde_derive::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[warn(non_camel_case_types)]
pub enum State {
    WaitingForPayment,
    InProgress,
    Done,
    Error,
}

impl Default for State {
    fn default() -> Self {
        State::WaitingForPayment
    }
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Size {
    Small,
    Medium,
    Large,
    Huge,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MazeType {
    Rectangular,
    Circular,
    Triangular,
    Hexagonal,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Job {
    #[serde(default)]
    pub state: State,
    pub size: Size,
    #[serde(rename = "type")]
    pub maze_type: MazeType,
    pub guaranteed: bool,
    #[serde(default)]
    pub payment: String,
    #[serde(default)]
    pub svg: String,
    #[serde(default)]
    pub pdf: String,
    #[serde(default)]
    pub maze_structure: String,
    #[serde(default)]
    pub maze_instance: String
}

impl Job {
    pub fn is_in_progress(&self) -> bool {
        self.state == State::InProgress || self.state == State::WaitingForPayment
    }
}
