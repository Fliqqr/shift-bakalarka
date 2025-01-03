use rand::seq::SliceRandom;
use rand::thread_rng;

/*
Function to process input data, this only works
if the function does not care about the order of the input.
*/
fn process_data(input: &[u64]) -> Result<bool, String> {
    if input[0] == 0 {
        return Err("edge-case".into());
    }

    Ok(true)
}

fn main() -> Result<(), String> {
    let mut input: Vec<u64> = vec![0, 1, 2];

    // Keep retrying the function until it succeeds
    let output = loop {
        match process_data(&input) {
            Err(error) => {
                println!("error: {}", error);

                // Reshuffle input
                input.shuffle(&mut thread_rng());
            }
            Ok(output) => break output,
        }
    };

    println!("correct output: {}", output);

    Ok(())
}
