#[derive(Debug)]
struct Rectangle {
    width: f64,
    length: f64,
}

impl Rectangle {
    // method
    fn area(self : &Rectangle) -> f64 {
        self.width * self.length
    }

    // associated function - similar to static in C++ and java
    fn square(side : f64) -> Rectangle {
        Rectangle{
            width: side,
            length: side,
        }
    }
}

fn main() {
    let width = 2.0;
    let length = 3.0;

    println!("The area of rectangle is {}.", area(width, length));
    println!("The area of rectangle is {}.", area1((width, length,)));
    println!("The area of rectangle is {}.", area2(Rectangle{width, length}));
    println!("The area of rectangle is {}.", Rectangle{width, length}.area());

    println!("Square rectangle {:?}", Rectangle::square(10.0)); // works because of deriving the Debug trait
}

fn area(width : f64, length : f64) -> f64 {
    width * length
}

// using tuple to group length and width and indicate that they belong together

// tuple is better than using fields separately, but the imposed ordering of the fields
// is more restrictive and user should always be aware that first dimension is width
// and the next is length
fn area1(dimensions : (f64, f64)) -> f64 {
    dimensions.0 * dimensions.1
}

// using a explicit struct to name the concept and add more meaning to the
// data we are dealing with.
fn area2(rect : Rectangle) -> f64 {
    rect.width * rect.length
}
