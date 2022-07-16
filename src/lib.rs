use std::io;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub trait Widget {
    fn draw<W: io::Write>(&self, term: &mut W) -> io::Result<()>;
}

pub mod raw;
pub mod input;
pub mod event;