use std::fs;

fn main() {
    println!("Day 05-02:");
    part_two();
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn from_string(input: String) -> Point {
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

        Point {
            x: coordinates[0],
            y: coordinates[1],
        }
    }
}

struct Line {
    a: Point,
    b: Point,
}

impl Line {
    fn from_string(input: String) -> Line {
        let points: Vec<Point> = input
            .split("->")
            .map(str::trim)
            .map(str::to_string)
            .map(Point::from_string)
            .collect();

        assert!(points.len() == 2, "Could not parse line!");
        Line {
            a: points[0],
            b: points[1],
        }
    }

    fn is_on(&self, point: Point) -> bool {
        self.is_on_vertical(point) || self.is_on_horizontal(point) || self.is_on_diagonal(point)
    }

    fn is_on_vertical(&self, point: Point) -> bool {
        if self.a.x == self.b.x && self.a.x == point.x {
            if self.a.y < self.b.y {
                return point.y >= self.a.y && point.y <= self.b.y;
            } else {
                return point.y <= self.a.y && point.y >= self.b.y;
            }
        }
        false
    }

    fn is_on_horizontal(&self, point: Point) -> bool {
        if self.a.y == self.b.y && self.a.y == point.y {
            if self.a.x < self.b.x {
                return point.x >= self.a.x && point.x <= self.b.x;
            } else {
                return point.x <= self.a.x && point.x >= self.b.x;
            }
        }
        false
    }

    fn is_on_diagonal(&self, point: Point) -> bool {
        let a_x = self.a.x - point.x;
        let a_y = self.a.y - point.y;

        let b_x = self.b.x - point.x;
        let b_y = self.b.y - point.y;

        let diagonal = a_x.abs() == a_y.abs() && b_x.abs() == b_y.abs();
        let x: bool;
        let y: bool;

        // check if the point is inside the given interval
        if self.a.x < self.b.x {
            x = point.x >= self.a.x && point.x <= self.b.x;
        } else {
            x = point.x <= self.a.x && point.x >= self.b.x;
        }

        if self.a.y < self.b.y {
            y = point.y >= self.a.y && point.y <= self.b.y;
        } else {
            y = point.y <= self.a.y && point.y >= self.b.y;
        }

        diagonal && x && y
    }
}

fn part_two() {
    let contents =
        fs::read_to_string("./data/05.txt").expect("Something went wrong reading the file");

    let lines: Vec<Line> = contents
        .lines()
        .map(str::to_string)
        .map(Line::from_string)
        .collect();

    let max_y = lines
        .iter()
        .map(|l| vec![l.a, l.b])
        .flatten()
        .map(|v| v.y)
        .max()
        .unwrap();
    let max_x = lines
        .iter()
        .map(|l| vec![l.a, l.b])
        .flatten()
        .map(|v| v.x)
        .max()
        .unwrap();

    let mut counter = 0;
    for x in 0..=max_x {
        for y in 0..=max_y {
            if lines
                .iter()
                .map(|l| l.is_on(Point { x, y }))
                .filter(|e| *e)
                .count()
                > 1
            {
                counter += 1;
            }
        }
    }

    println!("Counter: {:?}", counter);
}
