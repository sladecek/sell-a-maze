use std::{collections::VecDeque, sync::Mutex};

use uuid::Uuid;

pub struct Queue {
    pub uids: Mutex<VecDeque<Uuid>>
}
