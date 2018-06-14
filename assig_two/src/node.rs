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

    pub fn getData(self) -> Solution {
        self.data
    }

    pub fn getNext(self) -> Link {
        self.next
    }

    pub fn setData(&mut self, s: Solution) {
        self.data = s
    }

    pub fn setNext(&mut self, l: Link) {
        self.next = l
    } 
}
