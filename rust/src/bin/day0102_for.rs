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
            let step = match dir {
                'R' => 1,
                'L' => -1,
                _ => 0,
            };
            let ticks = num.abs();

            // Use for loop
            // Increment or decrement step by step
            // Increment the counter if it hits the target number
            // Edge case: Do not increment if the starting position is the target number
            let mut passes = 0;
            for _ in 0..ticks {
                current_dial_number = (current_dial_number + step + dial_size) % dial_size;
                if current_dial_number == target_number {
                    passes += 1;
                }
            }
            passes_target_counter += passes;
        }
        println!(
            "Current Dial Number: {}, Passes over target number {}: {}",
            current_dial_number, target_number, passes_target_counter
        );
    }
    println!(
        "Total Passes over target number {}: {}",
        target_number, passes_target_counter
    );
}
