use std::convert::TryInto;
use std::{error::Error, num::ParseIntError};

use warp::Filter;

#[derive(Debug)]
pub enum ToSegmentError {
    ValueError(ParseIntError),
    LengthError(usize),
}

pub enum Host<'a> {
    Localhost,
    Ipv4(&'a str),
}

impl Host<'_> {
    fn to_segmented_ip_addr(&self) -> Result<[u8; 4], ToSegmentError> {
        let ip_addr = match self {
            Host::Localhost => return Ok([127, 0, 0, 1]),
            Host::Ipv4(addr) => addr,
        };

        let segments = ip_addr
            .split(".")
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

pub async fn execute_server(host: Host<'_>, port: u16) -> Result<(), ToSegmentError> {
    let segmented_ip = host.to_segmented_ip_addr()?;

    #[cfg(debug_assertions)]
    println!(
        "🧙 Serving from http://{}:{}",
        segmented_ip
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("."),
        port
    );

    Ok(warp::serve(
        warp::get()
            .and(warp::path("graphiql"))
            .and(juniper_warp::graphiql_filter("/graphql", None)),
    )
    .run((segmented_ip, port))
    .await)
}
