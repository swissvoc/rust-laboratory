use std::ops::Deref;

/*
    A pointer is a general concept for a variable that contains an address in memory. This address refers to, or “points at,” some other data.
    The most common kind of pointer in Rust is a reference, which you learned about in Chapter 4. References are indicated by the `&` symbol
    and borrow the value they point to. They don’t have any special capabilities other than referring to data.
    Also, they don’t have any overhead and are the kind of pointer we use most often.

    Smart pointers, on the other hand, are data structures that not only act like a pointer but also have additional metadata and capabilities.
    The concept of smart pointers isn’t unique to Rust: smart pointers originated in C++ and exist in other languages as well.
    In Rust, the different smart pointers defined in the standard library provide functionality beyond that provided by references.
    One example that we’ll explore in this chapter is the reference counting smart pointer type.
    This pointer enables you to have multiple owners of data by keeping track of the number of owners and, when no owners remain, cleaning up the data.

    In Rust, which uses the concept of ownership and borrowing, an additional difference between references and smart pointers is that references are pointers
    that only borrow data; in contrast, in many cases, smart pointers own the data they point to.

    ...

    Smart pointers are usually implemented using structs. The characteristic that distinguishes a smart pointer from an ordinary struct is that
    smart pointers implement the `Deref` and `Drop` traits. The `Deref` trait allows an instance of the smart pointer struct to behave like a reference
    so you can write code that works with either references or smart pointers. The Drop trait allows you to customize the code
    that is run when an instance of the smart pointer goes out of scope.

    ...

    At compile time, Rust needs to know how much space a type takes up. One type whose size can’t be known at compile time is a recursive type,
    where a value can have as part of itself another value of the same type. Because this nesting of values could theoretically continue infinitely,
    Rust doesn’t know how much space a value of a recursive type needs. However, boxes have a known size, so by inserting a box in a recursive type definition,
    you can have recursive types.

    Let’s explore the cons list, which is a data type common in functional programming languages, as an example of a recursive type.
    The cons list type we’ll define is straightforward except for the recursion; therefore, the concepts in the example we’ll work with will be useful
    any time you get into more complex situations involving recursive types.

    ...

    A cons list is a data structure that comes from the Lisp programming language and its dialects. In Lisp, the cons function (short for “construct function”)
    constructs a new pair from its two arguments, which usually are a single value and another pair. These pairs containing pairs form a list.

    The cons function concept has made its way into more general functional programming jargon: “to cons x onto y” informally means
    to construct a new container instance by putting the element x at the start of this new container, followed by the container y.

    Each item in a cons list contains two elements: the value of the current item and the next item.
    The last item in the list contains only a value called Nil without a next item. A cons list is produced by recursively calling the cons function.
    The canonical name to denote the base case of the recursion is Nil. Note that this is not the same as the “null” or “nil” concept in Chapter 6,
    which is an invalid or absent value.

    Although functional programming languages use cons lists frequently, the cons list isn’t a commonly used data structure in Rust.
    Most of the time when you have a list of items in Rust, Vec<T> is a better choice to use.
    Other, more complex recursive data types are useful in various situations, but by starting with the cons list,
    we can explore how boxes let us define a recursive data type without much distraction.

    ...

    Contrast this with what happens when Rust tries to determine how much space a recursive type like the List enum in Listing 15-2 needs.
    The compiler starts by looking at the Cons variant, which holds a value of type i32 and a value of type List.
    Therefore, Cons needs an amount of space equal to the size of an i32 plus the size of a List. To figure out how much memory the List type needs,
    the compiler looks at the variants, starting with the Cons variant. The Cons variant holds a value of type i32 and a value of type List,
    and this process continues infinitely, as shown in Figure 15-1.

    https://doc.rust-lang.org/book/img/trpl15-01.svg

    ...

    The `Cons` variant will need the size of an `i32` plus the space to store the box’s pointer data. The `Nil` variant stores no values,
    so it needs less space than the Cons variant. We now know that any `List` value will take up the size of an `i32` plus the size of a box’s pointer data.
    By using a box, we’ve broken the infinite, recursive chain, so the compiler can figure out the size it needs to store a `List` value.
    Figure 15-2 shows what the `Cons` variant looks like now.

    https://doc.rust-lang.org/book/img/trpl15-02.svg

    ...

    Boxes provide only the indirection and heap allocation; they don’t have any other special capabilities,
    like those we’ll see with the other smart pointer types. They also don’t have any performance overhead that these special capabilities incur,
    so they can be useful in cases like the cons list where the indirection is the only feature we need.
*/
enum List {
    Cons(i32, Box<List>),
    Nil,
}

