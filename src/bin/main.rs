use num_bigint::BigUint;
use sell_a_maze::{
    job::{Job, MazeType, Size, State},
    maker::MazeMaker,
};
use std::str::FromStr;

fn main() {
  

    MazeMaker::make(
        &Job {
            state: State::Done,
            maze_type: MazeType::Circular,
            size: Size::Large,
            payment: String::from(""),
            guaranteed: false,
        },
        "",
    );
    
}
