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

#[test]
fn test_check_variable() {
    let name = "john doe";
/*  name = 10; will cause an error: "mismatched types" */
    println!("this is {}", name);

    /* let mut occupation = "software engineer";
    println!("his job is a {}", occupation);

    piece of code above will give you only warning: "variable does not need to be mutable"
    but it succeeds when running unit tests */
}

#[test]
fn test_shadowing() {
    let id = "abc12345";
    println!("{}", id);

    let id = 12345;
    println!("{}", id);

/*  12345 will be the latest value and abc12345 will be closed */
    println!("{}", id);

/*  rust allows users to use shadowing variable but it is not a good practice
    and causing confusion */
}