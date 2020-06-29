extern crate actix;
extern crate actix_broker;
use actix::prelude::*;
use actix_broker::{BrokerIssue, BrokerSubscribe, SystemBroker};
use std::{thread, time};
struct MOMStrategy1;
struct SubStrategy1;
struct SubStrategy2;
type BrokerType = SystemBroker;

impl Actor for MOMStrategy1 {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_sync::<BrokerType, OrderComplete>(ctx);
        self.issue_async::<BrokerType, _>(OrderExecutor("send order x".to_string()));
    }
}

impl Handler<OrderComplete> for MOMStrategy1 {
    type Result = ();

    fn handle(&mut self, msg: OrderComplete, _ctx: &mut Self::Context) {
        println!("Strategy1 Received: {:?}", msg);


        //self.issue_async::<BrokerType, _>(OrderExecutor("send order y".to_string()));
    }
}

impl Actor for SubStrategy1 {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_sync::<BrokerType, OrderExecutor>(ctx);
    }
}

impl Handler<OrderExecutor> for SubStrategy1 {
    type Result = ();

    fn handle(&mut self, msg: OrderExecutor, _ctx: &mut Self::Context) {
        println!("Strategy2 Received: {:?}", msg);
        self.issue_async::<BrokerType, _>(OrderComplete(0));
    }
}

impl Actor for SubStrategy2 {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_sync::<BrokerType, OrderExecutor>(ctx);
    }
}

impl Handler<OrderExecutor> for SubStrategy2 {
    type Result = ();

    fn handle(&mut self, msg: OrderExecutor, _ctx: &mut Self::Context) {
        println!("Strategy3 Received: {:?}", msg);
        self.issue_async::<BrokerType, _>(OrderComplete(1));
    }
}

#[derive(Clone, Debug, Message)]
#[rtype(result = "()")]
struct OrderExecutor(String);

#[derive(Clone, Debug, Message)]
#[rtype(result = "()")]
struct OrderComplete(u8);

fn main() {
    let _ = System::run(|| {
        SubStrategy1.start();
        SubStrategy2.start();
        MOMStrategy1.start();
    });
}
