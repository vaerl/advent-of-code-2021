use std::fs;

fn main() {
    println!("Day 01:");

    sonar_sweep();
    sonar_sweep_windowed();
}

/// Day 01-01
fn sonar_sweep() {
    let contents =
        fs::read_to_string("./data/01.txt").expect("Something went wrong reading the file");

    let mut lines = contents
        .lines()
        .map(|e| e.to_string().parse::<i32>().unwrap())
        .peekable();

    let mut counter = 0;

    while let (Some(current), Some(next)) = (lines.next(), lines.peek()) {
        if current < *next {
            counter += 1;
        }
    }

    println!("Found {} increases.", counter);
}

/// Day 01-02
fn sonar_sweep_windowed() {
    let contents =
        fs::read_to_string("./data/01.txt").expect("Something went wrong reading the file");

    let numbers: Vec<i32> = contents
        .lines()
        .map(|e| e.to_string().parse::<i32>().unwrap())
        .collect();

    let summed_windows = &numbers[..]
        .windows(3)
        .map(|v| v.iter().sum::<i32>())
        .collect::<Vec<i32>>();

    let mut peek = summed_windows.iter().peekable();

    let mut counter = 0;

    while let (Some(current), Some(next)) = (peek.next(), peek.peek()) {
        if current < next {
            counter += 1;
        }
    }

    println!("Found {} increases.", counter);
}
