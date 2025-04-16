// use std::env;

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     println!("{:?}", args);
// }

// use std::env;

// fn main() {
//     let name = env::args().skip(1).next();
//     match name {
//         Some(n) => println!("Hello, {}", n),
//         None => panic!("Hello, Anon, didnt catch your name!"),
//     }
// }


/**
 * 
 * 
 * 

# What "Inlined at Compile Time" Means in Rust

When I say that `const` values are "inlined at compile time," I'm referring to how the Rust compiler handles constants compared to regular variables or `static` values.

## Inlining Explained

When you declare a constant like:

```rust
const MAX_POINTS: u32 = 100_000;

fn main() {
    println!("The maximum points are {}", MAX_POINTS);
}
```

The compiler doesn't create a variable in memory to store this value. Instead, it directly substitutes (or "inlines") the literal value `100_000` everywhere `MAX_POINTS` is used. It's as if you had written:

```rust
fn main() {
    println!("The maximum points are {}", 100_000);
}
```

## Practical implications

1. **No memory allocation**: `const` values don't take up space in the program's memory at runtime
2. **No memory lookups**: The program doesn't need to look up the value when it's used
3. **Potential for further optimizations**: The compiler can optimize code better when it knows the exact value

## Compared to `static`

In contrast, a `static` variable:
- Has a fixed memory address
- Exists as an actual value in memory
- Requires a memory lookup when used
- Can be mutable (with `static mut`, though this is unsafe)

This is why `const` is generally preferred when you just need a fixed value that won't change, while `static` is used when you specifically need a value with a fixed memory location.

 * 
 * 
 */


// fn main() {
//     let pre = "Hello, ";
//     let mut post = "world!";
//     println!("{} {}", pre, post);
    
//     pre = "Good, ";
    
//     post = "morning!";
    
//     println!("{} {}", pre, post);
//     }

// Functions 

// fn factorial(num: u64) -> u64 {
//     let mut result: u64 = 1;
//     if num == 0 {
//         println!("Base case reached");
//         return 1;
//     }
    
//     result = num * factorial(num - 1);
//     println!("{}", result);
//     return result;
// }

// fn main() {

//     println!("this is the answer -> {}", factorial(5));
    
// }



// Closures 
// use std::env;
// fn main () {
//     let num = env::args().skip(1).next();
//     let mut x = 0;

//     match num {
//         Some(n) => {
//             match n.parse::<u64>() {
//                 Ok(parsed_num) => x = parsed_num,
//                 Err(_) => println!("Failed to parse '{}' as a number", n),
//             }
//         },
//         None => println!("No number provided"),
//     }
    
//     let squared = |num: u64| num * num;
//     println!("{}", squared(x));

// }

// fn main() {
//     let money_doubler = |deposit: u64| deposit * 2;
//     println!("{}", money_doubler(10));
// }

// I ran the command using {make run args="5"}   


// STRINGS
// fn main() {
//     let q = "Hello, world!";
//     let que: &str = "Hello, world!";
//     let mut s = String::from("Hello, world!");
//     s.push_str(", world!");
//     println!("{}", s);
//     println!("{}", que);
//     println!("{}", q);  

// }
    





