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