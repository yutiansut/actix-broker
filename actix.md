## Actix



### ActorResponse	A helper type for representing different types of message responses.
    .reply(val: Result<I, E>) -> Self
    .async<T>(fut: T) -> Self
### Addr	The address of an actor.
    .new(tx: AddressSender<A>) -> Addr<A>
    .connected(&self) -> bool
    .do_send<M>(&self, msg: M)
    .try_send<M>(&self, msg: M) -> Result<(), SendError<M>>
    .send<M>(&self, msg: M) -> Request<A, M>
    .recipient<M: 'static>(self) -> Recipient<M>
### Arbiter	Arbiters provide an asynchronous execution environment for actors, functions and futures. When an Arbiter is created, it spawns a new OS thread, and hosts an event loop. Some Arbiter functions execute on the current thread.
    .current() -> Arbiter
    .is_running() -> bool
    .stop(&self)
    .new() -> Arbiter
    .spawn<F>(future: F)
    .spawn_fn<F, R>(f: F)
    .send<F>(&self, future: F)
    .exec_fn<F>(&self, f: F)
    .exec<F, R>(&self, f: F) -> impl Future<Output = Result<R, Canceled>>
    .set_item<T: 'static>(item: T)
    .contains_item<T: 'static>() -> bool
    .get_item<T: 'static, F, R>(f: F) -> R
    .get_mut_item<T: 'static, F, R>(f: F) -> R
    .join(&mut self) -> Result<()>
    .local_join() -> impl Future<Output = ()>

### AtomicResponse	A specialized actor future holder for atomic asynchronous message handling.
### Context	An actor execution context.
    .with_receiver(rx: AddressReceiver<A>) -> Self
    .run(self, act: A) -> Addr<A>
    .into_future(self, act: A) -> ContextFut<A, Self>
    .handle(&self) -> SpawnHandle
    .set_mailbox_capacity(&mut self, cap: usize)
    .connected(&self) -> bool
### MessageResult	A helper type that implements the MessageResponse trait.
### Recipient	The Recipient type allows to send one specific message to an actor.
    .do_send(&self, msg: M) -> Result<(), SendError<M>>
    .try_send(&self, msg: M) -> Result<(), SendError<M>>
    .send(&self, msg: M) -> RecipientRequest<M>
    .connected(&self) -> bool
### Response	Helper type for representing different type of message responses
    .fut<T>(fut: T) -> Self
    .reply(val: Result<I, E>) -> Self
### SpawnHandle	A handle to a spawned future.
### Supervisor	Actor supervisor
    .start<F>(f: F) -> Addr<A>
    .start_in_arbiter<F>(sys: &Arbiter, f: F) -> Addr<A>
### System	System is a runtime manager.
### SystemRunner	Helper object that runs System's event loop
### WeakAddr	A weakly referenced counterpart to Addr<A>.