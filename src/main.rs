// scope is an important concept, as it allows us to keep memory clean without a garbage collector running, and also without having 
// to manually free memory. what we do have to do is control scope and manage where variables ae in scope or not.

fn string_literals() {
    // the following string literal would be in scope throughout the whole function, as it's declared within the corresponding braces.
    // it is valid so long as it is in scope.
    let s = "hello";

    {
        // we'll create an ns variable that is valid and in scope within these braces
        let ns = "goodbye";

        // this line compiles and runs with no issues, because ns is a valid variable in this block    
        println!("{ns}");

        // after the closing brace below, ns is destroyed, and the corresponding memory location is freed up for use
    }

    // this line compiles and runs with no issues as well, because s is a valid variable throughout the string_literal function
    println!("{s}");

    // the line below. however, results in a compiler error stating "cannot find value `ns` in this scope" because we're outside the
    // block it's valid in
    // println!("{ns}");
}

fn mutable_strings() {
    // since a string literal as above is immutable, we need another type we can use when we want to work with a string more flexibly
    // enter the String type. This is going to be of unknown size and can't be easily added to the stack.

    // we can create a String by calling the ::from function on the String type and passing the string literal "hello string"
    let mut s = String::from("hello string");

    // since this is a mutable type, we can make changes to s, in this case using push_str() to append a literal to the String s
    s.push_str(", from world!");

    println!("{}", s); // println!("{s}") would also work here to print the same thing

    // for types that do not quickly assign to the stack (such as our mutable and variable length String), when we bind an existing
    // variable to a new variable, the original variable will be dropped to avoid a double free error. simple types of fixed length 
    // are easier to copy to the stack, so the variable will be replicated with less cost.
    // see https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
    let s1 = s;

    // trying to print s here will result in a compiler error stating "value borrowed here after move"
    //println!("{s}");

    // but we can print s1
    println!("{s1}");

    // if we wanted to retain both s and s1, we could write let s1 = s.clone(), but there will be a performance cost
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn give_ownership(some_string: String) -> String {
    println!("{}", some_string);
    some_string // since there is no semicolon (;) this is an expression and returns a value
}

fn main() {
    // using string literal function to cover the basics of scope blocks
    string_literals();

    // once we've called the function and something has been returned, the string s that resides inside the function is no longer
    // in scope and isn't valid. Rust calls the drop function on s and returns the memory to the allocator. we now can't access s.
    mutable_strings();

    // if we try to access s we'll get an error
    //println!("{s}");

    // when we pass a value to a function, ownership passes in much the same way as when we bind the variable to a new name, at least
    // for types that don't implement Copy
    let s1 = String::from("banana");

    take_ownership(s1); // the function takes ownership of s1 here, and s1 is no longer valid in scope

    // this print leads to a compiler error because s1 has been dropped when its value was passed to the function.
    // we'd have to pass s1.clone() for this print to work.
    //println!("{s1}");

    // if we wanted to retain a value for later use, we could return it from a function and assign that to a variable
    let s2 = String::from("apple");

    let s3 = give_ownership(s2);

    println!("{s3}");
}


// the above method of passing data around and having to return from a function and bind to a new variable is quite tedious
// so there is a further method of transferring ownership that uses references and the concept of borrowing