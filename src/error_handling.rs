pub static mut HAS_ERROR: bool = false;

pub unsafe fn report(line: u64, location: &str, message: &str) {
    eprintln!("[{}] Error {}: {}", line, location, message);
    HAS_ERROR = true;
}

pub unsafe fn error(line: u64, message: &str) {
    report(line, "", message);
}