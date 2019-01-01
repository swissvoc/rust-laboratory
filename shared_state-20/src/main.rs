/*
    Message passing is a fine way of handling concurrency, but it’s not the only one.
    Consider this part of the slogan from the Go language documentation again: “communicate by sharing memory.”

    What would communicating by sharing memory look like? In addition, why would message-passing enthusiasts not use it and do the opposite instead?

    In a way, channels in any programming language are similar to single ownership, because once you transfer a value down a channel,
    you should no longer use that value. Shared memory concurrency is like multiple ownership: multiple threads can access the same memory location at the same time.
    As you saw in Chapter 15, where smart pointers made multiple ownership possible, multiple ownership can add complexity
    because these different owners need managing. Rust’s type system and ownership rules greatly assist in getting this management correct.
    For an example, let’s look at mutexes, one of the more common concurrency primitives for shared memory.

    Mutex is an abbreviation for mutual exclusion, as in, a mutex allows only one thread to access some data at any given time.
    To access the data in a mutex, a thread must first signal that it wants access by asking to acquire the mutex’s lock.
    The lock is a data structure that is part of the mutex that keeps track of who currently has exclusive access to the data.
    Therefore, the mutex is described as guarding the data it holds via the locking system.

    Mutexes have a reputation for being difficult to use because you have to remember two rules:
    1. You must attempt to acquire the lock before using the data.
    2. When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.

    For a real-world metaphor for a mutex, imagine a panel discussion at a conference with only one microphone. Before a panelist can speak,
    they have to ask or signal that they want to use the microphone. When they get the microphone, they can talk for as long as they want to
    and then hand the microphone to the next panelist who requests to speak. If a panelist forgets to hand the microphone off when they’re finished with it,
    no one else is able to speak. If management of the shared microphone goes wrong, the panel won’t work as planned!

    Management of mutexes can be incredibly tricky to get right, which is why so many people are enthusiastic about channels.
    However, thanks to Rust’s type system and ownership rules, you can’t get locking and unlocking wrong.

    ...

    As you might suspect, `Mutex<T>` is a smart pointer. More accurately, the call to lock returns a smart pointer called `MutexGuard`.
    This smart pointer implements `Deref` to point at our inner data; the smart pointer also has a `Drop` implementation that releases the lock automatically
    when a `MutexGuard` goes out of scope, which happens at the end of the inner scope in Listing 16-12.

    As a result, we don’t risk forgetting to release the lock and blocking the mutex from being used by other threads because the lock release happens automatically.

    ...

    `Arc<T>` is a type like `Rc<T>` that is safe to use in concurrent situations. The a stands for atomic, meaning it’s an atomically reference counted type.
    Atomics are an additional kind of concurrency primitive that we won’t cover in detail here: see the standard library documentation for `std::sync::atomic`
    for more details. At this point, you just need to know that atomics work like primitive types but are safe to share across threads.

    You might then wonder why all primitive types aren’t atomic and why standard library types aren’t implemented to use `Arc<T>` by default.
    The reason is that thread safety comes with a performance penalty that you only want to pay when you really need to.
    If you’re just performing operations on values within a single thread, your code can run faster if it doesn’t have to enforce the guarantees atomics provide.

    ...

    You might have noticed that counter is immutable but we could get a mutable reference to the value inside it; this means `Mutex<T>` provides interior mutability,
    as the `Cell` family does. In the same way we used `RefCell<T>` in Chapter 15 to allow us to mutate contents inside an `Rc<T>`,
    we use `Mutex<T>` to mutate contents inside an `Arc<T>`.

    Another detail to note is that Rust can’t protect you from all kinds of logic errors when you use `Mutex<T>`.
    Recall in Chapter 15 that using `Rc<T>` came with the risk of creating reference cycles, where two `Rc<T>` values refer to each other, causing memory leaks.
    Similarly, `Mutex<T>` comes with the risk of creating deadlocks. These occur when an operation needs to lock two resources
    and two threads have each acquired one of the locks, causing them to wait for each other forever.
    If you’re interested in deadlocks, try creating a Rust program that has a deadlock; then research deadlock mitigation strategies for mutexes
    in any language and have a go at implementing them in Rust. The standard library API documentation for `Mutex<T>` and `MutexGuard` offers useful information.
*/
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}