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
