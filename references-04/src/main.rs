fn main() {
    let mut s = String::from("some string");

    {
        // references allow you to refer to some value without taking ownership of it
        // either one mutable reference or any number of immutable references
        let _mutable_ref = &mut s;
        println!("{}", _mutable_ref.len());
    }

    let some = &s[..4]; // = &s[0..4] = &s[0..=3]
    println!("{}", some);

    let string = &s[5..];
    println!("{}", string);

    // The type of s here is &str: it’s a slice pointing to that specific point of the binary.
    // This is also why string literals are immutable; &str is an immutable reference.
    let sl = "string literal";
}
