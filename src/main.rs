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

#[test]
fn test_print_format() {
    let name = "john doe";
/*  println!(name); will cause an error: "format argument must be a string literal" */
    println!("{}", name);
}


#[test]
fn test_define_integer() {
    let u8_min: u8 = u8::MIN;
    let u8_max: u8 = u8::MAX;
    println!("u8_min is {}", u8_min);
    println!("u8_max is {}", u8_max);

    let i8_min = i8::MIN;
    let i8_max = i8::MAX;
    println!("i8_min is {}", i8_min);
    println!("i8_max is {}", i8_max);

/*  rust will automatically detect for integer (i32) */
    let x: i32 = 100;
    let y = 100;

    println!("x is {}", x);
    println!("y is {}", y);

/*  below statements will cause an error: "attempt to add with overflow"
    because rust prevents it from overflow

    let mut x = i32::MAX;
    x = x + 1;
    println!("x is {}", x); */

    /* below statements also will cause an error: "attempt to subtract with overflow"

    let mut y = i32::MIN;
    y -= 1;
    println!("y is {}", y); */
}

#[test]
fn test_define_float() {
    let weight: f32 = 987.1234;
    println!("{}", weight);

    let pi: f64 = 3.141592653589793238462643383279502884197;
    println!("{}", pi);

/*  rust will automatically detect for float (f64) */
    let x: f64 = 9.18279172912;
    let y = 10.21982912;

    println!("x is {}", x);
    println!("y is {}", y);
}

#[test]
fn test_define_bool() {
    let am_i_human = true;
    println!("am i human? {}", am_i_human);

    let am_i_tired: bool = false;
    println!("am i tired? {}", am_i_tired);
}

#[test]
fn test_define_char() {
    let new_char: char = 'a';
    println!("{}", new_char);
}

#[test]
fn test_define_string() {
    let name = "jack morris";
    println!("my name is {}", name);

    let city: &str = "jakarta";
    println!("i lived in {}", city);
}

#[test]
fn test_copy_variable() {
    let a: i8 = 10;
    let b: i8 = a;

    println!("a is {}", a);
    println!("b is {}", b);
}

#[test]
fn test_convert_integer_using_as() {
    let a: i8 = 100;
    let b: i16 = a as i16;
    let c: i32 = a as i32;

    println!("{}", a);
    println!("{}", b);
    println!("{}", c);

    let d: i32 = 1000000128;
    let e: i8 = d as i8;

    println!("{}", d);
    println!("{}", e);
}

#[test]
fn test_define_tuple() {
let tpl: (i8, bool, char) = (8, true, 'x');
println!("1st index is {}", tpl.0);
println!("2nd index is {}", tpl.1);
println!("3rd index is {}", tpl.2);
}

#[test]
fn test_operator() {
    let a: i8 = 10;
    let b: i8 = 5;

    println!("{}", a + b);
    println!("{}", a - b);
    println!("{}", a / b);
    println!("{}", a * b);
    println!("{}", a % b);
}

#[test]
fn test_augmented_operator() {
    let mut a: i8 = 10;

/*  when using augmented operator, make sure that the variable has been defined as 'mut'. */
    a += 10;
    println!("{}", a);

    a -= 10;
    println!("{}", a);
}