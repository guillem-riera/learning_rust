# Owning and Borrowing

Are concepts for safe memory management

## Owning

is the concept of the ownership over a memory reference (a value), normally indicated by a variable.


## The Rules of References

At any given time, you can have either:
1. **one mutable** reference
2. OR **any number of immutable** references.
3. References must always be valid.



```Rust
// Let's create a memory string (`Strings` are stored in the heap) and print it
let s = String::from("String");
println!("{}", s);
```

    String



```Rust
// we repeat the same steps and then continue at step 2
let s = String::from("String");
println!("{}", s);
// step 2: we point a new variable to be the new owner of that String in the heap
let new_string_owner = s;
println!("{}", new_string_owner);
// and this will fail.
// =================================================================
// Check how elegantly the compiler tells us that the value "moved"
// =================================================================
println!("{}", s);
```


    let new_string_owner = s;

                           ^ value moved here

    println!("{}", s);

                   ^ value borrowed here after move

    let s = String::from("String");

        ^ move occurs because `s` has type `String`, which does not implement the `Copy` trait

    borrow of moved value: `s`


## Borrowing




```Rust
// if we repeat the same steps but instead use a str slice (string literal), which is immutable ...
let s = "String";
println!("{}", s);
// step 2: we point a new variable to be the new owner of that string literal,
// but this time we use a reference of a static lifetime str, which is guaranteed to be valid for the duration of the entire program
let new_string_owner = s;
println!("{}", new_string_owner);
// and this will just work.
// ==========================
// Why does it work now?
// ==========================
// Because the variable is not mutable (slice), and is statical
println!("{}", s);
```

    String



```Rust
// Borrowing mutable String
// if we repeat the same steps but instead use a String (that is, the size is unknown at compile time and can grow, it lives in the heap)
let mut s = String::from("String");
println!("{}", s);
// step 2: we point a new variable to be the new owner (of the mutable String)
let mut new_string_owner = s;
new_string_owner += "(Modified)";
println!("{}", new_string_owner);
// and this will just NOT work. (try for yourself uncommenting and running it)
// println!("{}", s);
// ==========================
// Why doesn't it work now?
// ==========================
// Because the variable is now mutable, String lives in the heap and it does not implement `Copy`

// If we regain ownership, it all works as expected though!
// Give me my var back!
s = new_string_owner;
println!("{}", s);

// And this again won't work (try uncommenting). 'new_string_owner' is not anymore the current owner of the String value.
// println!("{}", new_string_owner);
// If you would uncomment you can see a very descriptive explanation from the compiler.
```

    String
    String(Modified)
    String(Modified)


## Rust Compiler on borrowed and moved

Rust compiler gives us very meaningfull messages about the borrowing and moving of variables.
Reading the trace almost always suggest the root cause issue.


```Rust
// Borrowing mutable String
let mut s = String::from("String");
println!("{}", s);
let mut new_string_owner = s;
new_string_owner += "(Modified)";
println!("{}", new_string_owner);
// Give me my var back!
s = new_string_owner;
println!("{}", s);

// And this again won't work (try uncommenting). 'new_string_owner' is not anymore the current owner of the String value.
println!("{}", new_string_owner);
// If you would uncomment you can see a very descriptive explanation from the compiler.
```


    s = new_string_owner;

        ^^^^^^^^^^^^^^^^ value moved here

    println!("{}", new_string_owner);

                   ^^^^^^^^^^^^^^^^ value borrowed here after move

    let mut new_string_owner = s;

        ^^^^^^^^^^^^^^^^^^^^ move occurs because `new_string_owner` has type `String`, which does not implement the `Copy` trait

    borrow of moved value: `new_string_owner`


## Borrowing when using functions

Passing a variable to a function will move or copy, just as assignment does.
This means that if a function takes a reference to a heap element (without cloning it or copying it), does something and then quits without returning this reference back, then this element will be dropped at the end of the function call.



```Rust
// Spoiler: this code seems to work properly

// This code mutates a string
fn append_to_string_taking_ownership(mut s: String) {
    s += " value appended";
    println!("{}", s);
}

let mut s = String::from("example");

println!("{}", s);

append_to_string_taking_ownership(s);

```

    example
    example value appended


### Borrowing: trying to mutate a string by its pointer


