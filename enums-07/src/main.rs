// Enums allow you to define a type by enumerating its possible values.
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(value) => Some(value + 1),
    }
}

fn main() {
    let dime = Coin::Dime;

    // Rust has an extremely powerful control flow operator called match that allows you to compare a value against a series of patterns
    // and then execute code based on which pattern matches. Patterns can be made up of literal values, variable names, wildcards, and many other things.
    let value: i32 = match coin {
        Coin::Penny => {
            println!("one");
            1
        },
        Coin::Nickel => {
            println!("five");
            5
        },
        Coin::Dime => {
            println!("ten");
            10
        },
        Coin::Quarter => {
            println!("twenty-five");
            25
        }
    };

    /*
        The Option type is used in many places because it encodes the very common scenario in which a value could be something or it could be nothing.
        Expressing this concept in terms of the type system means the compiler can check whether you’ve handled all the cases you should be handling;
        this functionality can prevent bugs that are extremely common in other programming languages.

        ...

        Rust doesn’t have the null feature that many other languages have. Null is a value that means there is no value there.
        In languages with null, variables can always be in one of two states: null or not-null.

        ...

        The problem with null values is that if you try to use a null value as a not-null value, you’ll get an error of some kind.
        Because this null or not-null property is pervasive, it’s extremely easy to make this kind of error.

        ...

        The problem isn’t really with the concept but with the particular implementation. As such, Rust does not have nulls,
        but it does have an enum that can encode the concept of a value being present or absent.
        This enum is `Option<T>`, and it is defined by the standard library.


        enum Option<T> {
            Some(T),
            None,
        }

        If we use `None` rather than `Some`, we need to tell Rust what type of `Option<T>` we have,
        because the compiler can’t infer the type that the `Some` variant will hold by looking only at a `None` value.
    */
    let five = Some(5);
    let none: Option<i32> = None;

    let six = plus_one(five);

    /*
        Rust also has a pattern we can use when we don’t want to list all possible values. (`_`)
    */
    let number = 32;

    /*
        match number {
            1 => println!("one"),
            _ => (),
        }
    */
    if let Some(1) = number {
        println!("three");
    }
}