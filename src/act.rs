use actix::prelude::*;
use crate::broker::{Broker, SystemBroker};
use crate::subscribe::BrokerSubscribe;
use crate::msgs::BrokerMsg;
use anymap::AnyMap;
use std::sync::Arc;
use parking_lot::Mutex;

pub trait ReplaceableBroker: Unpin + 'static {
    /// Send message asynchronously
    fn issue_async<M: Message + BrokerMsg + 'static>(&mut self, message: M);
}

///
/// Spying broker for performing unit tests
///
pub struct SpyingBroker {
    issued_async: AnyMap
}

impl SpyingBroker {
    pub fn new() -> Self {
        Self {
            issued_async: AnyMap::new()
        }
    }

    pub fn messages<M: 'static>(&self) -> Option<&Vec<M>> {
        self.issued_async.get::<Vec<M>>()
    }
}

impl ReplaceableBroker for SpyingBroker {
    fn issue_async<M: 'static>(&mut self, message: M) {
        match self.issued_async.get_mut::<Vec<M>>() {
            Some(vect) => {
                vect.push(message);
            },
            None => {
                self.issued_async.insert(vec![message]);
            }
        };
    }
}

///
/// A broker that uses actix-broker to issue messages
///
pub struct ActixBroker;

impl ActixBroker {
    pub fn new() -> Self {
        Self {}
    }
}

impl ReplaceableBroker for ActixBroker {
    fn issue_async<M: Message + BrokerMsg + 'static>(&mut self, message: M) {
        Broker::<SystemBroker>::issue_async(message);
    }
}

// Note: The message must implement 'Clone'
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct MessageOne;

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct MessageTwo;

#[derive(Clone, Message)]
#[rtype(result = "()")]
struct MessageThree;

pub struct ActorOne<B> {
    broker: Arc<Mutex<B>>
}

impl<B: ReplaceableBroker> ActorOne<B> {
    pub fn new(broker: Arc<Mutex<B>>) -> Self {
        Self {
            broker
        }
    }
}

impl<B: ReplaceableBroker> Actor for ActorOne<B> {
    // Note: The actor context must be Asynchronous,
    // i.e. it cannot be 'SyncContext'
    type Context = Context<Self>;

    fn started(&mut self,ctx: &mut Self::Context) {
        // Asynchronously subscribe to a message on the system (global) broker
        self.subscribe_system_async::<MessageOne>(ctx);
        self.subscribe_system_async::<MessageTwo>(ctx);
    }
}

// To subscribe to a messsage, the actor must handle it
impl<B: ReplaceableBroker> Handler<MessageOne> for ActorOne<B> {
    type Result = ();

    fn handle(&mut self, _msg: MessageOne, _ctx: &mut Self::Context) {
        // An actor does not have to handle a message to just issue it
        println!("MessageOne received, issuing MessageTwo");
        self.broker.lock().issue_async(MessageTwo {});
    }
}

impl<B: ReplaceableBroker> Handler<MessageTwo> for ActorOne<B> {
    type Result = ();

    fn handle(&mut self, _msg: MessageTwo, _ctx: &mut Self::Context) {
        // An actor does not have to handle a message to just issue it
        println!("MessageTwo received!");
    }
}


#[cfg(test)]
mod tests {
    use crate::act::{SpyingBroker, ActorOne, MessageOne, MessageTwo};
    use actix::Actor;
    use std::sync::Arc;
    use parking_lot::Mutex;
    use actix_rt::System;
    use test_case::test_case;

    #[test_case(1; "Should send 1 message")]
    #[test_case(8; "Should send 8 messages")]
    fn test_sending(sent_messages: usize) {
        let spy = Arc::new(Mutex::new(SpyingBroker::new()));
        let spy_clone = spy.clone();

        System::run(move || {
            let addr = ActorOne::start(ActorOne::new(spy_clone));
            for _ in 0..sent_messages {
                addr.do_send(MessageOne{});
            }
            System::current().stop();
        })
            .unwrap();

        let locked_spy = spy.lock();

        assert_eq!(sent_messages, locked_spy.messages::<MessageTwo>().expect("No MessageTwo messages").len())
    }
}