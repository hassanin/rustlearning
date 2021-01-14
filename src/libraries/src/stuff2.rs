use serde::export::fmt::Debug;
use std::io::{self, Write};

/// Trait for values to which you can send HTML.
trait WriteHtml {
    fn write_html(&mut self, html: &str) -> io::Result<()>;
}
/// You can write HTML to any std::io writer.
impl<W: Write> WriteHtml for W {
    fn write_html(&mut self, html: &str) -> io::Result<()> {
        println!("Wrote html {}", html);
        Ok(())
    }
}

/// Print out all the values produced by an iterator
fn dump<I>(iter: I)
where
    I: Iterator,
    I::Item: Debug,
{
    for (index, value) in iter.enumerate() {
        println!("{}: {:?}", index, value); // error
    }
}

trait Mohamed {
    type Item2;
    type Item1;
}
