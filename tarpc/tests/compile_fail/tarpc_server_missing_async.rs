#![allow(incomplete_features)]
#![feature(generic_associated_types)]

#[tarpc::service]
trait World {
    async fn hello(name: String) -> String;
}

struct HelloServer;

#[tarpc::server]
impl World for HelloServer {
    fn hello(name: String) ->  String {
        format!("Hello, {}!", name)
    }
}

fn main() {}
