/*
    Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data;
    normally, this action is disallowed by the borrowing rules.

    To mutate data, the pattern uses `unsafe` code inside a data structure to bend Rust’s usual rules that govern mutation and borrowing.
    We haven’t yet covered unsafe code; we will in Chapter 19. We can use types that use the interior mutability pattern
    when we can ensure that the borrowing rules will be followed at runtime, even though the compiler can’t guarantee that.
    The `unsafe` code involved is then wrapped in a safe API, and the outer type is still immutable.

    ...

    Unlike `Rc<T>`, the `RefCell<T>` type represents single ownership over the data it holds. So, what makes `RefCell<T>` different from a type like `Box<T>`?
    Recall the borrowing rules you learned in Chapter 4:

    1. At any given time, you can have either (but not both of) one mutable reference or any number of immutable references.
    2. References must always be valid.

    With references and `Box<T>`, the borrowing rules’ invariants are enforced at compile time. With `RefCell<T>`, these invariants are enforced at runtime.
    With references, if you break these rules, you’ll get a compiler error. With `RefCell<T>`, if you break these rules, your program will panic and exit.

    The advantages of checking the borrowing rules at compile time are that errors will be caught sooner in the development process,
    and there is no impact on runtime performance because all the analysis is completed beforehand.
    For those reasons, checking the borrowing rules at compile time is the best choice in the majority of cases, which is why this is Rust’s default.

    The advantage of checking the borrowing rules at runtime instead is that certain memory-safe scenarios are then allowed,
    whereas they are disallowed by the compile-time checks. Static analysis, like the Rust compiler, is inherently conservative.
    Some properties of code are impossible to detect by analyzing the code: the most famous example is the Halting Problem,
    which is beyond the scope of this book but is an interesting topic to research.

    Because some analysis is impossible, if the Rust compiler can’t be sure the code complies with the ownership rules, it might reject a correct program;
    in this way, it’s conservative. If Rust accepted an incorrect program, users wouldn’t be able to trust in the guarantees Rust makes.
    However, if Rust rejects a correct program, the programmer will be inconvenienced, but nothing catastrophic can occur.
    The `RefCell<T>` type is useful when you’re sure your code follows the borrowing rules but the compiler is unable to understand and guarantee that.

    Similar to `Rc<T>`, `RefCell<T>` is only for use in single-threaded scenarios and will give you a compile-time error if you try using it in a multi-threaded context.
*/
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    /*
        When creating immutable and mutable references, we use the `&` and `&mut` syntax, respectively. With `RefCell<T>`, we use the borrow and borrow_mut methods,
        which are part of the safe API that belongs to `RefCell<T>`. The borrow method returns the smart pointer type `Ref<T>`,
        and `borrow_mut` returns the smart pointer type `RefMut<T>`. Both types implement `Deref`, so we can treat them like regular references.

        The `RefCell<T>` keeps track of how many `Ref<T>` and `RefMut<T>` smart pointers are currently active. Every time we call borrow,
        the `RefCell<T>` increases its count of how many immutable borrows are active. When a `Ref<T>` value goes out of scope,
        the count of immutable borrows goes down by one. Just like the compile-time borrowing rules,
        `RefCell<T>` lets us have many immutable borrows or one mutable borrow at any point in time.

        If we try to violate these rules, rather than getting a compiler error as we would with references, the implementation of `RefCell<T>` will panic at runtime.

        ...

        A common way to use RefCell<T> is in combination with Rc<T>. Recall that Rc<T> lets you have multiple owners of some data,
        but it only gives immutable access to that data. If you have an Rc<T> that holds a RefCell<T>,
        you can get a value that can have multiple owners and that you can mutate!

        For example, recall the cons list example in Listing 15-18 where we used Rc<T> to allow multiple lists to share ownership of another list.
        Because Rc<T> holds only immutable values, we can’t change any of the values in the list once we’ve created them.
        Let’s add in RefCell<T> to gain the ability to change the values in the lists.

        ...

        This technique is pretty neat! By using `RefCell<T>`, we have an outwardly immutable `List` value.
        But we can use the methods on `RefCell<T>` that provide access to its interior mutability so we can modify our data when we need to.
        The runtime checks of the borrowing rules protect us from data races, and it’s sometimes worth trading a bit of speed for this flexibility in our data structures.

        The standard library has other types that provide interior mutability, such as `Cell<T>`, which is similar except that
        instead of giving references to the inner value, the value is copied in and out of the `Cell<T>`. There’s also `Mutex<T>`,
        which offers interior mutability that’s safe to use across threads; we’ll discuss its use in Chapter 16.

        Check out the standard library docs for more details on the differences between these types.
    */
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(List::Cons(Rc::clone(&value), Rc::new(List::Nil)));

    let b = List::Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = List::Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
