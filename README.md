# Rust First Steps

Learning notes (stored in Notebooks) and sample code that can help learning Rust.

## Who? (Target Audience)

- Me (guillem.riera@intive.com, guillem@rieragalm.es)
- Rust absolute beginners
- Curious people

## What?

Sample code, Notebooks

## Why?

- Knowledge Sharing
- Experimentation
- For Fun
- ... not?

## Index

1. [Install](./00_install_rust.md) : [Install Notebook](./00_Install_Rust.ipynb)
2. [Intro to Rust](./01_intro_to_rust.md) : [Intro to Rust Notebook](./01_Intro_to_Rust.ipynb)
3. [Interesting Stuff](./02_interesting_stuff.md) : [Interesting Stuff Notebook](./02_Interesting_Stuff.ipynb)
4. [Owning and Borrowing](./03_owning_and_borrowing.md) : [Owning and Borrowing Notebook](./03_Owning_and_Borrowing.ipynb)

## What is not yet Indexed

Some other code samples do not fit in Notebooks, so they are included as a reference in their own folders:

### Safe concurrency with threads and mutexes: `mutex_example` and `write_file`

Some samples on writing to a file asynchronously and just spawning threads for fun.

### A Rust example of REST API programming `rust_fist_api`

This one is a very **small** API but I allowed myself to **toy around Rust tooling**.

The API just answers with its version at this point in time, I wanted to research on 2 topics:

- Start a server (http is not a first class citizen in rust as opposed to GoLang)
- JSON Serialization


