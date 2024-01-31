#![feature(coroutines)]
#![feature(coroutine_trait)]
#![feature(trait_alias)]

use std::ops::Coroutine;
use std::pin::pin;
use std::pin::Pin;

trait VanillaCoroutine = Coroutine<Yield = (), Return = ()>;

fn make_hello() -> impl VanillaCoroutine {
    || loop {
        println!("Hello, world!");

        yield;
    }
}

fn make_increment() -> impl VanillaCoroutine {
    || {
        let mut x: u8 = 0;

        loop {
            println!("{x}");

            match x {
                255 => x = 0,
                _ => x = x + 1,
            }

            yield;
        }
    }
}

fn main() {
    let mut hello: Pin<&mut dyn VanillaCoroutine> = pin!(make_hello());

    let mut increment: Pin<&mut dyn VanillaCoroutine> = pin!(make_increment());

    // XXX: `hello`, `increment` are local due to a limitation in `pin!`
    // XXX: see https://doc.rust-lang.org/std/pin/macro.pin.html#remarks

    loop {
        hello.as_mut().resume(());
        increment.as_mut().resume(());
    }
}
