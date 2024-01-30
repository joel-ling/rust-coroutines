#![feature(coroutines)]
#![feature(coroutine_trait)]
#![feature(trait_alias)]

use std::ops::Coroutine;
use std::pin::pin;
use std::pin::Pin;

trait VanillaCoroutine = Coroutine<Yield = (), Return = ()>;

struct CoroutineRuntime {
    queue: Vec<fn() -> Pin<Box<dyn VanillaCoroutine>>>,
}

impl CoroutineRuntime {
    fn new() -> Self {
        CoroutineRuntime { queue: Vec::new() }
    }

    fn add(&mut self, coroutine_maker: fn() -> Pin<Box<dyn VanillaCoroutine>>) {
        self.queue.push(coroutine_maker);
    }

    fn execute(&mut self) {
        loop {
            for make_coroutine in self.queue.iter() {
                let coroutine: Pin<Box<dyn VanillaCoroutine>> =
                    make_coroutine();

                let mut pinned: Pin<&mut Pin<Box<dyn VanillaCoroutine>>> =
                    pin!(coroutine);

                pinned.as_mut().resume(());
            }
        }
    }
}

fn hello() -> Pin<Box<dyn VanillaCoroutine>> {
    let coroutine = || {
        println!("Hello, world!");

        yield;
    };

    Box::pin(coroutine)
}

fn main() {
    let mut runtime = CoroutineRuntime::new();

    runtime.add(hello);

    runtime.execute();
}
