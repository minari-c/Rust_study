use std::io;

fn main() {
    /* A simple loop, annotation same as c, c++ */
    let hi = "hi";          // let it's local variable but, same as constant
    let mut count = 0;       // it's added mut keyword, can mutable
    
    
    println!("Guess the number!");
    
    println!("Please input your guess.");
    
    /*
    double colon: scope?
    -> type::associate function
    associate function:
    not for instance, only for type.
    Same as static method
    */
    let mut guess = String::new();
    // String: growable UTF-8 encoding string type
    // new is associate function
    
    io::stdin().read_line(&mut guess).
        expect("Failed to read line");
    // stdin function: handle instance for stdin terminal
    // read_line method: one growable string argument to read and write stdin data
    //  return: io::Result Type value
    //      ( enumerations ): variants[ Ok, Err ]
    //          Ok: include stdin length
    //          Err: if call expect method then program is stop
    // expect(): .foo() format syntax in to newline
    
    println!("You guessed: {}", guess);
    
    let x = 5;
    let y = 10;
    println!("x = {} and y = {}", x, y);
    
    
    // loop {}
}

/*
 in Rust symbol
 use
 let, mut
 match
 method
 associated functions
 external crates

 prelude:
    Express program or external crates
 crate:
    binary source files
 
 variable, constant
 
 data type
 
 
 */


