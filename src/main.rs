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
/*  name = "jack napalm"; will cause an error: "cannot assign twice to immutable variable 'name'". */
    println!("my name is {}", name);

/*  rust allows user to declare variable first, then initialized. */
    let city: &str;
    city = "jakarta";
    println!("{} lives in {}", name, city);
}

#[test]
fn test_create_mutable_variable() {
    let mut x = 10;
/*  let x = 10;
    x = 100; will produce an error
    ever since variables in rust are immutable.

    to explicitely reassign value to the x variable,
    user can simply add 'mut' after let reserved word. */

    println!("x is {}", x);
    x = 100;
    println!("x is {}", x);
}

#[test]
fn test_check_variable() {
    let name = "john doe";
/*  name = 10; will cause an error: "mismatched types". */
    println!("this is {}", name);

/*  let mut occupation = "software engineer";
    println!("his job is a {}", occupation);

    piece of codes above will give user only warning: "variable does not need to be mutable"
    but it succeeds when running unit tests. */
}

#[test]
fn test_shadowing() {
    let id = "abc12345";
    println!("{}", id);

    let id = 12345;
    println!("{}", id);

/*  12345 will be the latest value and abc12345 will be closed. */
    println!("{}", id);

/*  rust allows users to use shadowing variable but it is not a good practice
    and causing confusion. */
}

#[test]
fn test_print_format() {
    let name = "john doe";
/*  println!(name); will cause an error: "format argument must be a string literal". */
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

/*  rust will automatically detect for integer (i32). */
    let x: i32 = 100;
    let y = 100;

    println!("x is {}", x);
    println!("y is {}", y);

/*  below statements will cause an error: "attempt to add with overflow"
    because rust prevents it from overflow.

    let mut x = i32::MAX;
    x = x + 1;
    println!("x is {}", x); */

/*  below statements also will cause an error: "attempt to subtract with overflow".

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

/*  rust will automatically detect for float (f64). */
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
/*  defining char data type is the same as the other languages
    by using single-quote character. */
    let new_char: char = 'a';
    println!("this is \'{}\' character", new_char);
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
    let tuple: (i8, bool, char) = (8, true, 'x');

/*  accessing tuple value by using tuple.index, e.g: tuple.0. */
    println!("1st index is {}", tuple.0);
    println!("2nd index is {}", tuple.1);
    println!("3rd index is {}", tuple.2);
    println!("the tuple values are {:?}", tuple);

    let another_tuple: (i32, f64, bool) = (10000, 90.98079, false);

/*  this is part of destructuring tuple. */
    let (a, b, c) = another_tuple;

    println!("{:?}", another_tuple);
    println!("{} {} {}", a, b ,c);

/*  skip the value by using _ (underscore). */
    let (d, e, _) = another_tuple;
    println!("{} {}", d, e);

/*  every variable declares in rust is immutable by default.
    to make the variable becomes mutable, use 'mut'. */
    let mut yet_stil_another_tuple: (bool, char, f64) = (false, 'a', 10.9998);
    yet_stil_another_tuple.0 = true;

    println!("{:?}", yet_stil_another_tuple);
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

#[test]
fn test_comparison_operator() {
    let a: u16 = 60000;
    let b: i16 = 32000;

/*  println!("is a bigger than b? {}", a > b); will cause an error: "mismatched types, expected 'u16'
    found 'i16'". */

    let c: u16 = b as u16;
    
    println!("is {} bigger than {}? {}", a, c, a > c);
    println!("is {} less than {}? {}", a, c, a < c);
    println!("is {} bigger or equal than {}? {}", a, c, a >= c);
    println!("is {} less or equal than {}? {}", a, c, a <= c);
    println!("is {} equal than {}? {}", a, c, a == c);
    println!("is {} not equal than {}? {}", a, c, a != c);
}

#[test]
fn test_boolean_operator() {
    let quiz_score: i8 = 90;
    let eval_score: i8 = 80;
    let am_i_passed: bool = (quiz_score >= 75 && quiz_score <= 100)
                                && (eval_score >= 75 && eval_score <= 100);
    println!("am i passed the test and eval? {}", am_i_passed);

    let is_remedial_test_eligible: bool = !am_i_passed;
    println!("should i take the remedial test? {}", is_remedial_test_eligible);
}

#[test]
fn test_array() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array value is {:?}", arr);

/*  accessing array value by using [index], e.g: arr[0]. */
    println!("index 0 is {}", arr[0]);
    println!("index 1 is {}", arr[1]);
    println!("index 2 is {}", arr[2]);
    println!("index 3 is {}", arr[3]);
    println!("index 4 is {}", arr[4]);

/*  defining multi-dimensional array with [[type; column]; row], e.g: [[i32; 4]; 2]. */
    let arr_2: [[i32; 4]; 2] = [
        [1, 2, 5, 11],
        [3, 4, 5, 9]
    ];

    println!("{:?}", arr_2);
    println!("row 1 and column 3: {}", arr_2[0][2]);
}

