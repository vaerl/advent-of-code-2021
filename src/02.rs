use std::fs;

fn main() {
    println!("Day 02-01:");
    dive();
    println!("Day 02-02:");
    dive_and_aim()
}

/// Day 02-01
fn dive() {
    let contents =
        fs::read_to_string("./data/02.txt").expect("Something went wrong reading the file");

    let mut depth = 0;
    let mut position = 0;

    contents
        .lines()
        .map(|l| l.split(" ").collect())
        .map(|v: Vec<&str>| (v[0], v[1].parse::<i32>().unwrap()))
        .for_each(|(d, n)| match d {
            "forward" => position += n,
            "up" => depth -= n,
            "down" => depth += n,
            &_ => panic!("Found undefined value {}!", d),
        });

    println!("At position {} and depth {}.", position, depth);
    println!("Multiplied: {}", position * depth)
}

/// Day 02-02
fn dive_and_aim() {
    let contents =
        fs::read_to_string("./data/02.txt").expect("Something went wrong reading the file");

    let mut depth = 0;
    let mut position = 0;
    let mut aim = 0;

    contents
        .lines()
        .map(|l| l.split(" ").collect())
        .map(|v: Vec<&str>| (v[0], v[1].parse::<i32>().unwrap()))
        .for_each(|(d, n)| match d {
            "forward" => {
                position += n;
                depth += aim * n
            }
            "up" => aim -= n,
            "down" => aim += n,
            &_ => panic!("Found undefined value {}!", d),
        });

    println!("At position {} and depth {}.", position, depth);
    println!("Multiplied: {}", position * depth)
}
