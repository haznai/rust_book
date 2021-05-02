const PI: f64 = 3.0; // this is obv. wrong

// main function is calling functions repesenting the subchapters
fn main() {
    // 3.1
    println!("Chapter 3.1: variables and mutability\n");
    variables();
    println!("----------");

    // 3.2
    println!("Chapter 3.2: data types\n");
    data_types();
    println!("----------");

    // 3.3
    println!("Chapter 3.3: data types\n");

    println!("----------");

    // function for chapter 3.1: variables and mutability
}
fn variables() {
    // testing how immutable variables work
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // testing how consts work
    println!("this is the constant \"PI\": {}", PI);

    // testing how shadowing works
    let y = 10;
    println!("unshadowed y: {}", y);
    let y = "ben".to_owned() + &y.to_string();
    println!("shadowed y: {}", y);
}

// function for chapter 3.2: data types
fn data_types() {
    // testing tuples
    let tup: (u8, f64, bool) = (1, 1.0, true);
    let (x, y, _) = tup; // pattern matching
    println!("x: {}, y: {}, z: _", x, y);

    // testing arrays
    let foo: [f32; 10] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let foo = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    println!("foo: {}", foo[9]);
}
