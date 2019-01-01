/*
    One increasingly popular approach to ensuring safe concurrency is message passing, where threads or actors communicate by sending each other messages
    containing data. Here’s the idea in a slogan from the Go language documentation: “Do not communicate by sharing memory; instead, share memory by communicating.”

    One major tool Rust has for accomplishing message-sending concurrency is the channel, a programming concept
    that Rust’s standard library provides an implementation of. You can imagine a channel in programming as being like a channel of water,
    such as a stream or a river. If you put something like a rubber duck or boat into a stream, it will travel downstream to the end of the waterway.

    A channel in programming has two halves: a transmitter and a receiver. The transmitter half is the upstream location where you put rubber ducks into the river,
    and the receiver half is where the rubber duck ends up downstream. One part of your code calls methods on the transmitter with the data you want to send,
    and another part checks the receiving end for arriving messages. A channel is said to be closed if either the transmitter or receiver half is dropped.

    Here, we’ll work up to a program that has one thread to generate values and send them down a channel,
    and another thread that will receive the values and print them out. We’ll be sending simple values between threads using a channel to illustrate the feature.
    Once you’re familiar with the technique, you could use channels to implement a chat system or a system where many threads perform parts of a calculation
    and send the parts to one thread that aggregates the results.

    ...

    We create a new channel using the mpsc::channel function; mpsc stands for multiple producer, single consumer.
    In short, the way Rust’s standard library implements channels means a channel can have multiple sending ends that produce values
    but only one receiving end that consumes those values. Imagine multiple streams flowing together into one big river: everything sent down any of the streams
    will end up in one river at the end. We’ll start with a single producer for now, but we’ll add multiple producers when we get this example working.

    The `mpsc::channel` function returns a tuple, the first element of which is the sending end and the second element is the receiving end.
    The abbreviations `tx` and `rx` are traditionally used in many fields for transmitter and receiver respectively, so we name our variables as such to indicate each end.
    We’re using a `let` statement with a pattern that destructures the tuples; we’ll discuss the use of patterns in `let` statements and destructuring in Chapter 18.
    Using a `let` statement this way is a convenient approach to extract the pieces of the tuple returned by `mpsc::channel`.

    ...

    The receiving end of a channel has two useful methods: `recv` and `try_recv`. We’re using `recv`, short for receive,
    which will block the main thread’s execution and wait until a value is sent down the channel. Once a value is sent, `recv` will return it in a `Result<T, E>`.
    When the sending end of the channel closes, `recv` will return an error to signal that no more values will be coming.

    The `try_recv` method doesn’t block, but will instead return a `Result<T, E>` immediately: an `Ok` value holding a message if one is available
    and an `Err` value if there aren’t any messages this time. Using `try_recv` is useful if this thread has other work to do while waiting for messages:
    we could write a loop that calls `try_recv` every so often, handles a message if one is available, and otherwise does other work for a little while
    until checking again.

    We’ve used `recv` in this example for simplicity; we don’t have any other work for the main thread to do other than wait for messages,
    so blocking the main thread is appropriate.
*/

use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("one"),
            String::from("two"),
            String::from("three"),
            String::from("four"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("five"),
            String::from("six"),
            String::from("seven"),
            String::from("eight"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}