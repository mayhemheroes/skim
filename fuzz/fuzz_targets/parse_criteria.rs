
use honggfuzz::fuzz;
use skim::item::parse_criteria;

fn main() {
    // Define the fuzzing loop
    loop {
        // Fuzzing input will be provided by Honggfuzz
        fuzz!(|data: &[u8]| {
            // Convert the input to a string
            if let Ok(name) = std::str::from_utf8(data) {
                // Call the parse_criteria function with the fuzzed input
                let _ = parse_criteria(name);

            }
        });
    }
}