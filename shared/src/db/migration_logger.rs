use log::Level;
use std::io::{self, ErrorKind, Write};
use std::str;

#[derive(Debug)]
pub struct MigrationLogger {
  level: Level,
  buffer: String,
}

impl MigrationLogger {
  pub fn new(level: Level) -> Self {
    Self {
      level,
      buffer: String::new(),
    }
  }

  fn write_lines(&mut self, write_remainder: bool) {
    let (lines, remainder) = match self.buffer.rsplit_once('\n') {
      None => return,
      Some(split) => split,
    };

    // Log all lines currently stored in the buffer
    for line in lines.lines() {
      // Only log a line if it is NOT blank (all whitespace)
      let trimmed = line.trim();
      if trimmed.len() > 0 {
        log::log!(self.level, "{}", trimmed);
      }
    }

    // A call to "flush" will force-log the remainder, even if not a complete line
    if write_remainder {
      let trimmed_remainder = remainder.trim();
      if trimmed_remainder.len() > 0 {
        log::log!(self.level, "{}", trimmed_remainder);
      }
    }

    self.buffer = remainder.to_string();
  }
}

impl Default for MigrationLogger {
  fn default() -> Self {
    Self::new(Level::Info)
  }
}

impl Write for MigrationLogger {
  fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    // Append to the internal string buffer
    let text = str::from_utf8(buf).map_err(|e| io::Error::new(ErrorKind::InvalidData, e))?;
    self.buffer += text;

    // Then, write any complete lines to the log file
    self.write_lines(false);

    Ok(buf.len())
  }

  fn flush(&mut self) -> io::Result<()> {
    // Write any complete lines, and the remaining text, to the log file
    self.write_lines(true);

    Ok(())
  }
}
