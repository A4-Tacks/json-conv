pub struct HjsonCompactFormatter;
impl serde_hjson::ser::Formatter for HjsonCompactFormatter {
    fn open<W>(&mut self, writer: &mut W, ch: u8) -> serde_hjson::Result<()>
    where
        W: std::io::Write,
    {
        writer.write(&[ch])?;
        Ok(())
    }

    fn comma<W>(&mut self, writer: &mut W, first: bool) -> serde_hjson::Result<()>
    where
        W: std::io::Write,
    {
        if !first {
            writer.write(b"\n")?;
        }
        Ok(())
    }

    fn colon<W>(&mut self, writer: &mut W) -> serde_hjson::Result<()>
    where
        W: std::io::Write,
    {
        writer.write(b":")?;
        Ok(())
    }

    fn close<W>(&mut self, writer: &mut W, ch: u8) -> serde_hjson::Result<()>
    where
        W: std::io::Write,
    {
        writer.write(&[ch])?;
        Ok(())
    }

    fn newline<W>(&mut self, _writer: &mut W, _add_indent: i32) -> serde_hjson::Result<()>
    where
        W: std::io::Write,
    {
        Ok(())
    }

    fn start_value<W>(&mut self, _writer: &mut W) -> serde_hjson::Result<()>
    where
        W: std::io::Write,
    {
        Ok(())
    }
}
