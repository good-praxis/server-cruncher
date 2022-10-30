use hcloud::models::server::Server;

pub enum Data {
    Servers(Vec<Server>),
}
