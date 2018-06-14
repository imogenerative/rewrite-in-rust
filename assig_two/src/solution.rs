use time::Time;

#[derive (Copy, Clone)]
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

#[cfg(test)]
mod test {
    use super::Solution;
    use super::Time;

    #[test]
    fn solution_tests() {
        let mut time = Time::new();
        let mut solution = Solution::new(time, 0, 0.0, 0.0);

        let mut second_time = Time::new();
        second_time.assign(12, 0, 0);
        solution.set_time(second_time);
        solution.set_degree(180);
        solution.set_x(1.0);
        solution.set_y(2.0);

        assert_eq!(solution.get_time().secs_since_midnight(), 43200);
        assert_eq!(solution.get_degree(), 180);
        assert_eq!(solution.get_x(), 1.0);
        assert_eq!(solution.get_y(), 2.0);
    }
}
