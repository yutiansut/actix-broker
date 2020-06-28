extern crate actix;
extern crate actix_broker;
extern crate actix_web;

use actix::prelude::*;
use actix_broker::{Broker, BrokerSubscribe, SystemBroker};
use actix_web::{web, App, Responder, HttpServer};

#[derive(Clone, Debug, Message)]
#[rtype(result = "()")]
struct Hello;

struct TestActor;

impl Actor for TestActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_async::<SystemBroker, Hello>(ctx);
    }
}

impl Handler<Hello> for TestActor {
    type Result = ();

    fn handle(&mut self, msg: Hello, _ctx: &mut Self::Context) {
        println!("TestActor: Received {:?}", msg);
    }
}

async fn index(info: web::Path<(String, u32)>) -> impl Responder {
    Broker::<SystemBroker>::issue_async(Hello);
    format!("Hello {}! id:{}", info.0, info.1)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    TestActor.start();
    HttpServer::new(|| App::new().service(
        web::resource("/{name}/{id}/index.html").to(index))
    )
        .bind("127.0.0.1:8080")?
        .run()
        .await
}