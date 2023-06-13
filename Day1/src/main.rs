use std::fs;

fn main() {
    let content = fs::read_to_string("./input.txt").expect("Trouble reading file");

    // Accumulator to accrue all the values before an empty line.
    let mut accumulator: u32 = 0;

    // Vec<u32> to contain all of the accumulated values before empty lines.
    let mut accumulated: Vec<u32> = Vec::new();

    // Grab every line in the file seperately
    for line in content.lines() {
        // Trim the line for later use.
        let trimmed_line = line.trim();
        // Check if we hit an empty line, this is the point where our accumulation needs to finish.
        let is_empty = trimmed_line.len() == 0;
        if is_empty {
            // If the empty line is found, we want to store the current accumulator value in a vector
            // For later use.
            accumulated.push(accumulator);
            accumulator = 0;
        } else {
            // If the empty line is not found, we want to parse the current trimmed line to become
            // a u32 and add it to the current accumulation.
            accumulator += trimmed_line.parse::<u32>().expect("Could not trim");
        }
    }

    // Sort the vector so we can just grab the last element of the list
    // We could do this in O(n) time, however I will do it in O(nlog(n)) due to the second requirement
    accumulated.sort();

    let highest = accumulated
        .get(accumulated.len() - 1)
        .expect("Accessing memory beyond our scope");
    let second_highest = accumulated
        .get(accumulated.len() - 2)
        .expect("Accessing memory beyond our score");
    let third_highest = accumulated
        .get(accumulated.len() - 3)
        .expect("Accessing memoery beyond our scope");

    let top_three_total = highest + second_highest + third_highest;

    println!("The elf with the highest amount of calorie snacks has: {highest}");
    println!("The elves with the top 3 highest amounts have: {top_three_total}");
}
