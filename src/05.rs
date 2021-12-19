use std::{collections::HashSet, fs, ops::RangeInclusive};

fn main() {
    println!("Day 05-01:");
    hydrothermal_venture();
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    /// Expects a string of the format "x,y"!
    fn from(input: String) -> Point {
        let coordinates = input
            .split(",")
            .map(str::parse::<isize>)
            .map(Result::unwrap)
            .collect::<Vec<isize>>();
        assert!(
            coordinates.len() == 2,
            "Something went wrong parsing the input {}!",
            input
        );
        return Point {
            x: coordinates[0],
            y: coordinates[1],
        };
    }
}

fn get_iter(a: isize, b: isize) -> RangeInclusive<isize> {
    if a < b {
        a..=b
    } else {
        b..=a
    }
}

fn hydrothermal_venture() {
    let contents =
        fs::read_to_string("./data/05.txt").expect("Something went wrong reading the file");
    let lines: Vec<Point> = contents
        .lines()
        .map(|f| {
            let points: Vec<Point> = f
                .split("->")
                .map(str::trim)
                .map(str::to_string)
                .map(Point::from)
                .collect();

            assert!(points.len() == 2, "Could not parse line!");
            return (points[0], points[1]);
        })
        .map(|(a, b)| {
            // only consider horizontal or vertical lines
            if a.x == b.x {
                get_iter(a.y, b.y)
                    .map(|f| Point { x: a.x, y: f })
                    .collect::<Vec<Point>>()
            } else if a.y == b.y {
                get_iter(a.x, b.x)
                    .map(|f| Point { x: f, y: a.y })
                    .collect::<Vec<Point>>()
            } else {
                vec![]
            }
        })
        .filter(|v| v.len() > 0)
        .flatten()
        .collect::<Vec<Point>>();

    let mut unique_points: HashSet<Point> = HashSet::new();
    lines.iter().for_each(|f| {
        unique_points.insert(*f);
    });

    let amount = unique_points
        .iter()
        .map(|a| lines.iter().filter(|&b| a == b).count())
        .filter(|&e| e > 1)
        .count();

    println!("Points used more than once: {:?}", amount);
}
