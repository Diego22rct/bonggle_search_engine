use std::collections::VecDeque;

#[derive(Debug)]
pub struct Network {
    queue: VecDeque<i32>,
}

impl Network {
    pub fn new() -> Network {
        Network {
            queue: VecDeque::new(),
        }
    }

    pub fn add_to_nt_queue(&mut self, value: i32) {
        if self.queue.contains(&value) {
            return;
        }
        self.queue.push_back(value);
    }

    pub fn get_nt_queue(&self) -> &VecDeque<i32> {
        &self.queue
    }

    pub fn remove_from_nt_queue(&mut self, value: i32) {
        if let Some(pos) = self.queue.iter().position(|&x| x == value) {
            self.queue.remove(pos);
        }
    }
}
