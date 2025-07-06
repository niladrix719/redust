use std::io::{self, BufReader, Read};

pub struct Resp<R: Read> {
    reader: BufReader<R>,
}

impl<R: Read> Resp<R> {
    pub fn new(rd: R) -> Self {
        Resp {
            reader: BufReader::new(rd),
        }
    }

    pub fn parse_texts(&mut self) -> Result<Vec<String>, std::io::Error> {
        let mut buf = [0u8; 1];
        self.reader.read_exact(&mut buf)?;
        let sym = buf[0];

        match sym {
            b'*' => self.parse_array(),
            _ => Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid response type",
            )),
        }
    }

    pub fn read_line(&mut self) -> Result<String, std::io::Error> {
        let mut s = String::new();
        loop {
            let mut buf = [0u8; 1];
            self.reader.read_exact(&mut buf)?;
            let ch = buf[0] as char;

            if ch == '\r' {
                let mut next = [0u8; 1];
                self.reader.read_exact(&mut next)?;
                if next[0] == b'\n' {
                    break;
                } else {
                    s.push(ch);
                    s.push(next[0] as char);
                    continue;
                }
            } else {
                s.push(ch);
            }
        }
        Ok(s)
    }

    pub fn parse_array(&mut self) -> Result<Vec<String>, std::io::Error> {
        let len_str = self.read_line()?;
        let len: usize = len_str.parse().unwrap_or(0);
        let mut elements = Vec::with_capacity(len);

        for _ in 0..len {
            let nested = self.parse_texts()?;
            elements.extend(nested);
        }

        Ok(elements)
    }
}
