use std::io;
enum fooError {
    Io(io::Error),
    ParseError(String),
}

impl From<io::Error> for fooError {
    fn from(error: io::Error) -> Self {
        fooError::Io(error)
    }
}



fn main() {
    let f = File::open("hello.txt").unwrap();
}
