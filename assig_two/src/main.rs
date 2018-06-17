use std::f32;

mod list;
mod solution;
mod time;

use list::List;
use solution::Solution;
use time::Time;

fn max(x: f32, y: f32) -> f32 {
    if x > y {
        x
    } else {
        y
    }
}

fn radians(d: f32) -> f32 {
    let rad: f32 = d / 360.0 * 2.0 * f32::consts::PI;

    rad
}

fn add(l: List, s: Solution) {
    // This is handled in list.rs
    unimplemented!();
}

fn aim(l: &mut List, t: Time) {
    const LAUNCHTOWN: i32 = 0;
    const DOOMSVILLE: i32 = 180;
    const UP: i32 = 90;
    const MOVEMENT: f32 = 43.635;
    const AIM: f32 = 35.0;
    const CHARGE: f32 = 65.0;

    let mut missile_pos: i32 = LAUNCHTOWN;
    let mut time: f32 = 0.0;
    let mut laser_pos: i32 = UP;
    let mut targeted: bool = false;
    let mut laser_shift: i32;
    let mut missile_shift: i32;
    let mut atime = Time::new();
    let mut solution = Solution::new(atime, 0,  0.0, 0.0);
    let mut degree_laser: i32;
    let mut degree_missile: i32;
    let mut x: f32;
    let mut y: f32;
    let mut now = Time::new();

    while missile_pos <= DOOMSVILLE {
        degree_missile = missile_pos;
        degree_missile += 1;
        degree_laser = laser_pos;
        targeted = false;

        while targeted == false && degree_missile != LAUNCHTOWN && degree_missile < DOOMSVILLE {
            laser_shift = (degree_missile - degree_laser).abs();
            missile_shift = (missile_pos - degree_missile).abs();

            if max(CHARGE, laser_shift as f32 * AIM) <= (missile_shift as f32) * MOVEMENT {
                x = 5000.0 * (degree_missile as f32).cos();
                y = 50.0 * (degree_missile as f32).sin();
                degree_laser = degree_missile;
                laser_pos = degree_laser;
                time += missile_shift as f32 * MOVEMENT;
                now.assign(12, 30, 0);
                now.add_time(time);
                solution.set_time(now);
                solution.set_degree(degree_missile);
                solution.set_x(x);
                solution.set_y(y);
                l.push(solution);
                targeted = true;

            } else {
                degree_missile += 1;
            }
        }
    }
}

fn display(l: List) {
    let mut atime = Time::new();
    let mut solution = Solution::new(atime, 0,  0.0, 0.0);

    while l::Link == list::Link::More { 
          solution = l.pop().unwrap();
        println!("At time {}, fire at {:3} degrees to impact missile at ({:9.3},{:7.3}.)", solution.get_time().to_string(), solution.get_degree(), solution.get_x(), solution.get_y())
    }
}

fn main() {
    let mut now = Time::new();
    let mut list = List::new();

    now.assign(12, 30, 0);

    aim(&mut list, now);
    display(list);
 
}
