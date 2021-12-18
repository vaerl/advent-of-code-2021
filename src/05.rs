use std::{collections::HashSet, fs, ops::RangeInclusive};

fn main() {
    println!("Day 05-01:");
    // hydrothermal_venture();
    println!("Day 05-02:");
    hydrothermal_venture_diagonal();
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

fn hydrothermal_venture_diagonal() {
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
            if a.x == b.x {
                get_iter(a.y, b.y)
                    .map(|y| Point { x: a.x, y })
                    .collect::<Vec<Point>>()
            } else if a.y == b.y {
                get_iter(a.x, b.x)
                    .map(|x| Point { x, y: a.y })
                    .collect::<Vec<Point>>()
            } else {
                let x_op = if a.x < b.x {
                    |index: isize| index + 1
                } else {
                    |index: isize| index - 1
                };

                let y_op = if a.y < b.y {
                    |index: isize| index + 1
                } else {
                    |index: isize| index - 1
                };

                let mut x_index = a.x;
                let mut y_index = a.y;

                let mut res: Vec<Point> = vec![];

                while x_index != b.x && y_index != b.y {
                    res.push(Point {
                        x: x_index,
                        y: y_index,
                    });
                    x_index = x_op(x_index);
                    y_index = y_op(y_index);
                }

                return res;
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
    // to low: 21277
}
