use time::Time;

pub struct Solution {
    ftime: Time,
    degree: i32,
    x: f32,
    y: f32,
}

impl Solution {
    pub fn new(ftime: Time, degree: i32, x: f32, y: f32) -> Self {
        Solution {
            ftime: ftime,
            degree: degree,
            x: x,
            y: y,
        }
    }

    pub fn getTime(self) -> Time {
        self.ftime
    }

    pub fn setTime(&mut self, t: Time) {
        self.ftime = t
    }

    pub fn getDegree(self) -> i32 {
        self.degree
    }

    pub fn setDegree(&mut self, d: i32) {
        self.degree = d
    }

    pub fn getX(self) -> f32 {
        self.x
    }

    pub fn setX(&mut self, x: f32) {
        self.x = x
    }

    pub fn getY(self) -> f32 {
        self.y
    }

    pub fn setY(&mut self, y: f32) {
        self.y = y
    }
}
