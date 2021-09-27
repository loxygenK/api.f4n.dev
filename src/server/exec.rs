use warp::Filter;

use crate::router::state::provide_context;
use crate::{logger, router::Query};

use super::host::{Host, ToSegmentError};

pub struct Server {
    query: Query,
}

impl Server {
    pub fn new(query: Query) -> Self {
        Self { query }
    }

    pub async fn execute_server(self, host: Host<'_>, port: u16) -> Result<(), ToSegmentError> {
        let log = warp::log(logger::LOGGER_NAME);

        let scheme = self.query.generate_scheme();
        let scheme_lang = scheme.as_schema_language();
        let graphql_filter = juniper_warp::make_graphql_filter(
            scheme,
            warp::any().map(provide_context).boxed(),
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
}