/*
    Implementing the `Deref` trait allows you to customize the behavior of the dereference operator, `*` (as opposed to the multiplication or glob operator).
    By implementing `Deref` in such a way that a smart pointer can be treated like a regular reference, you can write code that operates on references
    and use that code with smart pointers too.

    ...

    We define a struct named `MyBox` and declare a generic parameter `T`, because we want our type to hold values of any type.
    The `MyBox` type is a tuple struct with one element of type `T`. The `MyBox::new` function takes one parameter of type `T`
    and returns a `MyBox` instance that holds the value passed in.

    ...

    The `type Target = T`; syntax defines an associated type for the `Deref` trait to use.
    Associated types are a slightly different way of declaring a generic parameter, but you don’t need to worry about them for now;
    we’ll cover them in more detail in Chapter 19.

    We fill in the body of the `deref` method with `&self.0` so `deref` returns a reference to the value we want to access with the `*` operator.

    ...

    Without the `Deref` trait, the compiler can only dereference `&` references.
    The `deref` method gives the compiler the ability to take a value of any type that implements `Deref`
    and call the `deref` method to get a `&` reference that it knows how to dereference.

    ...

    Rust substitutes the `*` operator with a call to the `deref` method and then a plain dereference so we don’t have to think about
    whether or not we need to call the `deref` method. This Rust feature lets us write code that functions identically whether we have a regular reference
    or a type that implements `Deref`.

    The reason the `deref` method returns a reference to a value, and that the plain dereference outside the parentheses in `*(y.deref())` is still necessary,
    is the ownership system. If the `deref` method returned the value directly instead of a reference to the value, the value would be moved out of self.
    We don’t want to take ownership of the inner value inside `MyBox<T>` in this case or in most cases where we use the dereference operator.
*/

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

