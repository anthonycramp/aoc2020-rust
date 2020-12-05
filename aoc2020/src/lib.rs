use std::env;
use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::path::Path;

/// Returns the string contents of the file supplied as the first command line argument.
pub fn get_input_string_from_file() -> std::io::Result<String> {
    let working_dir = env::current_dir()?;
    let input_file_name = get_input_file_name()?;

    match read_file_as_string(&Path::new(&input_file_name)) {
        Err(why) => panic!(
            "Error reading from file {} (in {}): {}",
            input_file_name,
            working_dir.display(),
            why
        ),
        Ok(s) => Ok(s),
    }
}

/// Get the first argument on the command line (assumed to be a file name). Return
/// an error if there is no first argument.
pub fn get_input_file_name() -> std::io::Result<String> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => Err(Error::new(
            ErrorKind::Other,
            "Usage: cargo run -p <dayXX> -- <name of file containing input>",
        )),
        _ => Ok(String::from(&args[1])),
    }
}

/// Return the contents of the supplied file as a string
pub fn read_file_as_string(path: &Path) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    Ok(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
