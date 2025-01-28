# Rust Overview (compared to TS)

## Types

Denote types for:
- function arguments
- function return types
- constants
Types can be inferred, but generally denote them.

Types include:
- Scalar
    - Integer (8, 16, 32, 64, 128) with either signed (i) or unsigned(u) eg: i32
        - There is also isize & usize which are determined by architecture of executing machine (either 32 or 64 bit)
    - Floating Point (f32, f64)
    - Boolean (bool)
        Rust DOES NOT convert non-Boolean types to a Boolean. Must be explicit
    - Character (char)
- Compound Types
    - Tuple eg: (i32, f64, u8) = (500, 6.4, 1);
        - immutable!
        - Can be of different types
        - An empty Tuple `()` is called a unit type. It signifies an empty value or an empty return type.
        - access with dot notation
        - let x: (i32, f64, u8) = (500, 6.4, 1)
    - Array
        - immutable
        - All elements must be the same type
        - type defined as [type; size] eg  let arr: [i32; 5] = [1, 2, 3, 4, 5];
        - default values with [default_value; size] eg  let arr: [1; 5] = [1, 1, 1, 1, 1];
        - access via square bracket index
    - Vectors ?
        - Dynamic size

## Expressions vs Statements


This is an expression
```Rust
{
    let x = 3;
    x + 1
}
```

This is a statement
```Rust
{
    let x = 3;
    x + 1
};
```

The semicolon IS IMPORTANT. 

Expressions will evaluate while statements do not. 

So functions may not need a return if there is no semi colon.

## Functions

Like TS give return type after parameters but with `->`

```Rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
```

```Rust
fn plus_one(x: i32) -> i32 {
    x + 1;
}
```
This will throw an error as it would return `()` -unit type. Because of the (SEMICOLON)

## Control Flow


### If / Else
-Rust has same if / else syntax but condition must be of bool type.
```Rust
fn main() {
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
}
```


let var = if condition { value_a } else { value_b };

// GOOD
let num = if condition { 5 } else { 6 };

// BAD - types must be the same
 let number = if condition { 5 } else { "six" };

## Loops

### loop (basic)

```Rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```

### loop (named)

```Rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```
This returns 2, you can name a loop with `'counting_up:` syntax before `loop`. Notice how the loop is broken.

### While (nothing special)

```Rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

### For 

```Rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```

```Rust
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```
This uses a range expression, it does not include the last value. .rev reverses