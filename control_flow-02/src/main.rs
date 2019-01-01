fn main() {
    let mut count = 0;

    loop {
        count += 1;

        println!("count: {}", count);

        if count == 8 {
            break count;
        }
    };

    while count != 0 {
        println!("count: {}", count);
        count -= 1;
    }

    let some_numbers: [u8; 5] = [1, 2, 3, 4, 5];

    for (index, element) in some_numbers.iter().enumerate() {
        println!("some_numbers[{}]: {}", index, element);
    }
}
