use crate::repository::Repository;
use crate::repository::loxygenk_d::LoxygenKDRepository;
use crate::repository::mock::AssetRepository;
use crate::router::Query;
use crate::server::exec::Server;
use crate::service::Service;

pub enum Mode {
    Production,
    Development,
}

pub fn setup(mode: Mode) -> Server {
    let repository: Box<dyn Repository> = match mode {
        Mode::Development => Box::new(AssetRepository),
        Mode::Production => Box::new(LoxygenKDRepository)
    };
    let service = Service::new(repository);
    let query = Query::new(service);

    Server::new(query)
}
