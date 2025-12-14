fn get_dial_number(value: i32) -> i32 {
    ((value % 100) + 100) % 100
}

fn main() {
    let input = include_str!("day_01_input.txt");
    let mut counter = 0;
    let mut dial_number = 50;

    for line in input.lines() {
        if let Some((dir, num)) = line
            .chars()
            .next()
            .and_then(|d| line[1..].parse::<i32>().ok().map(|n| (d, n)))
        {
            match dir {
                'R' => dial_number += num,
                'L' => dial_number -= num,
                _ => panic!("Unknown direction: {}", dir),
            }
            dial_number = get_dial_number(dial_number);

            if dial_number == 0 {
                counter += 1;
            }

            println!("Dial: {}, Counter: {}", dial_number, counter);
        }
    }
}
