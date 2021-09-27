use std::convert::TryInto;
use std::num::ParseIntError;

use log::Level;
use warp::Filter;

use crate::logger;
use crate::service::generate_scheme;
use crate::service::state::provide_context;

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

pub async fn execute_server(host: Host<'_>, port: u16) -> Result<(), ToSegmentError> {
    let log = warp::log(logger::LOGGER_NAME);

    let scheme = generate_scheme();
    let scheme_lang = scheme.as_schema_language();
    let graphql_filter = juniper_warp::make_graphql_filter(
        scheme,
        warp::any().map(move || provide_context()).boxed(),
    );

    logger::info(format!(
        "🧙 Serving from http://{}:{}",
        host.to_string(),
        port
    ));

    warp::serve(
        warp::get()
            .and(warp::path("graphiql"))
            .and(juniper_warp::graphiql_filter("/graphql", None))
            .or(warp::path("graphql").and(graphql_filter))
            .or(warp::path("scheme").map(move || scheme_lang.clone()))
            .with(log),
    )
    .run((host.to_segmented_ip_addr()?, port))
    .await;

    Ok(())
}
