fn main() {
    println!("Hello, world!");

    // 1. variables
    //    - by default, variables are immutable
    //    - use 'mut' keyword to make a variable mutable
    //    - variable names are snake_case
    //    - variables must be declared before use
    //    - variables can be shadowed by declaring a new variable with the same name

    let x = 5;
    println!("The value of x is: {}", x);

    // 2. constants
    //    - constants are always immutable
    //    - must be declared with 'const' keyword
    //    - must have a type annotation
    //    - conventionally, constants are named in all uppercase with underscores
    //    - can be declared in any scope, including the global scope
    //    - cannot be assigned to the result of a function call or any value that is computed at runtime
    //    - cannot be muteted with "mut" keyword
    //    - constants can only be set to a constant expression, not the result of a function call or any other value that could only be computed at runtime
    const MAX_POINTS: u32 = 100_000;
    println!("The maximum points are: {}", MAX_POINTS);

    // 3. shadowing
    //   - shadowing allows you to declare a new variable with the same name as a a previous variable
    //   - the new variable shadows the previous variable
    let y = 5;
    let y = y + 1; // shadows previous y
    {
        let y = y * 2; // shadows previous y in this scope
        println!("The value of y in the inner scope is: {}", y);
    }
    println!("The value of y is: {}", y);

    // 4. data types
    //    - Rust is a statically typed language, which means that it must know the
    //      types of all variables at compile time
    //    - Rust can usually infer the type of a variable based on the value assigned to it
    //    - if you want to specify the type of a variable, you can do so by adding a type annotation
    //    - types of datatype :
    //       - scalar types
    //       - compound types
    //    - common scalar types:
    //          - integers ==> (rust defaults to signed 32-bit integers [i32])
    //               - types (dependant on sign):
    //                                 bits :  8,  16,  32,  64,  128, architecture dependent
    //                  - signed integers :   i8, i16, i32, i64, i128, isize
    //                  - unsigned integers : u8, u16, u32, u64, u128, usize
    //
    //               - types (dependant on representation) :
    //                  - decimal : 98_222
    //                  - hexadecimal : 0xff
    //                  - octal : 0o77
    //                  - binary : 0b1111_0000
    //                  - byte (u8 only) : b'A'

    //               - incase of integer overflow in debug mode, Rust will panic
    //                 in release mode, Rust will wrap around using two's complement wrapping
    let guess: u32 = "42".parse().expect("Not a number");
    println!("The guessed number is: {}", guess);

    //          - floating-point numbers : (default to 64-bit floating point [f64])
    //               - types : f32, f64
    //               - numeric operations : addition (+), subtraction (-), multiplication (*), division (/), remainder (%)

    //          - booleans : (1 byte in size)
    //               - values : true, false

    //          - characters : (4 bytes in size, represents a Unicode Scalar Value)
    //               - represented with single quotes : 'a', 'α', '∞', '😻'

    //    - common compound types:
    //            - tuples ( fixed length, can contain different types )
    //              - accessed via dot notation and index : tuple.0, tuple.1
    //              - destructuring : let (x, y, z) = tuple;

    //            - arrays ( fixed length, must contain same type )
    //              - accessed via index : array[0], array[1]
    //              - arrays have a fixed length, cannot grow or shrink in size
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // to avoid the warning of unused variables, we can use underscore before the variable name
    let (_a, b, _c) = tup;
    println!("The value of b is: {}", b);

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The first element of the array is: {}", arr[0]);

    // 5. functions
    //    - functions are declared using the 'fn' keyword
    //    - function names are snake_case
    //    - parameters must have type annotations
    //    - return type is specified after '->'
    //    - last value can be return implecitly without 'return' keyword
    let result = add(5, 10);
    println!("The result of the addition is: {}", result);

    // 6. conditional statements
    //    - use 'if', 'else if', and 'else' keywords
    let condition: bool = true;
    let _val: i32 = if condition { 5 } else { 6 };

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    //  7. loops
    //     - use 'loop', 'while', and 'for' keywords
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2; // break with a value
        }
    };
    println!("The result from the loop is: {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // range of numbers
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    /*
        8. comments
           - single line comments start with '//'
           - multi-line comments (block comments) are enclosed in '/*' and '*/'
    */
}

// function declaration
// - use 'fn' keyword
// - function names are snake_case
// - parameters must have type annotations
// - return type is specified after '->'
// - the body of the function is enclosed in curly braces

fn add(x: i32, y: i32) -> i32 {
    x + y // expression without semicolon returns the value
}
