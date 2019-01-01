fn main() {
    /*
        All programs have to manage the way they use a computer’s memory while running.
        Some languages have garbage collection that constantly looks for no longer used memory as the program runs;
        in other languages, the programmer must explicitly allocate and free the memory.

        Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks at compile time.
        None of the ownership features slow down your program while it’s running.
    */

    /*
        the stack
        - the place where parameters and local variables of each function is stored
        - stores values in the order it gets them and removes the values in the opposite order (last in, first out)
        - adding data is called pushing onto the stack, and removing data is called popping off the stack
        - all data on the stack must take up a known, fixed size

        the heap
        - a free-floating region where data with a size unknown at compile time or a size that might change is stored
        - accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there
        - allocating a large amount of space on the heap can also take time

        ...

        "The heap is less organized: when you put data on the heap, you ask for some amount of space.
        The operating system finds an empty spot somewhere in the heap that is big enough, marks it as being in use, and returns a pointer,
        which is the address of that location. This process is called allocating on the heap, sometimes abbreviated as just 'allocating.'"
    */

    /*
        ownership rules
        1. each value has a variable that is called its owner
        2. only one owner at a time.
        3. when the owner goes out of scope, the value will be dropped
    */

    {
        /*
            In the case of a string literal, we know the contents at compile time, so the text is hardcoded directly into the final executable.
            This is why string literals are fast and efficient. But these properties only come from the string literal’s immutability.
        */
        let _str: &str = "hello, world";
    } // variable 'str' goes out of scope, and the memory is freed

    /*
        https://doc.rust-lang.org/book/img/trpl04-01.svg

        ...

        A String is made up of three parts, a pointer to the memory that holds the contents of the string, a length, and a capacity.
        This group of data is stored on the stack. On the right is the memory on the heap that holds the contents.

        ...

        If you’ve heard the terms shallow copy and deep copy while working with other languages,
        the concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow copy.
        But because Rust also invalidates the first variable, instead of being called a shallow copy, it’s known as a move.

        ...

        https://doc.rust-lang.org/book/img/trpl04-04.svg
    */
    let _str1 = String::from("some string");
    let _str2 = _str1;

    // copy the heap data of _str2
    let _str2_cl = _str2.clone();

    /*
        Types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make.
        In other words, there’s no difference between deep and shallow copying here, so calling clone would not do anything different
        from the usual shallow copying and we can leave it out.

        Rust has a special annotation called the `Copy` trait that we can place on types like integers that are stored on the stack.
        If a type has the Copy trait, an older variable is still usable after assignment. Rust won’t let us annotate a type with the `Copy` trait
        if the type, or any of its parts, has implemented the `Drop` trait.
    */
    let _n1 = 100;
    let _n2 = _n1;

    // Passing a variable to a function will move or copy, just as assignment does. Returning values can also transfer ownership.
    let _str3 = takes_and_gives_back(_str2);
    takes_ownership(_str3);
}

fn takes_ownership(str: String) {
    println!("{}", str);
}

fn takes_and_gives_back(r#str: String) -> String {
    r#str
}
