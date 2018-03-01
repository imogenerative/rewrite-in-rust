use std::io;

fn main() {
    println!("The Local Chocolate Factory");
    println!("---------------------------");
    println!();

    println!("Enter the minimum number of banana pieces: ");
    let min_pieces: i32 = input().trim().parse::<i32>().unwrap();
    println!("Enter the maximum number of banana pieces: ");
    let max_pieces: i32 = input().trim().parse::<i32>().unwrap();
    println!("Enter the curry concentration step size: ");
    let concentration: f32 = input().trim().parse::<f32>().unwrap();

    check(min_pieces, max_pieces, concentration);

    println!("\n\tNumber of banana pieces");
    print!("Curry\t");

    for pieces in min_pieces..(max_pieces + 1) {
        print!("{}\t", pieces)
    }
    println!();

    let mut step: f32  = concentration;
    let mut cont: bool = true;

    while cont {
        print!("{:0.2}\t", step);
        for pieces in min_pieces..(max_pieces + 1) {
            let flavour: f32 = sfa(pieces, step);
            print!("{:5.1}\t", flavour);
            cont = cont && (flavour <= 500.0);
        }
        println!();
        step += concentration;
        cont = cont && (step <= 1.0);
    }
}

fn input() -> String {
    let mut buffer = String::new();

    io::stdin()
        .read_line(&mut buffer)
        .expect("Could not read input.");

    buffer
}

fn check(min: i32, max: i32, curry: f32) {
    assert!(min >= 0, "Banana pieces must be positive.");
    assert!(max >= 0, "Banana pieces must be positive.");
    assert!(curry >= 0.0, "Curry concentration must be positive.");

    assert!(max - min >= 0, "Minimum pieces greater than maximum pieces.");
    assert!(max - min <= 10,
            "Difference between minimum and maximum pieces wust be within 10.");

    assert!(curry <= 1.0, "Curry concentration must not be greater than 1.0.")
}

fn sfa(pieces: i32, concentration: f32) -> f32 {
    let factor: f32 = 120.0 - pieces as f32;
    let flavour: f32 = concentration * (800.0 - factor * factor);
    flavour
}
