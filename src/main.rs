fn main() {
    println!("Hello, world!");
}

#[test]
fn test_say_hi() {
    print!("hello ");
    print!("world! ");
    println!("welcome to the rust world!");
}

#[test]
fn test_create_variable() {
    let name = "john doe";
/*  name = "jack napalm"; will cause an error: "cannot assign twice to immutable variable 'name'" */
    println!("my name is {}", name);
}

#[test]
fn test_create_mutable_variable() {
    let mut x = 10;
/*  let x = 10;
    x = 100; will produce an error
    ever since variables in rust are immutable.

    to explicitely reassign value to the x variable,
    we simply add 'mut' after let reserved word. */

    println!("x is {}", x);
    x = 100;
    println!("x is {}", x);
}