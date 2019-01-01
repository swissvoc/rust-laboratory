/*
    Rather than ensuring that a type has the behavior we want, lifetimes ensure that references are valid as long as we need them to be.
    The main aim of lifetimes is to prevent dangling references, which cause a program to reference data other than the data it’s intended to reference.

    ...

    Most of the time, lifetimes are implicit and inferred, just like most of the time, types are inferred. We must annotate types when multiple types are possible.
    In a similar way, we must annotate lifetimes when the lifetimes of references could be related in a few different ways.
    Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.

    The concept of lifetimes is somewhat different from tools in other programming languages, arguably making lifetimes Rust’s most distinctive feature.

    ...

    The Rust compiler has a borrow checker that compares scopes to determine whether all borrows are valid.

    ...

    ```
    {
        let r;                // ---------+-- 'a
                              //          |
        {                     //          |
            let x = 5;        // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
                              //          |
        println!("r: {}", r); //          |
    }                         // ---------+
    ```

    Here, we’ve annotated the lifetime of `r` with `'a` and the lifetime of `x` with `'b`. As you can see,
    the inner 'b block is much smaller than the outer `'a` lifetime block. At compile time, Rust compares the size of the two lifetimes
    and sees that `r` has a lifetime of `'a` but that it refers to memory with a lifetime of `'b`. The program is rejected because `'b` is shorter than `'a`:
    the subject of the reference does not live as long as the reference.

    ...

    ```
    {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+
    ```

    Here, `x` has the lifetime `'b`, which in this case is larger than `'a`. This means `r` can reference `x`
    because Rust knows that the reference in `r` will always be valid while `x` is valid.
*/

