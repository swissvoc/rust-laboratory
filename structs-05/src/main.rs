/*
    A struct, or structure, is a custom data type that lets you name and package together multiple related values that make up a meaningful group.
    If you’re familiar with an object-oriented language, a struct is like an object’s data attributes.

    ...

    Structs are similar to tuples, which were discussed in Chapter 3. Like tuples, the pieces of a struct can be different types.
    Unlike with tuples, you’ll name each piece of data so it’s clear what the values mean. As a result of these names,
    structs are more flexible than tuples: you don’t have to rely on the order of the data to specify or access the values of an instance.
*/
struct User {
    username: String,
    email: String,
    id: usize
}

/*
    You can also define structs that look similar to tuples, called tuple structs.
    Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields;
    rather, they just have the types of the fields. Tuple structs are useful when you want to give the whole tuple a name
    and make the tuple be a different type than other tuples, and naming each field as in a regular struct would be verbose or redundant.

    ...

    Each struct you define is its own type, even though the fields within the struct have the same types.
*/
struct Vector3(i32, i32, i32);

fn build_user(email: String, username: String, id: usize) -> User {
    User {
        email,
        username,
        id
    }
}

fn main() {
    let mut user1 = User {
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        id: 1
    };

    // To get a specific value from a struct, we can use dot notation.
    user1.email = String::from("anotheremail@example.com");

    let origin = Vector3(0, 0, 0);
}
