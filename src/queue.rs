use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct QueueItem
{
    pub id: Uuid
}


#[derive(Clone, Debug)]
pub struct JobQueue
{

}

impl JobQueue {
    pub fn new()->Self {
        JobQueue{}
    }

    pub fn enqueue(_item: &QueueItem) {

    }

    pub fn dequeue()->Option<QueueItem> {
        None
    }

}