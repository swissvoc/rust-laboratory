use std::collections::HashMap;

/*
    Rust’s standard library includes a number of very useful data structures called collections. Most other data types represent one specific value,
    but collections can contain multiple values. Unlike the built-in array and tuple types, the data these collections point to is stored on the heap,
    which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs.

    Each kind of collection has different capabilities and costs, and choosing an appropriate one for your current situation is a skill you’ll develop over time.

*/
fn main() {
    /*
        Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory.
        Vectors can only store values of the same type. They are useful when you have a list of items, such as the lines of text in a file
        or the prices of items in a shopping cart.
    */
    {
        let mut v = vec![1, 2, 3, 4, 5];
        println!("v[0]: {}", &v[0]);

        match v.get(0) {
            Some(value) => println!("v[0]: {}", value),
            None => println!("Array index out of bounds.")
        }

        for mut_value in &mut v {
            // dereference operator `*`
            *mut_value *= 2;
        }

        for value in &v {
            println!("{}!", value)
        }
    }

    /*
        New Rustaceans commonly get stuck on strings for a combination of three reasons:
        Rust’s propensity for exposing possible errors, strings being a more complicated data structure than many programmers give them credit for, and UTF-8.
        These factors combine in a way that can seem difficult when you’re coming from other programming languages.

        It’s useful to discuss strings in the context of collections because strings are implemented as a collection of bytes,
        plus some methods to provide useful functionality when those bytes are interpreted as text.

        ...

        The String type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned,
        UTF-8 encoded string type. When Rustaceans refer to “strings” in Rust, they usually mean the String and the string slice &str types,
        not just one of those types. Although this section is largely about String, both types are used heavily in Rust’s standard library,
        and both String and string slices are UTF-8 encoded.
    */
    {
        let _str1 = String::new();
        let _str2 = "some string".to_string();

        let mut r#str = String::from("가나다라마바사");
        r#str.push_str("아자차카타파");

        let _tic = String::from("tic");
        let _tac = String::from("tac");
        let _toe = String::from("toe");

        let _beep = String::from("beep");
        let _boop = String::from("boop");

        let _beep_boop = format!("{}-{}", _beep, _boop);

        println!("{}, {}", _tic + "-" + &_tac + "-" + &_toe, _beep_boop);

        let pineapple = String::from("파인애플");

        /*
            Source: https://en.wikipedia.org/wiki/UTF-8

            1. One byte is needed to encode the 128 US-ASCII characters (Unicode range U+0000 to U+007F).
            2. Two bytes are needed for Latin letters with diacritics and for characters from Greek, Cyrillic, ... alphabets.
            (Unicode range U+0080 to U+07FF).
            3. Three bytes are needed for the rest of the Basic Multilingual Plane (which contains virtually all characters in common use).
            4. Four bytes are needed for characters in the other planes of Unicode, which are rarely used in practice.
        */
        println!("{}", pineapple.len()); // 파 (3 bytes) + 인 (3 bytes) + 애 (3 bytes) + 플 (3 bytes)
        println!("{}", &pineapple[9..=11]); // 플 (3 bytes)

        for r#char in pineapple.chars() {
            println!("{}", r#char);
        }

        /*
             The type HashMap<K, V> stores a mapping of keys of type K to values of type V. It does this via a hashing function,
             which determines how it places these keys and values into memory. Many programming languages support this kind of data structure,
             but they often use a different name, such as hash, map, object, hash table, dictionary, or associative array, just to name a few.

             Hash maps are useful when you want to look up data not by using an index, as you can with vectors, but by using a key that can be of any type.
             For example, in a game, you could keep track of each team’s score in a hash map in which each key is a team’s name and the values are each team’s score.
             Given a team name, you can retrieve its score.
        */
        let mut scores = HashMap::new();

        scores.insert(String::from("Red"), 15);
        scores.insert(String::from("Blue"), 10);

        /*
            Just like vectors, hash maps store their data on the heap. This `HashMap` `has keys of type `String` and values of type `i32`.
            Like vectors, hash maps are homogeneous: all of the keys must have the same type, and all of the values must have the same type.

            Another way of constructing a hash map is by using the `collect` method on a vector of tuples, where each tuple consists of a key and its value.
            The `collect` method gathers data into a number of collection types, including `HashMap`.

            ```
            let teams  = vec![String::from("Blue"), String::from("Yellow")];
            let initial_scores = vec![10, 50];

            let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
            ```
        */

        // The result is wrapped in Some because get returns an Option<&V>;
        // if there’s no value for that key in the hash map, get will return None.
        // let team_name = String::from("Blue");
        // let _score = scores.get(&team_name);

        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }

        /*
            It’s common to check whether a particular key has a value and, if it doesn’t, insert a value for it.
            Hash maps have a special API for this called `entry` that takes the key you want to check as a parameter.
            The return value of the `entry` method is an enum called `Entry` that represents a value that might or might not exist.
        */

        scores.entry(String::from("Red")).or_insert(20);
        scores.entry(String::from("Blue")).or_insert(25);

        println!("{:?}", scores);
    }
}
