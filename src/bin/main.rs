use sell_a_maze::{
    job::{Job, MazeType, Size, State},
    maker::MazeMaker,
};

fn main() {

    MazeMaker::make(
        &Job {
            state: State::Done,
            maze_type: MazeType::Circular,
            size: Size::Large,
            payment: String::from(""),
            guaranteed: false,
            svg: String::from("maze.svg"),
            pdf: String::from("maze.pdf"),
            maze_structure: String::from("maze.mas"),
            maze_instance: String::from("maze.mai"),
            protocol: String::from("protocol.log")
        }
    );
    
}
