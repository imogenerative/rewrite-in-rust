use std::ptr;

use solution::Solution;

pub struct Node {
    data: Solution,
    next: Link,
}

enum Link {
    Empty,
    More(Box<Node>)
}

impl Node {
    pub fn new(solution: Solution) -> Self {
        Node {
            data: solution,
            next: Link::Empty,
        }
    }

    pub fn get_data(self) -> Solution {
        self.data
    }

    pub fn get_next(self) -> Link {
        self.next
    }

    pub fn set_data(&mut self, s: Solution) {
        self.data = s
    }

    pub fn set_next(&mut self, l: Link) {
        self.next = l
    } 
}
