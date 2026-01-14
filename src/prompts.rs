use std::io::{self, Write};
use std::process;

pub fn prompt_input<T, F, E>(prompt: String, default: T, conv: F) -> Result<T, io::Error>
where
    F: FnOnce(String) -> Result<T, E>,
    E: std::fmt::Display,
{
    print!("{}: ", prompt);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let input = input.trim().to_string();

    if input.is_empty() {
        return Ok(default);
    }

    conv(input).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e.to_string()))
}

pub fn y_or_exit(prompt: &str, default: bool) -> io::Result<()> {
    print!("{}: ", prompt);

    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let input = input.trim().to_string();

    if default && input.is_empty() {
        return Ok(());
    }

    if input.to_lowercase() != "y" {
        process::exit(0);
    }

    Ok(())
}

pub fn prompt_selection<T: Clone>(prompt: &str, options: &[(T, &str)]) -> io::Result<T> {
    println!("{}:", prompt);
    for (i, (_, label)) in options.iter().enumerate() {
        println!("  {}. {}", i + 1, label);
    }

    print!("Selection (1): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let input = input.trim();

    if input.is_empty() {
        return Ok(options[0].0.clone());
    }

    let selection: usize = input
        .parse()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    if selection == 0 || selection > options.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Selection out of range",
        ));
    }

    Ok(options[selection - 1].0.clone())
}

pub fn new_section() {
    println!("");
    println!("==========================================");
    println!("");
}

pub fn exit_with_message(message: &str) {
    println!("{}", message);
    process::exit(0);
}
