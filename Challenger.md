# Workshop Rust Part II

## Welcome

Hello again,

Continuing from the last workshop, [Discovery.md](https://github.com/MartinFillon/Workshop-rust/blob/main/Discovery.md), this workshop aims to dive deeper into the language and learn more on the Rust's diverse features.

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

# Part 1 - Pattern Matching

To start this workshop off, let's start off simple by covering the basic of **Pattern** and **Matching**.

Using patterns gives you more control over a program’s control flow and it is therefore why **Patterns** are such a special syntax in Rust.

## Exercise 0

Create a function that takes an integer type as parameter and is able to return the static string equivalent of the grade.

| Percentage  | Grade Value |
| ----------- | ----------- |
| 100% - 90%  | A           |
| 89% - 80%   | B           |
| 79% - 70%   | C           |
| 69% - 60%   | D           |
| < 60%       | F           |

(Hint: Look at `match`)

## Exercise 1

Now, print out the classification and additional information.

Example:

Input: `78`

Output:
```sh
Grade: C
Passing: Yes
Close to next higher grade: No
```

(Hints: Use nested `match` statements to handle multiple conditions and use string formatting to construct the output message.)

# Part 2 - Lifetimes

We previously looked at **Borrowing** and its concept within the context of Rust. Therefore, we can now explore **Lifetimes**.

For most of these exercises, the compiler will almost always specify your error and how to solve your program with lifetime specifiers.

Try and understand these changes and what the compiler is telling you.

## Exercise 2

Look at this piece of code now. What does the compiler say?

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
}
```

Can you make it compile?

## Exercise 3

// create a function with lifetime

## Exercise 4

// Go further

# Part 3 - Iterators

Let's look at Iterators!

## Exercise 5

.next()

## Exercise 5.5

.into_iter()

## Exercise 6

If you've already done the previous workshop, you can take your function from Exercise 7. If you haven't feel free to use [this code right here.](https://github.com/MartinFillon/Workshop-rust/blob/main/Challenger/SafeDiv.rs)

Now, I want you to use this divide function to create a program that will iterate from a vector (one below for example) and execute the divide function for each float in the vector with any constant float value of your choice.

Here's how you can start this off:

```rust
let entries = vec![27.0, 297.0, 38502.5, 81.2];
```

(Hint: Look at `map`)

## Exercise 7 - Linting

[Clippy](https://github.com/rust-lang/rust-clippy) : A collection of lints to catch common mistakes and improve your Rust code.

You can directly modify your `Cargo.toml` to modify and add to your dependencies, or you can directly run this command:

```sh
cargo add clippy
```

Now, **Clippy** can automatically apply some lint suggestions. Feel free to look at all the features provided by the command.

```sh
cargo clippy
```

It's a good practice **Linting** such as this for your Rust projects.
