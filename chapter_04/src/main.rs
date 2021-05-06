fn main() {
    println!("\nchapter 4\n");

    // playing aorund with strings
    let mut s = String::from("hello");
    println!("value of \"s\": {}", s);
    s.push_str(", world!");
    println!("value of \"s\" after \"push_str\": {}\n", s);

    // now moving the s string to another function
    let s = String::from("hello");
    takes_ownership(s);
    println!("trying to use the \"s\" after the ownership has been moved");
    // println!("value of \"s\": {}", s); // doesn't compile
    println!("does not compile!\n");

    // passing the reference of a string
    let s = String::from("hello");
    let s_len = takes_reference(&s);
    println!("trying to use the \"s\" after passing the reference");
    println!("works! length of {} is {}\n", s, s_len);

    // working with mutable references
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        //s.push_str("original_ref"); // does not compile
        r1.push_str("r1_ref");
    } // r1 goes out of scope here, so we can make a new
      // mutrable reference with no problem
    println!("printing s: {}\n", s);
    let _r2 = &mut s;
    //let r3 = &mut s; // this won't compile
    //println!("{}, {}", r2, r3);

    // working with immutable references
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    let r4 = &s;
    let r5 = &s;
    //s.push_str(" World!"); // this won't work
    println!("{}, {}, {}, {}, {}", r1, r2, r3, r4, r5);
    s.push_str(" World!"); // this does work
}

// this function takes the ownership of a string
fn takes_ownership(_example_string: String) {
    println!("ownership has been moved");
}

// this function takes the reference of a string, not the ownership
fn takes_reference(s: &String) -> usize {
    s.len()
}
