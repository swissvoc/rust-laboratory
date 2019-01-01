use std::rc::Rc;

/*
    In the majority of cases, ownership is clear: you know exactly which variable owns a given value.
    However, there are cases when a single value might have multiple owners. For example, in graph data structures, multiple edges might point to the same node,
    and that node is conceptually owned by all of the edges that point to it. A node shouldn’t be cleaned up unless it doesn’t have any edges pointing to it.

    To enable multiple ownership, Rust has a type called `Rc<T>`, which is an abbreviation for reference counting.
    The `Rc<T>` type keeps track of the number of references to a value which determines whether or not a value is still in use.
    If there are zero references to a value, the value can be cleaned up without any references becoming invalid.

    Imagine `Rc<T>` as a TV in a family room. When one person enters to watch TV, they turn it on. Others can come into the room and watch the TV.
    When the last person leaves the room, they turn off the TV because it’s no longer being used. If someone turns off the TV while others are still watching it,
    there would be uproar from the remaining TV watchers!

    We use the `Rc<T>` type when we want to allocate some data on the heap for multiple parts of our program to read
    and we can’t determine at compile time which part will finish using the data last. If we knew which part would finish last,
    we could just make that part the data’s owner, and the normal ownership rules enforced at compile time would take effect.

    Note that `Rc<T>` is only for use in single-threaded scenarios. When we discuss concurrency in Chapter 16,
    we’ll cover how to do reference counting in multi-threaded programs.

    ...

    Via immutable references, `Rc<T>` allows you to share data between multiple parts of your program for reading only.
    If `Rc<T>` allowed you to have multiple mutable references too, you might violate one of the borrowing rules discussed in Chapter 4:
    multiple mutable borrows to the same place can cause data races and inconsistencies.

    But being able to mutate data is very useful! In the next section, we’ll discuss the interior mutability pattern and the `RefCell<T>` type
    that you can use in conjunction with an `Rc<T>` to work with this immutability restriction.
*/
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    /*
        We could have called `a.clone()` rather than `Rc::clone(&a)`, but Rust’s convention is to use `Rc::clone` in this case.
        The implementation of `Rc::clone` doesn’t make a deep copy of all the data like most types’ implementations of `clone` do.
        The call to `Rc::clone` only increments the reference count, which doesn’t take much time. Deep copies of data can take a lot of time.
        By using `Rc::clone` for reference counting, we can visually distinguish between the deep-copy kinds of clones and the kinds of clones
        that increase the reference count. When looking for performance problems in the code, we only need to consider the deep-copy clones
        and can disregard calls to `Rc::clone`.
    */
    let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
    let b = List::Cons(3, Rc::clone(&a));
    let c = List::Cons(4, Rc::clone(&a));
}
