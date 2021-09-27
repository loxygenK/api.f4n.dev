use crate::repository::mock::AssetRepository;
use crate::router::Query;
use crate::server::exec::Server;
use crate::service::Service;

pub enum Mode {
    Production,
    Development,
}

pub fn setup(mode: Mode) -> Server {
    match mode {
        Mode::Development => setup_development(),
        Mode::Production => unimplemented!(),
    }
}

fn setup_development() -> Server {
    let repository = Box::new(AssetRepository);
    let service = Service::new(repository);
    let query = Query::new(service);

    Server::new(query)
}
