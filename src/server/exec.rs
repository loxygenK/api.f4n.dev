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
        let scheme = self.query.generate_scheme();
        let scheme_lang = scheme.as_schema_language();

        let graphql = warp::path("graphql")
            .and(warp::post())
            .and(juniper_warp::make_graphql_filter(scheme, warp::any().map(provide_context).boxed()));

        let graphql_scheme = warp::path("scheme")
            .and(warp::get())
            .map(move || scheme_lang.clone());

        let graphiql = warp::path("graphiql")
            .and(warp::get())
            .and(juniper_warp::graphiql_filter("/graphql", None));

        let endpoint = graphql.or(graphql_scheme).or(graphiql);

        let cors = warp::cors()
            .allow_any_origin()
            .allow_header("Content-Type")
            .allow_methods(["GET", "POST"]);

        let server = warp::serve(
            endpoint
                .with(cors)
                .with(warp::log(logger::LOGGER_NAME))
        );

        logger::info(format!(
            "🧙 Serving from http://{}:{}",
            host.to_string(),
            port
        ));

        server.run((host.to_segmented_ip_addr()?, port)).await;

        Ok(())
    }
}
