/*
    Handling concurrent programming safely and efficiently is another of Rust’s major goals.
    Concurrent programming, where different parts of a program execute independently, and parallel programming,
    where different parts of a program execute at the same time, are becoming increasingly important as more computers take advantage of their multiple processors.
    Historically, programming in these contexts has been difficult and error prone: Rust hopes to change that.

    Initially, the Rust team thought that ensuring memory safety and preventing concurrency problems were two separate challenges to be solved with different methods.
    Over time, the team discovered that the ownership and type systems are a powerful set of tools to help manage memory safety and concurrency problems!
    By leveraging ownership and type checking, many concurrency errors are compile-time errors in Rust rather than runtime errors.
    Therefore, rather than making you spend lots of time trying to reproduce the exact circumstances under which a runtime concurrency bug occurs,
    incorrect code will refuse to compile and present an error explaining the problem.

    As a result, you can fix your code while you’re working on it rather than potentially after it has been shipped to production.
    We’ve nicknamed this aspect of Rust fearless concurrency. Fearless concurrency allows you to write code that is free of subtle bugs
    and is easy to refactor without introducing new bugs.

    ...

    Many languages are dogmatic about the solutions they offer for handling concurrent problems.

    For example, Erlang has elegant functionality for message-passing concurrency but has only obscure ways to share state between threads.
    Supporting only a subset of possible solutions is a reasonable strategy for higher-level languages,
    because a higher-level language promises benefits from giving up some control to gain abstractions.
    However, lower-level languages are expected to provide the solution with the best performance in any given situation
    and have fewer abstractions over the hardware. Therefore, Rust offers a variety of tools for modeling problems in whatever way is appropriate
    for your situation and requirements.

    ...

    In most current operating systems, an executed program’s code is run in a process, and the operating system manages multiple processes at once.
    Within your program, you can also have independent parts that run simultaneously. The features that run these independent parts are called threads.

    Splitting the computation in your program into multiple threads can improve performance because the program does multiple tasks at the same time,
    but it also adds complexity. Because threads can run simultaneously, there’s no inherent guarantee about the order
    in which parts of your code on different threads will run. This can lead to problems, such as:

    1. Race conditions, where threads are accessing data or resources in an inconsistent order
    2. Deadlocks, where two threads are waiting for each other to finish using a resource the other thread has, preventing both threads from continuing
    3. Bugs that happen only in certain situations and are hard to reproduce and fix reliably.

    Programming languages implement threads in a few different ways. Many operating systems provide an API for creating new threads.
    This model where a language calls the operating system APIs to create threads is sometimes called 1:1,
    meaning one operating system thread per one language thread.

    Many programming languages provide their own special implementation of threads. Programming language-provided threads are known as green threads,
    and languages that use these green threads will execute them in the context of a different number of operating system threads.
    For this reason, the green-threaded model is called the M:N model: there are `M` green threads per `N` operating system threads,
    where `M` and `N` are not necessarily the same number.

    Each model has its own advantages and trade-offs, and the trade-off most important to Rust is runtime support.
    Runtime is a confusing term and can have different meanings in different contexts.

    In this context, by runtime we mean code that is included by the language in every binary. This code can be large or small depending on the language,
    but every non-assembly language will have some amount of runtime code. For that reason, colloquially when people say a language has “no runtime,”
    they often mean “small runtime.” Smaller runtimes have fewer features but have the advantage of resulting in smaller binaries,
    which make it easier to combine the language with other languages in more contexts.

    Although many languages are okay with increasing the runtime size in exchange for more features,
    Rust needs to have nearly no runtime and cannot compromise on being able to call into C to maintain performance.

    The green-threading M:N model requires a larger language runtime to manage threads.
    As such, the Rust standard library only provides an implementation of 1:1 threading. Because Rust is such a low-level language,
    there are crates that implement M:N threading if you would rather trade overhead for aspects such as more control over
    which threads run when and lower costs of context switching, for example.
*/
use std::thread;
use std::time::Duration;

fn main() {
    /*
        The code in Listing 16-1 not only stops the spawned thread prematurely most of the time due to the main thread ending,
        but also can’t guarantee that the spawned thread will get to run at all. The reason is that there is no guarantee on the order in which threads run!

        We can fix the problem of the spawned thread not getting to run, or not getting to run completely,
        by saving the return value of `thread::spawn` in a variable. The return type of `thread::spawn` is `JoinHandle`.
        A `JoinHandle` is an owned value that, when we call the join method on it, will wait for its thread to finish.
    */
    let hdl1 = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    hdl1.join().unwrap();

    let v = vec![1, 2, 3];

    let hdl2 = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    hdl2.join().unwrap();
}
