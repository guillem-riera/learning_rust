# Owning and Borrowing

Are concepts for safe memory management

## Owning

is the concept of the ownership over a memory reference (a value), normally indicated by a variable.



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
// but this time we use a reference of a staticat lifetime str, which is guaranteed to be valid for the duration of the entire program
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
    String
    String



```Rust
// Borrowing mutable String
// if we repeat the same steps but instead use a String that is mutable ...
let mut s = String::from("String");
println!("{}", s);
// step 2: we point a new variable to be the new owner of that string literal, 
// but this time we use a reference of a staticat lifetime str, which is guaranteed to be valid for the duration of the entire program
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



```Rust

```
