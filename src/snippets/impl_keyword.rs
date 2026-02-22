// In JavaScript, you'd write a class like this:
//
// class Rectangle {
//   constructor(width, height) {
//     this.width = width;
//     this.height = height;
//   }
//
//   area() {
//     return this.width * this.height;
//   }
//
//   is_square() {
//     return this.width === this.height;
//   }
// }
//
// const r = new Rectangle(10, 5);
// console.log(r.area());      // 50
// console.log(r.is_square()); // false

// In Rust, data and behavior are separated.
// First you define the SHAPE of the data (the struct):
struct Rectangle {
    width: u32,
    height: u32,
}

// Then you attach behavior to it using `impl`:
impl Rectangle {
    // Associated function (like a static method in JS) — called with Rectangle::new(...)
    // It doesn't take `self` because it's not called on an instance, it creates one.
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    // Method — called on an instance, like r.area()
    // `&self` means "a reference to the instance this is called on" (like `this` in JS)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn main() {
    let r = Rectangle::new(10, 5);

    println!("Area: {}", r.area());       // Area: 50
    println!("Square? {}", r.is_square()); // Square? false
}
