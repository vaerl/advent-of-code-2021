use std::fs;

fn main() {
    println!("Day 02-01:");
    binary_diagnostic();
    println!("Day 02-02:");
    life_support_rating()
}

struct Amount {
    zeroes: usize,
    ones: usize,
}

fn invert_binary(input: String) -> String {
    input
        .chars()
        .map(|c| match c {
            '1' => "0".to_string(),
            '0' => "1".to_string(),
            _ => panic!("Could not identify char {}!", c),
        })
        .reduce(|a, b| format!("{}{}", a, b))
        .unwrap()
}

fn binary_diagnostic() {
    let contents =
        fs::read_to_string("./data/03.txt").expect("Something went wrong reading the file");

    // get the chars
    let data: Vec<Vec<char>> = contents
        .lines()
        .map(|e: &str| e.chars().collect::<Vec<char>>())
        .collect();

    // flip the input
    let gamma = data
        .iter()
        .fold(vec![vec![]; data[0].len()], |mut acc, e: &Vec<char>| {
            e.iter()
                .enumerate()
                .for_each(|(index, el)| acc[index].push(*el));
            acc
        })
        .iter()
        .map(|v| {
            v.iter()
                .fold(Amount { zeroes: 0, ones: 0 }, |acc, e| match e {
                    '0' => Amount {
                        zeroes: acc.zeroes + 1,
                        ones: acc.ones,
                    },
                    '1' => Amount {
                        zeroes: acc.zeroes,
                        ones: acc.ones + 1,
                    },
                    _ => panic!("Could not identify char {}!", e),
                })
        })
        .map(|a| {
            if a.zeroes > a.ones {
                "0".to_string()
            } else {
                "1".to_string()
            }
        })
        .reduce(|a, b| format!("{}{}", a, b))
        .unwrap();

    let gamma_num = isize::from_str_radix(&gamma, 2).unwrap();
    println!("Gamma as binary: {:?}", gamma);
    println!("Gamma as number: {}", gamma_num);

    let epsilon = invert_binary(gamma);
    let epsilon_num = isize::from_str_radix(&epsilon, 2).unwrap();
    println!("Epsilon as binary: {:?}", epsilon);
    println!("Epsilon as number: {}", epsilon_num);

    println!("Multiplied: {}", gamma_num * epsilon_num);
}

fn life_support_rating() {
    let contents =
        fs::read_to_string("./data/03.txt").expect("Something went wrong reading the file");

    // get the chars
    let data: Vec<Vec<char>> = contents
        .lines()
        .map(|e: &str| e.chars().collect::<Vec<char>>())
        .collect();
    let mut current_data_oxygen = data.to_vec();
    let mut current_data_scrubber = data.to_vec();
    // for each column n of the data:
    // - get the amount of ones and zeroes
    // - remove all elements that do not have the most common number at index n

    for index in 0..data[0].len() {
        if current_data_oxygen.len() == 1 {
            break;
        }
        let amount = current_data_oxygen.iter().map(|v| v[index]).fold(
            Amount { zeroes: 0, ones: 0 },
            |acc, e| match e {
                '0' => Amount {
                    zeroes: acc.zeroes + 1,
                    ones: acc.ones,
                },
                '1' => Amount {
                    zeroes: acc.zeroes,
                    ones: acc.ones + 1,
                },
                _ => panic!("Could not identify char {}!", e),
            },
        );

        let expected_char = if amount.zeroes > amount.ones {
            '0'
        } else {
            '1'
        };

        current_data_oxygen = current_data_oxygen
            .into_iter()
            .filter(|e| e[index] == expected_char)
            .collect();
    }

    for index in 0..data[0].len() {
        if current_data_scrubber.len() == 1 {
            break;
        }

        let amount = current_data_scrubber.iter().map(|v| v[index]).fold(
            Amount { zeroes: 0, ones: 0 },
            |acc, e| match e {
                '0' => Amount {
                    zeroes: acc.zeroes + 1,
                    ones: acc.ones,
                },
                '1' => Amount {
                    zeroes: acc.zeroes,
                    ones: acc.ones + 1,
                },
                _ => panic!("Could not identify char {}!", e),
            },
        );

        let expected_char = if amount.zeroes > amount.ones {
            '1'
        } else {
            '0'
        };

        current_data_scrubber = current_data_scrubber
            .into_iter()
            .filter(|e| e[index] == expected_char)
            .collect();
    }

    println!(
        "{:?}",
        current_data_oxygen
            .first()
            .unwrap()
            .into_iter()
            .map(char::to_string)
            .reduce(|a, b| format!("{}{}", a, b))
            .unwrap()
    );
    let oxygen = current_data_oxygen
        .first()
        .unwrap()
        .into_iter()
        .map(char::to_string)
        .reduce(|a, b| format!("{}{}", a, b))
        .unwrap();
    let oxygen_num = isize::from_str_radix(&oxygen, 2).unwrap();

    let scrubber = current_data_scrubber
        .first()
        .unwrap()
        .into_iter()
        .map(char::to_string)
        .reduce(|a, b| format!("{}{}", a, b))
        .unwrap();
    let scrubber_num = isize::from_str_radix(&scrubber, 2).unwrap();

    println!("Oxygen-Bin: {:?}", oxygen);
    println!("Oxygen: {:?}", oxygen_num);
    println!("Scrubber-Bin: {:?}", scrubber);
    println!("Scrubber: {:?}", scrubber_num);
    println!("Multiplied: {:?}", oxygen_num * scrubber_num);
}
