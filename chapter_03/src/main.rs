// testing consts
const PI: f64 = 3.0; // this is obv. wrong

// function is calling functions repesenting the subchapters
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
    println!("Chapter 3.3: functions\n");
    println!("calling function \"five()\": {}", five());
    println!("calling function \"plus_one(22)\": {}", plus_one(22));
    println!("----------");

    // 3.5
    println!("Chapter 3.5: control flow\n");
    println!("calling function \"testing_if()\": {}", testing_if());
    println!("calling function \"testing_match()\": {}", testing_match());
    println!("calling function \"testing_for()\"");
    testing_for();
    println!("----------");
}

// function for chapter 3.1
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

// function for chapter 3.3: functions
fn five() -> i32 {
    5
}

// function for chapter 3.3: functions
fn plus_one(x: i32) -> i32 {
    x + 1
}

// functon for chapter 3.5: control flow
fn testing_if() -> i32 {
    let condition = true;
    let number = if condition { 5 } else { 10 };
    number
}

// function for chapter 3.5: control flow
fn testing_match() -> i32 {
    let condition = true;
    match condition {
        true => 10,
        false => 20,
    }
}

// function for chapter 3.5: control flow
fn testing_for() {
    let a = [10, 20, 30, 40, 50];
    for number in a.iter() {
        println!("{}", number);
    }
}
