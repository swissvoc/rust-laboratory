/*
    Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions.
    You can create the closure in one place and then call the closure to evaluate it in a different context.
    Unlike functions, closures can capture values from the scope in which they’re defined.
*/

use std::thread;
use std::time::Duration;

/*
    ...

    Fortunately, another solution is available to us. We can create a struct that will hold the closure and the resulting value of calling the closure.
    The struct will execute the closure only if we need the resulting value, and it will cache the resulting value
    so the rest of our code doesn’t have to be responsible for saving and reusing the result. You may know this pattern as memoization or lazy evaluation.

    To make a struct that holds a closure, we need to specify the type of the closure, because a struct definition needs to know the types of each of its fields.
    Each closure instance has its own unique anonymous type: that is, even if two closures have the same signature, their types are still considered different.

    ...

    The `Fn` traits are provided by the standard library. All closures implement at least one of the traits: `Fn`, `FnMut`, or `FnOnce`.
    We’ll discuss the difference between these traits in the “Capturing the Environment with Closures” section; in this example, we can use the `Fn` trait.

    We add types to the Fn trait bound to represent the types of the parameters and return values the closures must have to match this trait bound.
    In this case, our closure has a parameter of type `u32` and returns a `u32`, so the trait bound we specify is `Fn(u32) -> u32`.

    ...

    When a closure captures a value from its environment, it uses memory to store the values for use in the closure body.
    This use of memory is overhead that we don’t want to pay in more common cases where we want to execute code that doesn’t capture its environment.
    Because functions are never allowed to capture their environment, defining and using functions will never incur this overhead.

    Closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter:
    taking ownership, borrowing mutably, and borrowing immutably. These are encoded in the three Fn traits as follows:

    1. `FnOnce` consumes the variables it captures from its enclosing scope, known as the closure’s environment.
    To consume the captured variables, the closure must take ownership of these variables and move them into the closure when it is defined.
    The `Once` part of the name represents the fact that the closure can’t take ownership of the same variables more than once, so it can be called only once.

    2. `FnMut` can change the environment because it mutably borrows values.

    3. `Fn` borrows values from the environment immutably.

    When you create a closure, Rust infers which trait to use based on how the closure uses the values from the environment.
    All closures implement `FnOnce` because they can all be called at least once. Closures that don’t move the captured variables also implement `FnMut`,
    and closures that don’t need mutable access to the captured variables also implement `Fn`.

    ...

    Caching values is a generally useful behavior that we might want to use in other parts of our code with different closures.
    However, there are two problems with the current implementation of Cacher that would make reusing it in different contexts difficult.

    The first problem is that a `Cacher` instance assumes it will always get the same value for the parameter `arg` to the `value` method.

    ...

    The problem is that the first time we called `c.value` with 1, the `Cacher` instance saved `Some(1)` in `self.value`. Thereafter,
    no matter what we pass in to the value method, it will always return 1.

    Try modifying `Cacher` to hold a hash map rather than a single value. The keys of the hash map will be the `arg` values that are passed in,
    and the values of the hash map will be the result of calling the closure on that key.
    Instead of looking at whether `self.value` directly has a `Some` or a `None` value, the value function will look up the `arg` in the hash map
    and return the value if it’s present. If it’s not present, the `Cacher` will call the closure and save the resulting value in the hash map
    associated with its `arg` value.

    The second problem with the current `Cacher` implementation is that it only accepts closures that take one parameter of type `u32` and return a `u32`.
    We might want to cache the results of closures that take a string slice and return `usize` values, for example.
    To fix this issue, try introducing more generic parameters to increase the flexibility of the `Cacher` functionality.
*/

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    /*
        Closures don’t require you to annotate the types of the parameters or the return value like `fn` functions do.
        Type annotations are required on functions because they’re part of an explicit interface exposed to your users.
        Defining this interface rigidly is important for ensuring that everyone agrees on what types of values a function uses and returns.
        But closures aren’t used in an exposed interface like this: they’re stored in variables and used without naming them
        and exposing them to users of our library.

        Closures are usually short and relevant only within a narrow context rather than in any arbitrary scenario. Within these limited contexts,
        the compiler is reliably able to infer the types of the parameters and the return type, similar to how it’s able to infer the types of most variables.

        Making programmers annotate the types in these small, anonymous functions would be annoying and largely redundant
        with the information the compiler already has available.

        As with variables, we can add type annotations if we want to increase explicitness and clarity
        at the cost of being more verbose than is strictly necessary.

        ...

        With type annotations added, the syntax of closures looks more similar to the syntax of functions.
        The following is a vertical comparison of the syntax for the definition of a function that adds 1 to its parameter and a closure that has the same behavior.

        ```
        fn  add_one_v1   (x: u32) -> u32 { x + 1 }
        let add_one_v2 = |x: u32| -> u32 { x + 1 };
        let add_one_v3 = |x|             { x + 1 };
        let add_one_v4 = |x|               x + 1  ;
        ```
    */
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    /*
        In the workout generator example, we only used closures as inline anonymous functions.
        However, closures have an additional capability that functions don’t have: they can capture their environment and access variables from the scope
        in which they’re defined.
    */
    let x = 4;
    let y = 4;

    let equal_to_x = |z| z == x;

    println!("{}", equal_to_x(y));
}