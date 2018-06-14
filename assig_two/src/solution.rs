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

    pub fn get_time(self) -> Time {
        self.ftime
    }

    pub fn set_time(&mut self, t: Time) {
        self.ftime = t
    }

    pub fn get_degree(self) -> i32 {
        self.degree
    }

    pub fn set_degree(&mut self, d: i32) {
        self.degree = d
    }

    pub fn get_x(self) -> f32 {
        self.x
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x
    }

    pub fn get_y(self) -> f32 {
        self.y
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y
    }
}
