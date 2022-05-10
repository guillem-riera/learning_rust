# Rust interesting stuff

## 'Null' absence, Optionals, Some and None

Rust has no 'null' value by design (think about the memory safety, null pointers, etc. errors that plague C/C++ programs and all the error handling that it incurrs accessing memory collections).

Instead, Rust embraces optionals and provides `Some` and `None` functions to handle elegant access to optionals.

In the following examples we will access a collection of characters out of a string and access some members through the `nth` collection function, which wraps the content type into an `Optional` value.


```Rust
let phrase = String::from("Rust has no 'null' values");
let first = phrase.chars().nth(0);
// unwrapping an optional can panic
println!("{}", first.unwrap());
```

    R



```Rust
// showcasing a wrong unwrap:
let last = phrase.chars().nth(100);
// unwrapping an optional can panic
println!("{}", last.unwrap());
```

    thread '<unnamed>' panicked at 'called `Option::unwrap()` on a `None` value', src/lib.rs:112:21
    stack backtrace:
       0: _rust_begin_unwind
       1: core::panicking::panic_fmt
       2: core::panicking::panic
       3: _run_user_code_2
       4: evcxr::runtime::Runtime::run_loop
       5: evcxr::runtime::runtime_hook
       6: evcxr_jupyter::main
    note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.



    Child process terminated with status: exit status: 101



```Rust
let phrase = String::from("Rust:Some/None");
let positions: Vec<u8> = (0u8..23).collect();
for p in positions {
    let optional_character = phrase.chars().nth(p.into());
    match optional_character {
        Some(character) => println!("Char: {} in Position: {}", character, p),
        None => println!("No Char in Position: {}", p)
    }
}

```

    Char: R in Position: 0
    Char: u in Position: 1
    Char: s in Position: 2
    Char: t in Position: 3
    Char: : in Position: 4
    Char: S in Position: 5





    ()



    Char: o in Position: 6
    Char: m in Position: 7
    Char: e in Position: 8
    Char: / in Position: 9
    Char: N in Position: 10
    Char: o in Position: 11
    Char: n in Position: 12
    Char: e in Position: 13
    No Char in Position: 14
    No Char in Position: 15
    No Char in Position: 16
    No Char in Position: 17
    No Char in Position: 18
    No Char in Position: 19
    No Char in Position: 20
    No Char in Position: 21
    No Char in Position: 22


### Alternative: Unwrapping from Optional to Result


```Rust
// In this variation we can get a Result out of an Optional value.
// It is the same, but semantically different: we "speak" about an 'OK' match, instead of 'Some' or 'None'
let phrase = String::from("Rust:Optional/Result");
let positions: Vec<u8> = (0u8..23).collect();
for p in positions {
    let optional_character = phrase.chars().nth(p as usize);
    // use .ok_or from an Optional to get a result
    match optional_character.ok_or("item not found") {
        Ok(character) => println!("Char: {} in Position: {}", character, p),
        _notOk => println!("No Char in Position: {}", p)
    }
}
```




    ()



    Char: R in Position: 0
    Char: u in Position: 1
    Char: s in Position: 2
    Char: t in Position: 3
    Char: : in Position: 4
    Char: O in Position: 5
    Char: p in Position: 6
    Char: t in Position: 7
    Char: i in Position: 8
    Char: o in Position: 9
    Char: n in Position: 10
    Char: a in Position: 11
    Char: l in Position: 12
    Char: / in Position: 13
    Char: R in Position: 14
    Char: e in Position: 15
    Char: s in Position: 16
    Char: u in Position: 17
    Char: l in Position: 18
    Char: t in Position: 19
    No Char in Position: 20
    No Char in Position: 21
    No Char in Position: 22


## match, if and if let

Match is a robust switch statement, and `if let` is a short circuit conditional assignation and code statement execution.

See this samples to compare the usage in conditionals:


```Rust
fn do_something_for_element_10_in_a_non_idiomatic_way(v: Vec<u8>) {
    for e in v {
        if e == 10 {
            println!("{}", e);
        }
    }
}

fn do_something_for_if_match_10(v: Vec<u8>) {
    for e in v {
        match e {
            10 => println!("{}", e),
            _ => ()
        }
    }
}


fn do_something_if_let_10(v: Vec<u8>) {
    for e in v {
        if let 10 = e {
            println!("{}", e);
        }
    }
}



let elements: Vec<u8> = (0..255).collect();

do_something_for_element_10_in_a_non_idiomatic_way(elements);

let elements: Vec<u8> = (0..255).collect();

do_something_for_if_match_10(elements);

let elements: Vec<u8> = (0..255).collect();

do_something_if_let_10(elements);


```

    10
    10
    10


## Strings

Rust has 2 different concepts for strings:
- `string literals`, which are not mutable and can be allocated on the *stack* because the size is known by the compiler.
- `Strings` (capitalized), which *may* mutate (they might still require the *mut* keyword) and that are of unknown size, therefore allocated on the *heap*. 



```Rust
let string_literal_example = "immutable string";
println!("{}", string_literal_example);
```

    immutable string



```Rust
let mut mutable_string_with_variable_size = String::from("some value");
mutable_string_with_variable_size += " which can grow and change size";
mutable_string_with_variable_size += " and therefore will be allocated in the heap.";
println!("{}", mutable_string_with_variable_size);
```

    some value which can grow and change size and therefore will be allocated in the heap.


## Variable Scope (and lifecycle)

Variables are scoped in Rust and their lifecycle is tied to their declaration.

`{}` define scopes (even without the function keyword).

When a variable goes out of scope (right after the closing curly brace), Rust calls a special function for us: `drop`.



```Rust
{
    let mut s = String::from("hello"); // s is valid from this point forward and is allocated in the heap because
    // its size is not known at all times by the compiler.
} // this scope is now over, and s is no longer valid. Rust calls `drop` at this point for us.
// this means that the memory will be freed (there are no more owners).
```
