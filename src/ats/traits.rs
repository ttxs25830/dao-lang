use std::error::Error;
pub trait SourceReader {
    fn get_char() -> char;
    fn end_sentence();
    fn send_error(error: dyn Error);
}
