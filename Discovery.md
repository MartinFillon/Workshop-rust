# Workshop Rust

## Welcome

Hello there,

You are here to start to learn rust.
In this workshop you will learn the basics of rust.
To start this workshop lets learn abour rust projects.

## Exercice 0

Start by creating a directory and running:
```sh
cargo init
```
This command will initialize a rust project. Rust projects uses cargo as a version controller and builder. (Like nvm and npm for the js boys)

To build your project run
```sh
cargo build
```

The binary is then found at the path:
```sh
./target/debug/$(PROJECT_NAME)
```

## Exercice 1

Debugging and printing.
In rust you can debug really easily just pass whatever you want to the `dbg!()` macro and it will be printed on the stderr.
Moreover you can use `print{ln}!()` to print text or formatted string to stdout

## Exercice 2

Lets start by creating a simple function
Create a function that takes a number and returns it successor.
Have a look at integer types in rust they are multiples.

## Exercie 3

Create a function taking a `String` and returning its len as the right integer type

## Exercice 4

Try running your function multiple time with the same variable named `a`
What happens ?
Does it work ?

You get an error, this is due to rust safety feature. In rust you cannot pass by copy a variable.
Everything is passed to a function using moving to move data to function. That means that a variable change owners.
To prevent that you can use multiple things the first (and the one you must use here) is Borrowing.
Borrowing is done using a `&` and the type must be adapted. (It works as a pointer in C but you don't allow them).

Make so that this code works:

```rust
fn main() -> {
	let s: String = "Vous savez ce que c'est une dicatature Dolorès ?".to_string();
	dbg!(strlen(&s));
	dbg!(strlen(&s));
}
```

You can note that we forced you to use a `String` type, but there is another one `&str`
The simple difference is that a String is mutable and a `&str` is a constant that should not be modified

## Exercice 5

Lets have some rust fun now, and use some of its best features.
In rust you can have some safer things. Lets see this.
Create a function that takes 2 floats and returns their division wrapped in the `Option` type.

Please handle right the division by zero with this.

## Exercice 6

Now that we have an `Option` type you can call either `unwrap` on it to fail if its none and get the value in the other case.
But you can also use `match` on it to prevent it from failling, just exit the program if its `None`.

## Exercice 7

Lets discover `Result` and `?` they are, respectively, a type and an operator you can use to safely have your code sending errors.
I let you search on the internet for some examples.

