#[derive(Copy, Clone)]
pub struct Time {
    pub hour: i32,
    pub minute: i32,
    pub second: i32,
}

impl Time {
    pub fn new() -> Self {
        Time {
            hour: 0,
            minute: 0,
            second: 0,
        }
    }

    pub fn assign(&mut self, hour: i32, minute: i32, second: i32) {
        self.hour = hour;
        self.minute =  minute;
        self.second = second;
    }

    pub fn inc_hour(&mut self) {
        self.hour += 1;
    }

    pub fn inc_minute(&mut self) {
        self.minute += 1;

        if self.minute == 60 {
            self.minute = 0;
            Time::inc_hour(self);
        }
    }

    pub fn inc_second(&mut self) {
        self.second += 1;

        if self.second == 60 {
            self.second = 0;
            Time::inc_minute(self);
        }
    }

    pub fn to_string(&mut self) -> String {
        if self.hour >= 12 {
            if self.hour == 12 {
                return format!("{:02}:{:02}:{:02} PM", 12, self.minute, self.second);
            } else {
                return format!("{:02}:{:02}:{:02} PM", self.hour - 12, self.minute, self.second);
            }
        } else {
            if self.hour == 0 {
                return format!("{:02}:{:02}:{:02} AM", 12, self.minute, self.second);
            } else {
                return format!("{:02}:{:02}:{:02} AM", self.hour, self.minute, self.second);
            }
        }
    }

    pub fn to_mil_string (&mut self) -> String {
        return format!("{:02}:{:02}:{:02}", self.hour, self.minute, self.second);
    }

    pub fn secs_since_midnight(self) -> i32 {
        self.second + (self.minute * 60) + (self.hour * 60 * 60)
    }

    pub fn equals(self, t: Time) -> bool {
        self.secs_since_midnight() == t.secs_since_midnight()
    }
}

#[cfg(test)]
mod test {
    use super::Time;

    #[test]
    fn basic_tests() {
        let mut time = Time::new();
        time.assign(12, 30 , 0);

        assert_eq!(time.secs_since_midnight(), 45000);

        let mut second_time = Time::new();
        second_time.assign(12, 30, 0);
        assert_eq!(time.equals(second_time), true);
    }

    #[test]
    fn inc_tests() {
        let mut time = Time::new();
        time.assign(0, 58, 59);

        time.inc_hour();
        time.inc_minute();
        time.inc_second();
        assert_eq!(time.secs_since_midnight(), 7200);
    }

    #[test]
    fn string_tests() {
        let mut time = Time::new();
        time.assign(12, 0, 0);

        assert_eq!(time.to_string(), "12:00:00 PM");
        time.assign(0, 0, 0);
        assert_eq!(time.to_string(), "12:00:00 AM");
        assert_eq!(time.to_mil_string(), "00:00:00");
    }
}
