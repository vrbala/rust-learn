fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    // type annotations and not variable declarations
    // scalar types - integers, floats, booleans and characters
    // compound types - tuples, arrays

    // tuple example
    // elements with different type, fixed length
    let t = (500, 20.0, 'a');
    let (x, y, z) = t; // tuple destructuring
    println!("The values are {}, {}, {}", x, y, z);
    println!("The values are {}, {}, {}", t.0, t.1, t.2); // index based access

    // array example - arrays are allocated in stack!!
    // all elements - same type. fixed length
    let _a = [1, 2, 3, 4, 5]; // [u32; 5] - [type; length] is array type annotation

    // initializing an array with same value is done by [value; length]
    let a = [3; 8];
    println!("Array: {:?}", a);
}
