/*
    The iterator pattern allows you to perform some task on a sequence of items in turn.
    An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished.
    When you use iterators, you don’t have to re-implement that logic yourself.

    In Rust, iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up.

    ...

    All iterators implement a trait named Iterator that is defined in the standard library. The definition of the trait looks like this:

    ```
    trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;

        // methods with default implementations elided
    }
    ```

    ...

    The `Iterator` trait has a number of different methods with default implementations provided by the standard library;
    you can find out about these methods by looking in the standard library API documentation for the `Iterator` trait.
    Some of these methods call the `next` method in their definition, which is why you’re required to implement the `next` method
    when implementing the `Iterator` trait.

    Methods that call `next` are called consuming adaptors, because calling them uses up the iterator.
    One example is the `sum` method, which takes ownership of the iterator and iterates through the items by repeatedly calling `next`, thus consuming the iterator.
    As it iterates through, it adds each item to a running total and returns the total when iteration is complete.

    ...

    Other methods defined on the Iterator trait, known as iterator adaptors, allow you to change iterators into different kinds of iterators.
    You can chain multiple calls to iterator adaptors to perform complex actions in a readable way. But because all iterators are lazy,
    you have to call one of the consuming adaptor methods to get results from calls to iterator adaptors.

    ...

    Now that we’ve introduced iterators, we can demonstrate a common use of closures that capture their environment by using the `filter` iterator adaptor.
    The `filter` method on an iterator takes a closure that takes each item from the iterator and returns a boolean. If the closure returns `true`,
    the value will be included in the iterator produced by `filter`. If the closure returns `false`, the value won’t be included in the resulting iterator.
*/

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

/*
    The `shoes_in_my_size` function takes ownership of a vector of shoes and a shoe size as parameters.
    It returns a vector containing only shoes of the specified size.

    In the body of `shoes_in_my_size`, we call `into_iter` to create an iterator that takes ownership of the vector.
    Then we call `filter` to adapt that iterator into a new iterator that only contains elements for which the closure returns `true`.

    The closure captures the `shoe_size` parameter from the environment and compares the value with each shoe’s size,
    keeping only shoes of the size specified. Finally, calling `collect` gathers the values returned by the adapted iterator into a vector
    that’s returned by the function.
*/

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

/*
    ```
    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;

            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }
    ```
*/

fn main() {
    let v1 =  vec![1, 2, 3];
    let v1_iter = v1.iter();
    let v1_sum: i32 = v1_iter.sum();

    println!("{}", v1_sum);

    let v2: Vec<i32> = vec![4, 5, 6];

    /*
        Because map takes a closure, we can specify any operation we want to perform on each item.
        This is a great example of how closures let you customize some behavior while reusing the iteration behavior that the Iterator trait provides.
    */
    let v3: Vec<i32> = v2.iter()
        .map(|x| x * 2)
        .collect();

    println!("{:?}", v3);

    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    println!("{:?}", in_my_size);
}