#[test]
fn test_mutable_array() {
    let mut array: [u8; 5] = [0, 1, 2, 4, 90];
    array[0] = 100;

    println!("{:?}", array);
    println!("{:?}", array[2]);
}

/*  declaring const in rust, user simply adds 'const'.
    const cannot be declared using 'mut' because of the immutability by default.
    it should be declared in compiled-time and declared the data type explicitly.
    naming convention in const generally uses SCREAM_SNAKE_CASE.
    
    as it's a global const variable, every function inside of main.rs can invoke this. */
#[allow(unused)]
const MAXIMUM_I32: i32 = i32::MAX;

#[test]
fn test_define_const() {
    const MINIMUM_I32: i32 = i32::MIN;
    println!("maximum range for i32 is {}", MAXIMUM_I32);
    println!("minimum range for i32 is {}", MINIMUM_I32);

    const PI: f64 = 3.141592653589793238462643383279502884197;
/*  PI = 39.10; will cause an error: "invalid left-hand side of assignment",
    since const is immutable. */
    println!("pi is {}", PI);
}

#[test]
fn test_variable_scope() {
/*  pet_name is declared in function scope. */
    let pet_name: &str = "marble";

    {
/*      place_of_birth is also declared inside of the inner scope.
        its scope is accessible only inside of this scope
        and its inner scope. */
        let place_of_birth: &str = "jakarta";

        {
/*          but pet_name can be accessible where it is declared
            and inside of the inner scope. */
            println!("my pet name is {}", pet_name);
        }

        println!("i hardly remember that he was born in {}", place_of_birth);
    }
    
/*  when the program hits the outer of the scope, place_of_birth will pop up
    from the memory and rust automatically frees them.
    
    that's why when user invokes place_of_birth out of its scope,
    it causes an error.
    
    whereas, pet_name is still accessible in the function scope until program ends. */
    println!("repeat! my pet name is still {}", pet_name);

/*  println!("oh wait... yes! he was really born in {}", place_of_birth);
    will cause an error that the variable is out of scope where it is declared. */
}

#[test]
fn test_outer_scope() {
/*  println!("{}", pet_name); will cause an error: "cannot find value 'pet_name' in this scope". */
    let occupation: &str = "accountant";
    println!("i am working as an {}", occupation);
}

#[test]
fn test_shadowing_scope() {
    let nation: &str = "england";
    let age: i8 = 17;

    {
        let nation: &str = "germany";
        let age: i8 = age - 5;
        println!("i lived in {}", nation);
        println!("i am {} years-old", age);
    }

    println!("i was born in {}", nation);
    println!("now i am {} years-old", age);
}

#[test]
fn test_stack_and_heap_memory() {
    call_first_stack();
    call_second_stack();
}

#[allow(unused)]
fn call_first_stack() {
    let num: i8 = 99;
    let str: String = String::from("hello");

    println!("{} and {}", num, str);
}

#[allow(unused)]
fn call_second_stack() {
    let num: i8 = 99;
    let str: String = String::from("world");
    println!("{} and {}", num, str);
}

#[test]
fn test_overflow() {
/*  for integer, rust will set i32 for integer type by default. */
    let a = 99;
    println!("integer value is {}", a);

/*  for float, rust will set f64 for float type by default. */
    let mut b = 10.5;
    println!("float value is {}", b);
    
/*  a = 2147483647 + 1; is error at compile-time and will cause an overflow. */
    println!("integer max plus one is {}", a);

/*  b = f64::MAX + 1; will cause an error: "cannot add '{integer}' to 'f64'".
    consider using a floating-point literal by writing it with '.0'.
    
    b = f64::MAX + 1.0; is not error at runtime but will cause an overflow. */
    b = f64::MAX + 1.0;
    println!("float max plus one is {}", b);
}

#[test]
fn isize_and_usize() {
    let a: isize = -100;
    let b: usize = 100;

    println!("signed value is {}", a);
    println!("unsigned value is {}", b);
}