# Workshop Rust

## Welcome

Hello there,

You are here to start to learn Rust.
In this workshop you will learn the basics of Rust.

To start this workshop off, lets learn about Rust projects.

## Exercice 0

Start by creating a directory and running:
```sh
cargo init
```
This command will initialize a Rust project. Rust projects uses cargo as a version controller and builder. (Like nvm and npm for the js boys)

To build your project and create an executable:
```sh
cargo build
```

The binary is then found at the path:
```sh
./target/debug/$(PROJECT_NAME)
```

To quickstart your project and directly run your executable:
```sh
cargo run
```

## Exercice 1

Debugging and printing.

In Rust you can debug really easily just pass whatever you want to the `dbg!()` macro and it will be printed on the stderr.
Moreover, you can use `println!()` to print text or formatted string to stdout.

## Exercice 2

Lets start by creating a simple function.

Create a function that takes a number and returns it successor.
Have a look at integer types in Rust they are multiples.

## Exercie 3

Create a function taking a `String` and returning its len as the right integer type.

## Exercice 4

Try running your function multiple time with the same variable named `a`.
What happens ?
Does it work ?

You get an error, this is due to Rust safety feature. In Rust you cannot pass by copy a variable.
Everything is passed to a function using moving to move data to function. That means that a variable changes owners.
To prevent that you can use multiple things the first (and the one you must use here) is **Borrowing**.
**Borrowing** is done using a `&` and the type must be adapted. (It works as a pointer in C but you don't allow them).

Make so that this code works:

```rust
fn main() {
	let s: String = "Vous savez ce que c'est une dicatature Dolorès ?".to_string();
	dbg!(strlen(&s));
	dbg!(strlen(&s));
}
```

You can note that we forced you to use a `String` type, but there is another one `&str`.
The simple difference is that a String is **mutable** and a `&str` is a **constant** that should not be modified.

## Exercice 5

Lets have some Rust fun now, and use some of its best features.

In Rust you can have some safer things. Lets see this.
Create a function that takes 2 floats and returns their division wrapped in the `Option` type.

Please handle right the division by zero with this.

## Exercice 6

Now that we have an `Option` type you can call either `unwrap` on it to fail if its none and get the value in the other case.
But you can also use `match` on it to prevent it from failling, just exit the program if its `None`.

## Exercice 7

Lets discover `Result` and `?` they are, respectively, a type and an operator you can use to safely have your code sending errors.

I let you search on the internet for some examples.

## Exercise 8 - Test your Program

Lets test our code now.

Look at the `#[test]` annotation, this attribute indicates your program that this is a **test function**.
Make sure to specify this attribute at appropriate times.
Then, you can start checking your code and assert that its behavior is correct.

*Feel free to look further into attributes in Rust that may be useful. e.g. `#[ignore]` or `#[should_panic]` in this context for tests*

Then look at the `#[cfg(test)]` annotation.
This will tell Rust to compile and run the test code only when you run `cargo test`.
(not when you run `cargo build`)

Therefore, you can now run your tests only with:
```sh
cargo test
```
