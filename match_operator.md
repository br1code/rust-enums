# The match Control Flow Operator

Rust has an extremely powerful control flow operator called match that allows you to compare a value
against a series of patterns and then execute code based on which pattern matches.

Patterns can be made up of literal values, variable names, wildcards, and many other things.

The power of match comes from the expressiveness of the patterns and the fact that the compiler
confirms that all possible cases are handled.

We can write a function that can take an unknown United States coin and,
in a similar way as the counting machine, determine which coin it is and return its value in cents:

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
````

Let’s break down the match in the value_in_cents function.
First, we list the match keyword followed by an expression, which in this case is the value coin.
This seems very similar to an expression used with if, but there’s a big difference:
with if, the expression needs to return a Boolean value, but here, it can be any type.
The type of coin in this example is the Coin enum that we defined on line 1.

Next are the match arms. An arm has two parts: a pattern and some code.
The first arm here has a pattern that is the value Coin::Penny and then the => operator that separates
the pattern and the code to run.

When the match expression executes, it compares the resulting value against the pattern of each arm, in order.
If a pattern matches the value, the code associated with that pattern is executed.
If that pattern doesn’t match the value, execution continues to the next arm.

The code associated with each arm is an expression, and the resulting value of the expression in the
matching arm is the value that gets returned for the entire match expression.

Curly brackets typically aren’t used if the match arm code is short.
If you want to run multiple lines of code in a match arm, you can use curly brackets.
For example, the following code would print “Lucky penny!” every time the method was called with a Coin::Penny
but would still return the last value of the block, 1:
````rust
fn value_in_cents_v2(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
````

## Patterns that Bind to Values

Another useful feature of match arms is that they can bind to the parts of the values that match the pattern.
This is how we can extract values out of enum variants.

As an example, let’s change one of our enum variants to hold data inside it.
From 1999 through 2008, the United States minted quarters with different designs for each of the 50 states on one side.
No other coins got state designs, so only quarters have this extra value.
We can add this information to our enum by changing the Quarter variant to include a UsState value stored inside it.
````rust
#[derive(Debug)] // so we can inspect the state later
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum CoinV2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
````

Let’s imagine that a friend of ours is trying to collect all 50 state quarters.
While we sort our loose change by coin type, we’ll also call out the name of the state associated with each quarter
so if it’s one our friend doesn’t have, they can add it to their collection.

In the match expression for this code, we add a variable called state to the pattern that matches values of
the variant Coin::Quarter. When a Coin::Quarter matches, the state variable will bind to the value of that quarter’s state.
Then we can use state in the code for that arm, like so:
````rust
fn value_in_cents_v3(coin: CoinV2) -> u8 {
    match coin {
        CoinV2::Penny => 1,
        CoinV2::Nickel => 5,
        CoinV2::Dime => 10,
        CoinV2::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
````


If we were to call value_in_cents(Coin::Quarter(UsState::Alaska)), coin would be Coin::Quarter(UsState::Alaska).
When we compare that value with each of the match arms, none of them match until we reach Coin::Quarter(state).
At that point, the binding for state will be the value UsState::Alaska.
We can then use that binding in the println! expression, thus getting the inner state value out of the Coin enum variant for Quarter.

Matching with Option<T> ---
In the previous section, we wanted to get the inner T value out of the Some case when using Option<T>;
we can also handle Option<T> using match as we did with the Coin enum!

Let’s say we want to write a function that takes an Option<i32> and, if there’s a value inside, adds 1 to that value.
If there isn’t a value inside, the function should return the None value and not attempt to perform any operations.

This function is very easy to write, thanks to match, and will look like this:
````rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn a() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
````

Combining match and enums is useful in many situations. You’ll see this pattern a lot in Rust code:
match against an enum, bind a variable to the data inside, and then execute code based on it.

## Matches Are Exhaustive

we must exhaust every last possibility in order for the code to be valid.
Especially in the case of Option<T>, when Rust prevents us from forgetting to explicitly handle the None case,
it protects us from assuming that we have a value when we might have null,
thus making the billion-dollar mistake discussed earlier impossible.

## The _ Placeholder
Rust also has a pattern we can use when we don’t want to list all possible values.

For example, a u8 can have valid values of 0 through 255.
If we only care about the values 1, 3, 5, and 7, we don’t want to have to list out 0, 2, 4, 6, 8, 9 all the way up to 255.
Fortunately, we don’t have to: we can use the special pattern _ instead:
````rust
fn b() {
    let some_u8_value = 0u8; // this is 0 of type u8
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}
````

The _ pattern will match any value. By putting it after our other arms, the _ will match all the possible cases that aren’t
specified before it. The () is just the unit value, so nothing will happen in the _ case.
As a result, we can say that we want to do nothing for all the possible values that we don’t list before the _ placeholder.

