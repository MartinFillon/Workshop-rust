# Workshop Rust Part II

## Welcome

Hello again,

Continuing from the last workshop, [Discovery.md](https://github.com/MartinFillon/Workshop-rust/blob/main/Discovery.md), this workshop aims to dive deeper into the language and learn more on the Rust's diverse features.

This one will be a little more advanced so try and work at your own pace and feel free to ask question at any point.

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
| 100 - 90    | A           |
| 89 - 80     | B           |
| 79 - 70     | C           |
| 69 - 60     | D           |
| < 60        | F           |

<details>
  <summary> HINT </summary>

  Look at `match`
</details>

## Exercise 1

Now, print out the classification and additional information.

*Example*:

Input: `78`

Output:
```sh
Grade: C
Passing: Yes
```

<details>
  <summary> HINT </summary>

  Use nested `match` statements to handle multiple conditions and use string formatting to construct the output message.
</details>

> Simple right? Imagine a FizzBuzz in Rust

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

What about here? What's wrong now?

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is '{}'", result);
}
```

Do you understand what the compiler is saying?

# Part 3 - Iterators

Let's look at **Iterators** now.

An Iterator pattern will basically allow you to perform some task on a sequence of items in turn.

## Exercise 4

Create a function where you:
- create an array of values (whatever you want)
- create a mutable iterator of this array
- print the iterator
- iterate through the iterator with any kind of looping method to print each element in the array

<details>
  <summary> HINT </summary>

  You can format your prints with `"{:?}"`
</details>

## Exercise 5

We are now gonna look at the `.next()` implementation of an `Iterator`.

So here, just change the way you iterate your array by deleting your loop and simply use `.next()` on your iterator. What does this do?

Now, print individual elements in the array however many times you want.

What happens when you `.next()` the last element and print it?

## Exercise 6

Now, we simply want you to print the iterator again. After your `.next()` implementations, what happens to the iterator here?

What about the array? Print the array and see what your program tells you.

What does the `.next()` method really do in your program?

Feel free to print the iterator between each call of `next` and understand the implementation.

## Exercise 7

Ranges (like `0..10` in Rust) are also **iterators**.

With this, we understand an iterator is something that we can call the `.next()` method on repeatedly, and it gives us a sequence of things.

Try and implement this with a basic parsing of arguments in your main. This may be a good practice for your future projects in Rust.

Maybe you could create a parsing of arguments for your program in *Exercise 1* like this:

*Example*

`./target/debug/<.exe> Audrey 48`

Output:
```txt
Student: Audrey
Grade: F
Passing: No
```

You can do some error handling and even print the usage with a specific flag,...

<details>
  <summary> HINT </summary>

  Look at the modules below to manage arguments and look at how to exit your program.

  ```rust
  use std::env;
  use std::process;
  ```

  Be careful here and make sure you understand what type of values are returned when using the different functions.
</details>

## Exercise 8 - Challenge

Try to recode a function that returns the factorial of `u64` value.

For this, we don't want you to use:
 - return
 - for or while
 - additional variables

and for a little bit more of a challenge:
 - no recursion

**The aim of this exercise is for computing the factorial elegantly with ranges and iterators.**

<details>
  <summary> HINT </summary>

  Search around on different functions you can use like `fold` and `rfold`
</details>

## Bonus - Linting in Rust

[Clippy](https://github.com/rust-lang/rust-clippy) is a collection of lints to catch common mistakes and improve your Rust code.

You can directly modify your `Cargo.toml` to modify and add to your dependencies, or you can directly run this command:

```sh
cargo add clippy
```

Now, **Clippy** can automatically apply some lint suggestions. Feel free to look at all the features provided by the command.

```sh
cargo clippy
```

It's a good practice to do this for your Rust projects.
