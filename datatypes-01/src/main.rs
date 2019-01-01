fn main() {
    /*
        data types
        - scalar types: integer types, floating-point types, boolean type, character type
        - compound types: tuples, arrays
    */

    // Each signed variant can store numbers from -(2n - 1) to 2^(n - 1) - 1 inclusive, where n is the number of bits that variant uses.
    // Unsigned variants can store numbers from 0 to 2^n - 1.
    let _signed_8bit: i8 = 0x7f;
    let _unsigned_8bit: u8 = 0xff;

    // Additionally, the isize and usize types depend on the kind of computer your program is running on:
    // 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.
    let _signed_arch: isize;
    let _unsigned_arch: usize;

    let _10k: i32 = 10_000;
    let _six_in_binary: u8 = 0b00000110;

    // Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively.
    // The default type is f64 because on modern CPUs it’s roughly the same speed as f32 but is capable of more precision.
    let _triple_seven = 7.77;

    let r#_true = true;

    let _ch = '초';

    // A tuple is a general way of grouping together some number of other values with a variety of types into one compound type.
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    let tup = (0, 0, 1);
    println!("({}, {}, {})", tup.0, tup.1, tup.2);

    // Arrays in Rust are different from arrays in some other languages because arrays in Rust have a fixed length, like tuples.
    // Arrays are useful when you want your data allocated on the stack rather than the heap or ...
    // An array isn’t as flexible as the vector type, though. A vector is a similar collection type provided by the standard library
    // ... that is allowed to grow or shrink in size.
    let arr = [1, 2, 3];
    println!("arr[0]: {}", arr[0]);
}
