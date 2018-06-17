use time::Time;

#[derive (Copy, Clone)]
pub struct Solution {
    time: Time,
    degree: i32,
    x: f32,
    y: f32,
}

impl Solution {
    pub fn new(time: Time, degree: i32, x: f32, y: f32) -> Self {
        Solution {
            time: time,
            degree: degree,
            x: x,
            y: y,
        }
    }

    pub fn set_time(&mut self, t: Time) {
        self.time = t;
    }

    pub fn set_degree(&mut self, d: i32) {
        self.degree = d;
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    pub fn get_time(&mut self) -> Time{
        self.time
    }

    pub fn get_degree(&mut self) -> i32{
        self.degree
    }

    pub fn get_x(&mut self) -> f32 {
        self.x
    }

    pub fn get_y(&mut self) -> f32 {
        self.y
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use super::Time;

    #[test]
    fn solution_tests() {
        let mut time = Time::new();
        let mut solution = Solution::new(time, 0, 0.0, 0.0);

        // check setters all work
        let mut second_time = Time::new();
        second_time.assign(12, 0, 0);
        solution.set_time(second_time);
        solution.set_degree(180);
        solution.set_x(1.0);
        solution.set_y(2.0);

        // confirm setters populated the solution
        assert_eq!(solution.time.secs_since_midnight(), 43200);
        assert_eq!(solution.degree, 180);
        assert_eq!(solution.x, 1.0);
        assert_eq!(solution.y, 2.0);
    }
}