/*
    Lifetime annotations don’t change how long any of the references live.
    Just as functions can accept any type when the signature specifies a generic type parameter, functions can accept references with any lifetime
    by specifying a generic lifetime parameter. Lifetime annotations describe the relationships of the lifetimes of multiple references to each other
    without affecting the lifetimes.

    Lifetime annotations have a slightly unusual syntax: the names of lifetime parameters must start with an apostrophe (`'`)
    and are usually all lowercase and very short, like generic types. Most people use the name `'a`. We place lifetime parameter annotations
    after the `&` of a reference, using a space to separate the annotation from the reference’s type.

    ...

    The function signature now tells Rust that for some lifetime `'a`, the function takes two parameters,
    both of which are string slices that live at least as long as lifetime `'a`.
    The function signature also tells Rust that the string slice returned from the function will live at least as long as lifetime `'a`.
    These constraints are what we want Rust to enforce. Remember, when we specify the lifetime parameters in this function signature,
    we’re not changing the lifetimes of any values passed in or returned. Rather, we’re specifying that the borrow checker should reject any values
    that don’t adhere to these constraints. Note that the `longest` function doesn’t need to know exactly how long `x` and `y` will live,
    only that some scope can be substituted for `'a` that will satisfy this signature.

    When annotating lifetimes in functions, the annotations go in the function signature, not in the function body.
    Rust can analyze the code within the function without any help. However, when a function has references to or from code outside that function,
    it becomes almost impossible for Rust to figure out the lifetimes of the parameters or return values on its own.
    The lifetimes might be different each time the function is called. This is why we need to annotate the lifetimes manually.

    When we pass concrete references to `longest`, the concrete lifetime that is substituted for `'a` is the part of the scope of `x`
    that overlaps with the scope of `y`.

    In other words, the generic lifetime 'a will get the concrete lifetime that is equal to the smaller of the lifetimes of `x` and `y`.
    Because we’ve annotated the returned reference with the same lifetime parameter `'a`,
    the returned reference will also be valid for the length of the smaller of the lifetimes of `x` and `y`.

    ...


    The way in which you need to specify lifetime parameters depends on what your function is doing. For example,
    if we changed the implementation of the longest function to always return the first parameter rather than the longest string slice,
    we wouldn’t need to specify a lifetime on the y parameter.

    ...

    ```
    fn longest<'a>(x: &'a str, y: &str) -> &'a str {
        x
    }
    ```

    When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters.
    If the reference returned does not refer to one of the parameters, it must refer to a value created within this function,
    which would be a dangling reference because the value will go out of scope at the end of the function.

    ```
    fn longest<'a>(x: &str, y: &str) -> &'a str {
        let result = String::from("really long string");
        result.as_str()
    }
    ```

    ...

    The problem is that result goes out of scope and gets cleaned up at the end of the longest function.
    We’re also trying to return a reference to result from the function. There is no way we can specify lifetime parameters
    that would change the dangling reference, and Rust won’t let us create a dangling reference. In this case, the best fix would be
    to return an owned data type rather than a reference so the calling function is then responsible for cleaning up the value.

    ...

    ```
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }
    ```

    The reason this function compiles without lifetime annotations is historical: in early versions (pre-1.0) of Rust,
    this code wouldn’t have compiled because every reference needed an explicit lifetime. At that time, the function signature would have been written like this:

    ```
    fn first_word<'a>(s: &'a str) -> &'a str {
    ```

    After writing a lot of Rust code, the Rust team found that Rust programmers were entering the same lifetime annotations over and over in particular situations.
    These situations were predictable and followed a few deterministic patterns. The developers programmed these patterns into the compiler’s code
    so the borrow checker could infer the lifetimes in these situations and wouldn’t need explicit annotations.

    This piece of Rust history is relevant because it’s possible that more deterministic patterns will emerge and be added to the compiler.
    In the future, even fewer lifetime annotations might be required.

    The patterns programmed into Rust’s analysis of references are called the lifetime elision rules. These aren’t rules for programmers to follow;
    they’re a set of particular cases that the compiler will consider, and if your code fits these cases, you don’t need to write the lifetimes explicitly.

    The elision rules don’t provide full inference. If Rust deterministically applies the rules but there is still ambiguity as to
    what lifetimes the references have, the compiler won’t guess what the lifetime of the remaining references should be.
    In this case, instead of guessing, the compiler will give you an error that you can resolve by adding the lifetime annotations
    that specify how the references relate to each other.

    Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes.

    The compiler uses three rules to figure out what lifetimes references have when there aren’t explicit annotations.
    The first rule applies to input lifetimes, and the second and third rules apply to output lifetimes.
    If the compiler gets to the end of the three rules and there are still references for which it can’t figure out lifetimes, the compiler will stop with an error.

    These rules apply to `fn` definitions, as well as `impl` blocks.

    The first rule is that each parameter that is a reference gets its own lifetime parameter. In other words,
    a function with one parameter gets one lifetime parameter: `fn foo<'a>(x: &'a i32)`;
    a function with two parameters gets two separate lifetime parameters: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`; and so on.

    The second rule is if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters:
    `fn foo<'a>(x: &'a i32) -> &'a i32`.

    The third rule is if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method,
    the lifetime of self is assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write
    because fewer symbols are necessary.

    ...

    One special lifetime we need to discuss is 'static, which denotes the entire duration of the program. All string literals have the 'static lifetime.

    ```
    let s: &'static str = "I have a static lifetime.";
    ```

    The text of this string is stored directly in the binary of your program, which is always available. Therefore, the lifetime of all string literals is 'static.

    You might see suggestions to use the 'static lifetime in error messages. But before specifying 'static as the lifetime for a reference,
    think about whether the reference you have actually lives the entire lifetime of your program or not. You might consider whether you want it to live that long,
    even if it could. Most of the time, the problem results from attempting to create a dangling reference or a mismatch of the available lifetimes.
    In such cases, the solution is fixing those problems, not specifying the 'static lifetime.
*/
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = "abcd";
    let string2 = "xyz";

    let result = longest(string1, string2);
    println!("The longest string is {}", result);
}
