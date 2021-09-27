use std::convert::TryInto;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum ToSegmentError {
    ValueError(ParseIntError),
    LengthError(usize),
}

#[allow(unused)]
pub enum Host<'a> {
    Localhost,
    Ipv4(&'a str),
}

impl Host<'_> {
    pub fn to_segmented_ip_addr(&self) -> Result<[u8; 4], ToSegmentError> {
        let ip_addr = match self {
            Host::Localhost => return Ok([127, 0, 0, 1]),
            Host::Ipv4(addr) => addr,
        };

        let segments = ip_addr
            .split('.')
            .map(|s| s.parse::<u8>())
            .collect::<Result<Vec<u8>, _>>();

        if let Err(e) = segments {
            return Err(ToSegmentError::ValueError(e));
        }
        let segments = segments.unwrap();

        if segments.len() != 4 {
            return Err(ToSegmentError::LengthError(segments.len()));
        }

        Ok(segments.as_slice().try_into().unwrap())
    }
}

impl ToString for Host<'_> {
    fn to_string(&self) -> String {
        self.to_segmented_ip_addr()
            .expect("Invalid IP Address provided")
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(".")
    }
}
