## actix_rt

Arbiter  提供匿名异步执行器// 开启一个新的线程 // 开启一个新的eventloop
    .current() 
    .is_running()
    .stop
    .new()
    .spawn<F>
    .spawn_fn<F, R>
    .send<F>
    .exec_fn<F>
    .exec<F, R>(&self, f: F) -> impl Future<Output = Result<R, Canceled>>
    .set_item<T: 'static>(item: T)
    .contains_item<T: 'static>() -> bool
    .get_item<T: 'static, F, R>(f: F) -> R
    .get_mut_item<T: 'static, F, R>(f: F) -> R
    .join(&mut self) -> Result<()>
    .local_join() -> impl Future<Output = ()>

Builder
    .name<T: Into<String>>(self, name: T) -> Self
    .stop_on_panic(self, stop_on_panic: bool) -> Self
    .build(self) -> SystemRunner
    .run<F>(self, f: F) -> Result<()>

Runtime  一个单线程的线程池, 用于提供一个actor的运行环境
    .new() -> Result<Runtime>
    .spawn<F>(&self, future: F) -> &Self
    .block_on<F>(&mut self, f: F) -> F::Output

System  用于管理runtime的
    .builder() -> Builder
    .new<T: Into<String>>(name: T) -> SystemRunner
    .run_in_tokio<T: Into<String>>(
                                    name: T,
                                    local: &LocalSet
                                ) -> impl Future<Output = Result<()>>
    .current() -> System
    .is_set() -> bool
    .with_current<F, R>(f: F) -> R
    .id(&self) -> usize
    .stop(&self)
    .stop_with_code(&self, code: i32)
    .stop_on_panic(&self) -> bool
    .arbiter(&self) -> &Arbiter
    .run<F>(f: F) -> Result<()>
SystemRunner
    .run(self) -> Result<()>
    .block_on<F, O>(&mut self, fut: F) -> O

