fn main() {
    // VARIABLES
    // Variables are immutable by default
    let y = 6;
    println!("the value of x is: {}", y);
    // This is not allowed with immutable variables
    // y = 9;

    // mut keyword makes a variable mutable
    let mut x = 5;
    println!("the value of x is: {}", x);
    x = 6;
    println!("the value of x is: {}", x);
    // Shadowing is declaring a new variable with the same name
    // While assigning is a new value
    let x = x + 2;
    println!("the value of x is: {}", x);
    // Mutating a mutable variable's type is not allowed
    // x = "hello";
    // Shadowing a mutable variable's to a different type is allowed
    let x = "hello";
    println!("the value of x is: {}", x);

    // Constants are always immutable and must be type annotated
    const MAX: u32 = 100_000;
    // This is not allowed
    // const MAX = 345;
    println!("the value of MAX is: {}", MAX);

    // TYPES
    // Types are inferred, but can also be defined explicitly when needed

    // Integer default to i32 (signed integer)
    let mut default_integer = 56;
    default_integer = -45;
    let mut defined_integer: u64 = 76;
    // This is not allowed, unsigned integer cannot be negative
    // defined_integer = -87;

    // Floating point number defaults to f64
    let default_float = 2.0;

    // Character type use single quotes
    // They can also be emojis!
    let char_var = 'k';
    let char_var_emoji = '😵';

    // Tuples have fixed length and get group several values with different types as one variable
    let tup: (char, i32, f64) = ('😵', 56, 4.7);
    // You can destructure tuples through pattern matching to access individual values
    let (x, y, z) = tup;
    println!("the emoji in this list is {}", x);
    // You can also access value through dot notation
    let tup_float = tup.2;
    println!("the float in this list is {}", tup_float);

    // Arrays have fixed lengths, and members must all have the same types
    let arr = [1, 23, 67];
    // Typed with [type; length]
    let months: [&str; 3] = ["January", "February", "March"];
    // Can be initialized with [initial_value; length]
    let init_arr = [0; 3];
    // [0, 0, 0]
    // You can access members of the array with indexing
    println!("The second member of init_arr is {}", init_arr[1]);
    // The Book says accessing out of bound index compiles but gets a runtime error
    // But when I do cargo run with this it does not compile
    // println!("The fifth member of init_arr is {}", init_arr[4]);
}
