
use honggfuzz::fuzz;
use skim::global::mark_new_run;

fn main() {
    // Define the fuzzing loop
    loop {
        // Fuzzing input will be provided by Honggfuzz
        fuzz!(|data: &[u8]| {
            // Convert the input to a string
            if let Ok(name) = std::str::from_utf8(data) {
                // Call the mark_new_run function with the fuzzed input
                let _ = mark_new_run(name);

            }
        });
    }
}