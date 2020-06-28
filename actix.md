## Actix



### ActorResponse	A helper type for representing different types of message responses.
### Addr	The address of an actor.
### Arbiter	Arbiters provide an asynchronous execution environment for actors, functions and futures. When an Arbiter is created, it spawns a new OS thread, and hosts an event loop. Some Arbiter functions execute on the current thread.
### AtomicResponse	A specialized actor future holder for atomic asynchronous message handling.
### Context	An actor execution context.
### MessageResult	A helper type that implements the MessageResponse trait.
### Recipient	The Recipient type allows to send one specific message to an actor.
### Response	Helper type for representing different type of message responses
### SpawnHandle	A handle to a spawned future.
### Supervisor	Actor supervisor
### System	System is a runtime manager.
### SystemRunner	Helper object that runs System's event loop
### WeakAddr	A weakly referenced counterpart to Addr<A>.