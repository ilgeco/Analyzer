use std::{path::Path, fs, process::exit, io::{ErrorKind, Read}};

/// Retrive the content of a file in a particular Path
///
/// ## Return Value
///     Content of the file
///
/// ## Exception
///
///     If file cannot be opened exit with -1
///
pub fn retrive_file(file_name: impl AsRef<Path>) -> String {
    match fs::read_to_string(&file_name) {
        Ok(x) => x,
        Err(_) => {
            eprintln!(
                "{}",
                format!("File {} not found", file_name.as_ref().to_str().unwrap())
            );
            exit(-1)
        }
    }
}

/// Retrive the content from the ClipBoard
///
/// ## Return Value
///     Content of the file
///
/// ## Exception
/// 
///     If file cannot be opened exit with -1
///
pub fn retrive_clip() -> String {
    match terminal_clipboard::get_string() {
        Ok(x) => x,
        Err(_) => {
            eprintln!("Clipboard fail");
            exit(-1)
        }
    }
}

/// Retrive a String from stdin
///
/// ## Return Value
///
///     * Ok(String) if stdin != ""
///     * Error otherwise
pub fn retrive_stdin() -> std::io::Result<String> {
    if atty::isnt(atty::Stream::Stdin) {
        let mut buffer = String::new();
        std::io::stdin().lock().read_to_string(&mut buffer)?;

        Ok(buffer)
    } else {
        Err(std::io::Error::new(ErrorKind::Other, "oh no!"))
    }
}


/// Retrive a String following this priority
/// File > stdin > clip
/// ## Return Value
///
///     * Ok(String) if stdin != ""
///     * Error otherwise
pub fn retrive_string(file1: Option<impl AsRef<Path>>) -> String{
    match file1{
        Some(file1) => retrive_file(file1),
        None => match retrive_stdin() {
            Ok(x) => x,
            Err(_) => retrive_clip(),
        },
    }
}
