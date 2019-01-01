/*
    Every programming language has tools for effectively handling the duplication of concepts. In Rust, one such tool is generics.
    Generics are abstract stand-ins for concrete types or other properties. When we’re writing code, we can express the behavior of generics
    or how they relate to other generics without knowing what will be in their place when compiling and running the code.

    Similar to the way a function takes parameters with unknown values to run the same code on multiple concrete values,
    functions can take parameters of some generic type instead of a concrete type, like i32 or String.

    ...

    You might be wondering whether there is a runtime cost when you’re using generic type parameters.
    The good news is that Rust implements generics in such a way that your code doesn’t run any slower using generic types than it would with concrete types.

    Rust accomplishes this by performing monomorphization of the code that is using generics at compile time.
    Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.

    ...

    ```
    let integer = Some(5);
    let float = Some(5.0);
    ```

    When Rust compiles this code, it performs monomorphization. During that process, the compiler reads the values that have been used in `Option<T>` instances
    and identifies two kinds of `Option<T>`: one is `i32` and the other is `f64`. As such, it expands the generic definition of `Option<T>`
    into `Option_i32` and `Option_f64`, thereby replacing the generic definition with the specific ones.

    ...

    ```
    enum Option_i32 {
        Some(i32),
        None,
    }

    enum Option_f64 {
        Some(f64),
        None,
    }

   fn main() {
        let integer = Option_i32::Some(5);
        let float = Option_f64::Some(5.0);
    }
    ```

    Because Rust compiles generic code into code that specifies the type in each instance, we pay no runtime cost for using generics.
    When the code runs, it performs just as it would if we had duplicated each definition by hand.
    The process of monomorphization makes Rust’s generics extremely efficient at runtime.
*/

/*
    The syntax for using generics in struct definitions is similar to that used in function definitions.
    First, we declare the name of the type parameter inside angle brackets just after the name of the struct.
    Then we can use the generic type in the struct definition where we would otherwise specify concrete data types.

    Note that because we’ve used only one generic type to define `Point<T>`, this definition says that the `Point<T>` struct is generic over some type `T`,
    and the fields `x` and `y` are both that same type, whatever that type may be.

    ...

    To define a `Point` struct where `x` and `y` are both generics but could have different types, we can use multiple generic type parameters.
    For example, we can change the definition of `Point` to be generic over types `T` and `U` where `x` is of type `T` and `y` is of type `U`.
*/
struct Point<T, U> {
    x: T,
    y: U,
}

impl Point<f64, f64> {
    fn _distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T, U> Point<T, U> {
    fn _x(&self) -> &T {
        &self.x
    }

    fn _y(&self) -> &U {
        &self.y
    }

    // Generic type parameters in a struct definition aren’t always the same as those you use in that struct’s method signatures.
    fn _mix_up<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

/*
    A trait tells the Rust compiler about functionality a particular type has and can share with other types.
    We can use traits to define shared behavior in an abstract way. We can use trait bounds to specify that a generic can be any type that has certain behavior.

    Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.

    ...

    A type’s behavior consists of the methods we can call on that type.
    Different types share the same behavior if we can call the same methods on all of those types.
    Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.
*/
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

/*
    Now that you know how to define traits and implement those traits on types, we can explore how to use traits to accept arguments of many different types.

    ...

    ```
    pub fn notify(item: impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }
    ```

    ...

    When should you use this form over `impl Trait`? While `impl Trait` is nice for shorter examples, trait bounds are nice for more complex ones.
    For example, say we wanted to take two things that implement `Summary`:

    ```
    pub fn notify(item1: impl Summary, item2: impl Summary) {
    ```

    This would work well if `item1` and `item2` were allowed to have different types (as long as both implement `Summary`).
    But what if you wanted to force both to have the exact same type? That is only possible if you use a trait bound:

    ```
    pub fn notify<T: Summary>(item1: T, item2: T) {
    ```

    ...

    ```
    // specify multiple traits with `+`
    pub fn notify(item: impl Summary + Display) {
    ```

    ...

    However, there are downsides to using too many trait bounds. Each generic has its own trait bounds,
    so functions with multiple generic type parameters can have lots of trait bound information between a function’s name and its parameter list,
    making the function signature hard to read. For this reason, Rust has alternate syntax for specifying trait bounds inside a where clause
    after the function signature. So instead of writing this:

    ```
    fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
    ```

    we can use a where clause, like this:

    ```
    fn some_function<T, U>(t: T, u: U) -> i32
        where T: Display + Clone,
              U: Clone + Debug
    {
    ```

    ...

    Traits and trait bounds let us write code that uses generic type parameters to reduce duplication
    but also specify to the compiler that we want the generic type to have particular behavior. The compiler can then use the trait bound information
    to check that all the concrete types used with our code provide the correct behavior. In dynamically typed languages,
    we would get an error at runtime if we called a method on a type that the type didn’t implement. But Rust moves these errors to compile time
    so we’re forced to fix the problems before our code is even able to run. Additionally, we don’t have to write code that checks for behavior at runtime
    because we’ve already checked at compile time. Doing so improves performance without having to give up the flexibility of generics.
*/
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let _p1 = Point { x: 2, y: 4.5 };
    let _p2 = Point { x: 0.5, y: 0 };

    println!("_p1.x: {}, _p1.y: {}", _p1.x, _p1.y);
    println!("_p2.x: {}, _p2.y: {}", _p2.x, _p2.y);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
    hockey team in the NHL."),
    };

    println!("New article available! {}", article.summarize());

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
