// fn get_dial_number(value: i32) -> i32 {
//     ((value % 100) + 100) % 100
// }

// current_position: current position on the dial
// rotation_tick: how many ticks to rotate (can be negative for left rotation)
// dial_size: total size of the dial (e.g., 100 for a 0-99 dial)
// fn passes_over_zero(current_position: i32, rotation_tick: i32, dial_size: i32) -> i32 {
//     let mut passed_zero_count = 0;
//     let mut rotation_tick = rotation_tick;
//     println!(
//         "Calculating passes over zero: current_position = {}, rotation_tick = {}, dial_size = {}",
//         current_position, rotation_tick, dial_size
//     );
//     if (rotation_tick > dial_size) || (rotation_tick < -dial_size) {
//         passed_zero_count += rotation_tick.abs() / dial_size;
//         dbg!(rotation_tick %= dial_size);
//     }
//     let new_position = (current_position + rotation_tick).rem_euclid(dial_size);
//     println!(
//         "Current position: {}, Rotation tick: {}, New position: {}",
//         current_position, rotation_tick, new_position
//     );
//     passed_zero_count +=
//         (current_position.div_euclid(dial_size) - new_position.div_euclid(dial_size)).abs();
//     // Handle edge case where the value ends exactly on zero
//     passed_zero_count
// }

// fn passes_over_target(current_position: i32, rotation_tick: i32, dial_size: i32, target: i32) -> i32 {
//     let new_position = current_position + rotation_tick;
//     let passed = ((current_position - target).div_euclid(dial_size))
//         - ((new_position - target).div_euclid(dial_size));
//     passed.abs()
// }

fn main() {
    let input = include_str!("day_01_input.txt");
    let dial_size = 100;
    let target_number = 0;

    let mut current_dial_number = 50;
    let mut passes_target_counter = 0;

    for line in input.lines() {
        if let Some((dir, num)) = line
            .chars()
            .next()
            .and_then(|d| line[1..].parse::<i32>().ok().map(|n| (d, n)))
        {
            let rotation_tick = match dir {
                'R' => num,
                'L' => -num,
                _ => 0,
            };

            let passes = {
                let new_position = current_dial_number + rotation_tick;
                let passed = ((current_dial_number - target_number).div_euclid(dial_size))
                    - ((new_position - target_number).div_euclid(dial_size));
                let mut passes = passed.abs();
                let normalized_new = ((new_position % dial_size) + dial_size) % dial_size;
                let normalized_start = ((current_dial_number % dial_size) + dial_size) % dial_size;

                if passes == 0
                    && normalized_new == target_number
                    && normalized_start != target_number
                    && rotation_tick != 0
                {
                    passes += 1;
                }

                if passes == 1
                    && normalized_start == target_number
                    && rotation_tick < dial_size
                    && rotation_tick > -dial_size
                {
                    passes -= 1;
                }

                passes
            };

            passes_target_counter += passes;

            current_dial_number =
                ((current_dial_number + rotation_tick) % dial_size + dial_size) % dial_size;
        }
        println!(
            "Current Dial Number: {}, Passes over target number {}: {}",
            current_dial_number, target_number, passes_target_counter
        );
    }

    println!("Passed by target count: {}", passes_target_counter);
}
