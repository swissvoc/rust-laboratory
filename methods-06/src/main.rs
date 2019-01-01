struct Rectangle {
    width: u32,
    height: u32,
}

/*
    Methods are similar to functions: they’re declared with the fn keyword and their name, they can have parameters and a return value,
    and they contain some code that is run when they’re called from somewhere else. However, methods are different from functions in that
    they’re defined within the context of a struct (or an enum or a trait object), and their first parameter is always self,
    which represents the instance of the struct the method is being called on.
*/
impl Rectangle {
    fn _new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn _area(&self) -> u32 {
        self.width * self.height
    }

    fn _can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1._area()
    );
}