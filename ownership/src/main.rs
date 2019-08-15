fn main() {

    let mut str = String::from("hello"); // "...." is string literal allocated in stack

    // String::from builds a string in heap using the literal passed in, as initial value
    // now that constructed string value can be mutable

    str.push_str(", world");
    println!("String: {}", str);

    // move & copy
    // let _s2 = str; this causes move - see below
    let _s2 = str.clone(); // without .clone, str will be moved to s2 and cannot be used anymore i.e. println! below will cause failure.
    println!("{}", str);
    // clone is deep copy
    // move is shallow copy + invalidating original reference.

    // stack only data can be efficiently copied
    // heap data types have Drop trait
    // stack data types have Copy trait
    // for obvious reasons, there can't be a data type with both the above traits
    
    // most scalars and compound types with scalars only are Copy-able

    // passing values are arguments to functions can have move or copy semantics dicated by above
    // returning values from functions can also have similar semantics
    
    let s1 = String::from("hello");
    takes_ownership(s1); // moves
    // trying to use s1 here fails

    let _s2 = get_string(); // gets a string by move


    // --------------------------------------------------------------------------------
    
    // passing/receiving scalars do copy and not moves
    let _f = five(); // gets value by copy

    let x = 5;
    take_num(x);
    println!("after calling function: {}", x);


    // --------------------------------------------------------------------------------
    // REFERENCES
    // Can be used when we want to pass a value to a function without giving away the ownership
    // since references don't own the values, there is no drop when they go out of scope.
    // --------------------------------------------------------------------------------

    // problem: how to send a string to a function and continue using it?
    let s1 = String::from("hello");
    let (s1, l) = calculate_length(s1);
    println!("Length of {} is {}.", s1, l);
    // above is a lot of ceremony for a common usage of passing a value to a function and continue using it
    // references (borrowing) help avoid it

    let l = calculate_length2(&s1);
    // --------------------------------------------------------------------------------
    // here s1 points to the actual string in heap and we are getting a reference to it
    //  some reference '&s1'
    //  |
    //  v     ----------------
    //  s1 -> |String "hello"|
    //        ----------------
    // --------------------------------------------------------------------------------
    println!("Length of {} is {}.", s1, l);

    // --------------------------------------------------------------------------------
    // SLICES
    // they are references to contiguous block of data
    // mostly similar to references but have associated range.
    // eg: substring, sub array etc.,
    // created by &s[start..end]
    // type is &str for strings.
    // they usually store reference to first element and the length of slice.
    //
    // BONUS: string literals are actually string slices. so, of type &str
    // --------------------------------------------------------------------------------
    let s1 = "Hello world";
    let hello = &s1[..5]; // starting value is omitted here because it is first position 0
    println!("Substr is {}.", hello);

    let s1 = String::from("Hello world");
    let fw = first_word(&s1);
    println!("first word is {}.", fw);
}

fn first_word(s : &String) -> &str {

    for (i, &val) in s.as_bytes().iter().enumerate() {
        if val == b' ' {
            return &s[..i] // slice till first space character
        }
    }

    return &s[..] // slice representing complete string
}

fn calculate_length2(s : &String) -> usize {
    // --------------------------------------------------------------------------------
    // after the function call - 
    //  s
    //  |
    //  v     ----------------
    //  s1 -> |String "hello"|
    //        ----------------
    // --------------------------------------------------------------------------------


    // --------------------------------------------------------------------------------
    // another thing to note is the reference is read-only. so s can't be modified here
    // to do that, one must use mutable reference like '&mut String'
    // and there can be one and only one mutable reference to a piece of data in a scope
    // --------------------------------------------------------------------------------

    // --------------------------------------------------------------------------------
    // Some more niceties provided by references
    // when there are immutable references (borrows), one cannot create a mutable reference
    // this is till the last usage of immutable references i.e. if they are used no more in code, mutable references can be created.
    // eg:
    //
    // let mut s = String::from("hello");
    //
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // println!("{} and {}", r1, r2);
    // // r1 and r2 are no longer used after this point
    //
    // let r3 = &mut s; // no problem
    // println!("{}", r3);
    // --------------------------------------------------------------------------------
    
    s.len()
}

fn calculate_length(s : String) -> (String, usize) {
    let length = s.len();
    (s, length) // note: (s, s.len()) won't work because first reference to s moves it for returning. 
}

fn takes_ownership(s: String) {
    println!("after taking ownership: {}", s);
}

fn get_string() -> String {
    String::from("hello from get_string")
}

fn five() -> u8 {
    5
}

fn take_num(n: u8) {
    println!("got {}", n);
}
