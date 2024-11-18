# Workshop Rust Part IV

## Welcome

Welcome back now that you've done `Discovery`, `Challenger` and `Columbia` its now time for `Endeavour`.

As you might have seen everything here is based around NASA Space Shuttles.
Thats why they are gonna be the base for our project here.

Continuing from the last workshop, [Columbia.md](https://github.com/MartinFillon/Workshop-rust/blob/main/Columbia.md), this workshop aims to dive deeper into the language and learn more on the Rust's diverse features.

First of all, a quick review of the basic commands that will be useful:

Create a Rust project:

```sh
cargo init
```

To build your project and create an executable:

```sh
cargo build
```

To quickstart your project:

```sh
cargo run
```

## Exercice 0

Until now you have been using `println!` to print out on the standard output.
But do you know there is a way to print on the standard error.
Find it and print the string `"error"` on the standard error.

<details>
  <summary> HINT </summary>

Look at `eprint`

</details>

## Exercice 1

Now that you know how to print on the standard output and the error output. We are going to write a function to use them both.
That function, prototyped below, will if the parameter error is set to true print to standard error otherwise print to the standard output.

```rs
fn print_helper(error: bool, to_print: String)
```

## Exercice 2

Now that we know how to print on both output easily we can see a problem. We can no longer print any variable just a String.
Find a way either by modifying this function, or by changing the way you call it to print the message.
`Hello {n} world` 10 times where n is the number of times you printed it.

<details>
  <summary> HINT </summary>

Look at `format!`

</details>

## Exercice 3

But there is a problem, I can't use this function if I want to print inside a file.
Change it so it is able to take any type that we can write into.

<details>
  <summary> HINT </summary>

Look at `Write` and `write!`

</details>

## Exercice 4

Know that we saw all that look again at the different functions you used until now.
What do they have in common ?
They all are rust-macros as per the `!` at the end of their name.
In rust macros are functions that are evaluated before compilation and that modifies the tokens inside them.
They are very powerful bits of code that can be used to abstract some part of the code that would otherwise be harder to understand.
They are used in a form of programming called `Metaprogramming`

Lets init a new project inside our current project in order to write our own macros called `workshop-macro`

```sh
mkdir workshop-macros
cd workshop-macros
cargo init --lib
```

Now lets declare them as a dependency of our original project by adding the line

```toml
workshop-macros = { path = "./workshop-macros" }
```

Remember to delete the content of `lib.rs`

## Exercice 5

Now that everything is ready lets write a simple macro that adds two numbers.
Its call should look like this

```rust
fn main() {
    let x = add!(1, 2);
    println!("{x}");
}
```

<details>
  <summary> HINT </summary>

Look at `macro_rules`

</details>
