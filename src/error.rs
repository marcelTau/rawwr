#[derive(Debug, Clone)]
pub enum ScannerError {
    Default,
    Error {
        line: i32,
        message: String,
    },
}

impl ScannerError {
    pub fn report(error: &ScannerError) -> () {
        match error {
            ScannerError::Error {
                line,
                message,
            } => {
                println!("{} at (line: {})", message, line);
            }
            _ => (),
        }
    }
}
