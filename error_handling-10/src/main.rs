use std::fs::File;
use std::io::ErrorKind;

/*
    Rust groups errors into two major categories: recoverable and unrecoverable errors. For a recoverable error, such as a file not found error,
    it’s reasonable to report the problem to the user and retry the operation. Unrecoverable errors are always symptoms of bugs,
    like trying to access a location beyond the end of an array.

    Most languages don’t distinguish between these two kinds of errors and handle both in the same way, using mechanisms such as exceptions.
    Rust doesn’t have exceptions. Instead, it has the type `Result<T, E>` for recoverable errors and the `panic!` macro that stops execution
    when the program encounters an unrecoverable error. This chapter covers calling `panic!` first and then talks about returning `Result<T, E>` values.
    Additionally, we’ll explore considerations when deciding whether to try to recover from an error or to stop execution.
*/
fn main() {
    //  A backtrace is a list of all the functions that have been called to get to this point. Backtraces in Rust work as they do in other languages:
    // the key to reading the backtrace is to start from the top and read until you see files you wrote. That’s the spot where the problem originated.
    // panic!("unrecoverable error");

    /*
        Most errors aren’t serious enough to require the program to stop entirely. Sometimes, when a function fails,
        it’s for a reason that you can easily interpret and respond to. For example, if you try to open a file and that operation fails
        because the file doesn’t exist, you might want to create the file instead of terminating the process.

        ...
    */
    /*
        let f = File::open("hello.txt");

        let f = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
                },
                other_error => panic!("There was a problem opening the file: {:?}", other_error),
            },
        };
    */
    let _f = File::open("some_text.txt").map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("some_text.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error);
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });

    /*
        Using match works well enough, but it can be a bit verbose and doesn’t always communicate intent well.
        The `Result<T, E>` type has many helper methods defined on it to do various tasks. One of those methods, called `unwrap`,
        is a shortcut method that is implemented. If the `Result` value is the `Ok` variant, `unwrap` will return the value inside the `Ok`.
        If the `Result` is the `Err` variant, unwrap will call the `panic!` macro for us.

        ```
        let f = File::open("hello.txt").unwrap();
        ```

        ...

        Another method, `expect`, which is similar to `unwrap`, lets us also choose the `panic!` error message.
        Using `expect` instead of `unwrap` and providing good error messages can convey your intent and make tracking down the source of a panic easier.

        ```
        let f = File::open("hello.txt").expect("Failed to open hello.txt");
        ```
    */
}
