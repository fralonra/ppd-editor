use std::collections::{vec_deque::Iter, VecDeque};

use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct FixedVec<T: PartialEq + Clone> {
    #[serde(skip)]
    capacity: usize,
    data: VecDeque<T>,
}

impl<T> FixedVec<T>
where
    T: PartialEq + Clone,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            data: VecDeque::new(),
        }
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn iter(&self) -> Iter<T> {
        self.data.iter()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn push(&mut self, value: T) {
        if let Some(position) = self.data.iter().position(|v| *v == value) {
            self.data.remove(position);
        }

        self.data.push_front(value);

        while self.data.len() > self.capacity {
            self.data.pop_back();
        }
    }

    pub fn resize(&mut self, size: usize, value: T) {
        self.capacity = size;

        if self.data.len() > size {
            self.data.resize(size, value);
        }
    }
}
