use test_case::test_case;
use actix_broker::act::{ActixBroker, ActorOne, MessageOne};
use actix::Actor;
use parking_lot::Mutex;
use std::sync::Arc;
use actix_broker::{Broker, SystemBroker};



#[actix_rt::main]
async fn main() {
    let broker = ActixBroker::new();

    ActorOne::start(ActorOne::new(Arc::new(Mutex::new(broker))));

    async move {
        Broker::<SystemBroker>::issue_async(MessageOne {});
        Broker::<SystemBroker>::issue_async(MessageOne {});
        Broker::<SystemBroker>::issue_async(MessageOne {});
        Broker::<SystemBroker>::issue_async(MessageOne {});
        Broker::<SystemBroker>::issue_async(MessageOne {});
        Broker::<SystemBroker>::issue_async(MessageOne {});
        Broker::<SystemBroker>::issue_async(MessageOne {});
        Broker::<SystemBroker>::issue_async(MessageOne {});
        Broker::<SystemBroker>::issue_async(MessageOne {});
        Broker::<SystemBroker>::issue_async(MessageOne {});
        Broker::<SystemBroker>::issue_async(MessageOne {});
    }.await;
}