/*
    The second trait important to the smart pointer pattern is `Drop`, which lets you customize what happens when a value is about to go out of scope.
    You can provide an implementation for the `Drop` trait on any type, and the code you specify can be used to release resources like files or network connections.
    We’re introducing `Drop` in the context of smart pointers because the functionality of the Drop trait is almost always used when implementing a smart pointer.
    For example, `Box<T>` customizes `Drop` to deallocate the space on the heap that the box points to.

    In some languages, the programmer must call code to free memory or resources every time they finish using an instance of a smart pointer.
    If they forget, the system might become overloaded and crash. In Rust, you can specify that a particular bit of code be run whenever a value goes out of scope,
    and the compiler will insert this code automatically. As a result, you don’t need to be careful about placing cleanup code everywhere in a program
    that an instance of a particular type is finished with—you still won’t leak resources!

    ...

    Unfortunately, it’s not straightforward to disable the automatic `drop` functionality.
    Disabling `drop` isn’t usually necessary; the whole point of the `Drop` trait is that it’s taken care of automatically.
    Occasionally, however, you might want to clean up a value early.
    One example is when using smart pointers that manage locks: you might want to force the `drop` method that releases the lock to run
    so other code in the same scope can acquire the lock.

    Rust doesn’t let you call the `Drop` trait’s `drop` method manually; instead you have to call the `std::mem::drop` function provided by the standard library
    if you want to force a value to be dropped before the end of its scope.
*/
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    /*
        The most straightforward smart pointer is a box, whose type is written `Box<T>`. Boxes allow you to store data on the heap rather than the stack.
        What remains on the stack is the pointer to the heap data.

        Boxes don’t have performance overhead, other than storing their data on the heap instead of on the stack.
        But they don’t have many extra capabilities either. You’ll use them most often in these situations:

        1. When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
        2. When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
        3. When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type (trait objects)

        We’ll demonstrate the first situation in the “Enabling Recursive Types with Boxes” section.

        In the second case, transferring ownership of a large amount of data can take a long time because the data is copied around on the stack.
        To improve performance in this situation, we can store the large amount of data on the heap in a box.
        Then, only the small amount of pointer data is copied around on the stack, while the data it references stays in one place on the heap.

        The third case is known as a trait object, and Chapter 17 devotes an entire section, “Using Trait Objects That Allow for Values of Different Types,”
        just to that topic. So what you learn here you’ll apply again in Chapter 17!

        ...

        Putting a single value on the heap isn’t very useful, so you won’t use boxes by themselves in this way very often.
        Having values like a single `i32` on the stack, where they’re stored by default, is more appropriate in the majority of situations.
        Let’s look at a case where boxes allow us to define types that we wouldn’t be allowed to if we didn’t have boxes.

        ...
    */
    let _list = List::Cons(1,
                           Box::new(List::Cons(2,
                                               Box::new(List::Cons(3,
                                                                   Box::new(List::Nil))))));
    /*
        Deref coercion is a convenience that Rust performs on arguments to functions and methods.
        Deref coercion converts a reference to a type that implements `Deref` into a reference to a type that `Deref` can convert the original type into.
        Deref coercion happens automatically when we pass a reference to a particular type’s value as an argument to a function or method
        that doesn’t match the parameter type in the function or method definition.
        A sequence of calls to the `deref` method converts the type we provided into the type the parameter needs.

        Deref coercion was added to Rust so that programmers writing function and method calls don’t need to add as many explicit references
        and dereferences with `&` and `*`. The deref coercion feature also lets us write more code that can work for either references or smart pointers.

        ...

        When the `Deref` trait is defined for the types involved, Rust will analyze the types and use `Deref::deref` as many times as necessary
        to get a reference to match the parameter’s type. The number of times that `Deref::deref` needs to be inserted is resolved at compile time,
        so there is no runtime penalty for taking advantage of deref coercion!

        ...

        Similar to how you use the `Deref` trait to override the `*` operator on immutable references,
        you can use the `DerefMut` trait to override the `*` operator on mutable references.

        Rust does deref coercion when it finds types and trait implementations in three cases:

        1. From `&T` to `&U` when `T: Deref<Target=U>`
        2. From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
        3. From `&mut T` to `&U` when `T: Deref<Target=U>`

        The first two cases are the same except for mutability. The first case states that if you have a `&T`, and `T` implements `Deref` to some type `U`,
        you can get a `&U` transparently. The second case states that the same deref coercion happens for mutable references.

        The third case is trickier: Rust will also coerce a mutable reference to an immutable one.
        But the reverse is not possible: immutable references will never coerce to mutable references.
        Because of the borrowing rules, if you have a mutable reference, that mutable reference must be the only reference to that data
        (otherwise, the program wouldn’t compile). Converting one mutable reference to one immutable reference will never break the borrowing rules.
        Converting an immutable reference to a mutable reference would require that there is only one immutable reference to that data,
        and the borrowing rules don’t guarantee that.

        Therefore, Rust can’t make the assumption that converting an immutable reference to a mutable reference is possible.
    */
    let my_box = MyBox::new(String::from("Rust"));
    hello(&my_box); // hello(&(*m)[..]);

    let _c = CustomSmartPointer { data: String::from("my stuff") };
}