```Rust
// Spoiler: This code forgets to return the mutated String back and it won't work as expected
// This code mutates a string
fn append_to_string_taking_ownership_wrongly(mut s: String) {
    s += " value appended";
    println!("{}", s);
}

let mut s = String::from("example");

println!("{}", s);

append_to_string_taking_ownership_wrongly(s);

println!("{}", s);
```


    append_to_string_taking_ownership_wrongly(s);

                                              ^ value moved here

    println!("{}", s);

                   ^ value borrowed here after move

    let mut s = String::from("example");

        ^^^^^ move occurs because `s` has type `String`, which does not implement the `Copy` trait

    borrow of moved value: `s`


## Iterating while borrowing


```Rust
use std::collections::HashMap;

// Declaring a mutable map
let mut map: HashMap<&str, &str> = HashMap::new();

// Fill in some values
map.insert("key1", "value1");
map.insert("key2", "value2");
map.insert("key3", "value3");

// iterate the keys and values
for (key, value) in map {
    println!("{}:{}", key, value);
}

```

    key1:value1
    key3:value3
    key2:value2





    ()



### Iterating: can't we iterate a collection multiple times?


```Rust
use std::collections::HashMap;

// If you try to iterate a HashMap two times it will result in a compiler error

let mut map: HashMap<&str, &str> = HashMap::new();

map.insert("key1", "value1");
map.insert("key2", "value2");
map.insert("key3", "value3");

// iterate the keys and values for the first time
for (key, value) in map {
    println!("{}:{}", key, value);
}

// iterate the keys and values for the second time results in an error:
for (key, value) in map {
println!("{}:{}", key, value);
}

// for loops take the value to iterate over by value
// ===============================================================
// Why?
// ===============================================================
// When we call for (key, value) in map {}, the ownership of map is transferred to the for loop and afterwards it is gone, as it does not implement Copy.
//
```


    for (key, value) in map {

                        ^^^ `map` moved due to this implicit call to `.into_iter()`

    for (key, value) in map {

                        ^^^ value used here after move

    let mut map: HashMap<&str, &str> = HashMap::new();

        ^^^^^^^ move occurs because `map` has type `HashMap<&str, &str>`, which does not implement the `Copy` trait

    use of moved value: `map`

    help: consider borrowing to avoid moving into the for loop
    
    &map


### Iterate multiple times with an 'iter' iterator


```Rust
use std::collections::HashMap;

// How avoid this?

let mut map: HashMap<&str, &str> = HashMap::new();

map.insert("key1", "value1");
map.insert("key2", "value2");
map.insert("key3", "value3");

// iterate the keys and values using an explicit type reference iterator. Note that we are not mutating the values
// otherwise this would not work
for (key, value) in map.iter() {
    println!("{}:{}", key, value);
}

// iterate the keys and values for the second time results then works:
for (key, value) in map.iter() {
    println!("{}:{}", key, value);
}

// iter() loops iterate over by reference
// ===============================================================
// Why?
// ===============================================================
// When we call for (key, value) in map.iter() {}, we use many references which are not mutable.
//
```

    key1:value1
    key2:value2
    key3:value3
    key1:value1
    key2:value2
    key3:value3





    ()



### The three forms of iteration

There are three common methods which can create iterators from a collection:

1. iter(), which iterates over &T.
2. iter_mut(), which iterates over &mut T.
3. into_iter(), which iterates over T.

4. Various things in the standard library may implement one or more of the three, where appropriate.

# Quick summary of the rules around ownership, move and copy semantics, and dropping



```Rust
let x1 = 42;
let y1 = Box::new(84);
{ // starts a new scope
    // x1's value is Copy, so it will not be moved into z
    // y1's value is NOT Copy, so it will be moved into z
    let z = (x1, y1);
    println!("{},{}", z.0, z.1);
} // z goes out of scope, and is dropped;
// it in turn drops the values from x1 and y1

// x1's value is Copy, so it was not moved into z
let x2 = x1;
println!("{}", x2);
// y1's value is not Copy, so it was moved into z
// let y2 = y1;
```

    42,84
    42



```Rust
let x1 = 42;
let y1 = Box::new(84);
{ // starts a new scope
// x1's value is Copy, so it will not be moved into z
// y1's value is NOT Copy, so it will be moved into z
let z = (x1, y1);
println!("{},{}", z.0, z.1);
} // z goes out of scope, and is dropped;
// it in turn drops the values from x1 and y1

// x1's value is Copy, so it was not moved into z
let x2 = x1;
println!("{}", x2);
// y1's value is not Copy, so it was moved into z
let y2 = y1;

```


    let z = (x1, y1);

                 ^^ value moved here

    let y2 = y1;

             ^^ value used here after move

    let y1 = Box::new(84);

        ^^ move occurs because `y1` has type `Box<i32>`, which does not implement the `Copy` trait

    use of moved value: `y1`

