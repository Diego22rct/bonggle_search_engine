use std::collections::VecDeque;

#[derive(Debug)]
struct Network {
    queue: VecDeque<i32>,
}

impl Network {
    fn new() -> Network {
        Network {
            queue: VecDeque::new(),
        }
    }

    fn add_to_nt_queue(&mut self, value: i32) {
        self.queue.push_back(value);
    }

    fn remove_from_nt_queue(&mut self) -> Option<i32> {
        self.queue.pop_front()
    }
}
