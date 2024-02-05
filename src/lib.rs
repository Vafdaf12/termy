use std::io;

#[cfg(test)]
mod tests;

pub trait Widget {
    fn draw<W: io::Write>(&self, term: &mut W) -> io::Result<()>;
}

pub mod input;
pub mod raw;
