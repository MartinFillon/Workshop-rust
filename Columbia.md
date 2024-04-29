# Workshop Rust Part III

# Columbia

Welcome back now that you've done `Discovery` and `Challenger` its now time for `Columbia`.

As you might have seen everything here is based around NASA Space Shuttles.
Thats why they are gonna be the base for our project here.

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

## Exercice 0

For this exercice you gonna create a folder named `Shuttles` with a `discovery.rs` file inside.
In this file create a function `hello` that returns the string `"This is Discovery Shuttle. Over."`

You need to print the return from that function using this main:

```rust
fn main() -> {
    println!("{}\n", hello());
}
```

Make it work.

## Exercice 1

Now that you know that, create a **struct** `Discovery` that contains 3 members:
- one **public** witch is a string for the name.
- one **private** for the fuel which you must choose the type accordingly.
- one **private** for the speed which you must choose the type accordingly.

Create a function dedicated for this struct.

This function will create a new one of your structure through its parameters (the function's parameters).

<details>
  <summary> HINT </summary>

  Look at `impl`
</details>

## Exercice 2

Now we are gonna need to be able to print out the shuttle.

We expect a print like this: `"O=>"`

```rust
fn main() -> {
    let disc = Discovery::new();
    println!("{}\n", disc);
}
```

## Exercice 3

Now that you discovered traits lets create yours.

Create a trait `Shuttle` with a function `isTravelPossible` tells you if have enough fuel to cover a certain distance.

*The distance is a floating point number such as `1.7976931348623157E+307`, which should fit in it.*

The formula you need to use is: `fuel * speed == distance`

## Exercice 4

Now lets do this for `Challenger` and `Columbia`
Which have their respective files and as prints respectively:

- `!==>`
- `>>=`

## Exercice 5

Now I want to create a function that takes any of those shuttles and tells me if they are able to cover the distance of 1.0E+42.

Example of main:

```rust
fn main() -> {
    /*Define some Shuttles*/
    println!("{}", can_cover(chall1));
    println!("{}", can_cover(chall2));
    println!("{}", can_cover(chall3))
    println!("{}", can_cover(disc1));
    println!("{}", can_cover(disc2));
    println!("{}", can_cover(disc3));
    println!("{}", can_cover(columbia1));
    println!("{}", can_cover(columbia2));
    println!("{}", can_cover(columbia3));
}
```

## Bonus

Make your structures members Bean compliant.
