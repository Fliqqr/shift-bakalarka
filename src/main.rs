fn unsafe_process(input: u64) -> Result<u64, String> {
    if input < 5 {
        return Err("Number less than 5".into());
    }

    Ok(input + 2)
}

fn acceptance_check(res: Result<u64, String>) -> Result<u64, String> {
    let output = match res {
        Err(error) => {
            // Handle obvious error
            return Err(format!("Process failed with error: {}", error));
        }
        Ok(output) => output,
    };

    // Perform other checks to ensure output matches our criteria
    // that might have not been tested during output generation

    if output > 10 {
        return Err("Number larger than 10".into());
    }

    Ok(output)
}

fn main() -> Result<(), String> {
    let output = acceptance_check(unsafe_process(6))?;
    println!("Output: {}", output);

    acceptance_check(unsafe_process(2))?;

    Ok(())
}
