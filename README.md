# Enums and Pattern Matching

> Personal notes on enums in Rust that I wrote while reading the book ["The Rust Programming Language"](https://doc.rust-lang.org/stable/book/)

Enums allow you to define a type by enumerating its possible variants. First, we’ll define and use
an ``enum`` to show how an ``enum`` can encode meaning along with data.

Next, we’ll explore a particularly useful ``enum``, called ``Option``, which expresses that a value
can be either something or nothing.

Then we’ll look at how pattern matching in the ``match`` expression makes it easy to run different
code for different values of an ``enum``.

Finally, we’ll cover how the ``if let`` construct is another convenient and concise idiom available
to you to handle enums in your code.

# Defining an Enum

Let’s look at a situation we might want to express in code and see why enums are useful and more
appropriate than structs in this case.

Say we need to work with IP addresses. Currently, two major standards are used for IP addresses:
version four and version six. These are the only possibilities for an IP address that our program will come across:
we can enumerate all possible variants, which is where enumeration gets its name.

Any IP address can be either a version four or a version six address, but not both at the same time.
That property of IP addresses makes the enum data structure appropriate, because enum values can only be one of its variants.
Both version four and version six addresses are still fundamentally IP addresses, so they
should be treated as the same type when the code is handling situations that apply to any kind of IP address.

We can express this concept in code by defining an ``IpAddrKind`` enumeration and listing the possible kinds
an IP address can be, V4 and V6.
````rust
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
}
````

Note that the variants of the enum are namespaced under its identifier, and we use a double colon to separate the two.

The reason this is useful is that now both values ``IpAddrKind::V4`` and ``IpAddrKind::V6`` are of the same type: ``IpAddrKind``.
We can then, for instance, define a function that takes any ``IpAddrKind``:
````rust
fn route(ip_kind: IpAddrKind) {}
````

Using enums has even more advantages. Thinking more about our IP address type, at the moment we don’t have a way
to store the actual IP address data; we only know what kind it is.
Given that you might tackle this problem as shown in the next example:
````rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let home = IpAddrKind {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}
````

Here, we’ve defined a struct ``IpAddr`` that has two fields: a kind field that is of type ``IpAddrKind`` (the ``enum`` we defined previously)
and an address field of type ``String``. We have two instances of this ``struct``. The first, ``home``, has the value ``IpAddrKind::V4`` as
its kind with associated address data of ``127.0.0.1``. The second instance, ``loopback``, has the other variant of
``IpAddrKind`` as its kind value, ``V6``, and has address ``::1`` associated with it. We’ve used a ``struct`` to bundle the
kind and address values together, so now the variant is associated with the value.

We can represent the same concept in a more concise way using just an enum, rather than an ``enum`` inside a ``struct``,
by putting data directly into each ``enum`` variant. This new definition of the ``IpAddr`` ``enum`` says that both ``V4`` and ``V6``
variants will have associated ``String`` values:

We attach data to each variant of the ``enum`` directly, so there is no need for an extra ``struct``.
````rust
enum IpAddress {
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddress::V4(String::from("127.0.0.1"));

    let loopback = IpAddress::V6(String::from("::1"));
}
````

There’s another advantage to using an ``enum`` rather than a ``struct``: each variant can have different types and amounts of associated data.
Version four type IP addresses will always have four numeric components that will have values between ``0`` and ``255``.
If we wanted to store ``V4`` addresses as four ``u8`` values but still express ``V6`` addresses as one ``String`` value, we wouldn’t
be able to with a ``struct``. Enums handle this case with ease:
````rust
enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddress::V4(127, 0, 0, 1);

    let loopback = IpAddress2::V6(String::from("::1"));

}
````

We’ve shown several different ways to define data structures to store version four and version six IP addresses.
However, as it turns out, wanting to store IP addresses and encode which kind they are is so common
that the standard library has a definition we can use: [See enum.IpAddr on the documentation](https://doc.rust-lang.org/std/net/enum.IpAddr.html)

Let’s look at another example:
````rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
````

This ``enum`` has four variants with different types:

- ``Quit`` has no data associated with it at all.
- ``Move`` includes an anonymous ``struct`` inside it.
- ``Write`` includes a single ``String``.
- ``ChangeColor`` includes three ``i32`` values.

There is one more similarity between enums and structs:
just as we’re able to define methods on structs using ``impl``, we’re also able to define methods on enums.
Here’s a method named ``call`` that we could define on our ``Message`` ``enum``:
````rust
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call(); // WOW enum with methods ...
}
````

# The Option Enum and Its Advantages Over Null Values

Let’s look at another ``enum`` in the standard library that is very common and useful: ``Option``.

This section explores a case study of ``Option``, which is another ``enum`` defined by the standard library.

The ``Option`` type is used in many places because it encodes the very common scenario in which a value
could be something or it could be nothing.

Programming language design is often thought of in terms of which features you include, but the features you exclude are important too.
Rust doesn’t have the ``null`` feature that many other languages have.
Null is a value that means there is no value there.
In languages with ``null``, variables can always be in one of two states: null or not-null.

The problem with ``null`` values is that if you try to use a ``null`` value as a not-null value, you’ll get an error of some kind.
Because this ``null`` or not-null property is pervasive, it’s extremely easy to make this kind of error.

However, the concept that ``null`` is trying to express is still a useful one:
a null is a value that is currently invalid or absent for some reason.

The problem isn’t really with the concept but with the particular implementation.
As such, Rust does not have nulls, but it does have an ``enum`` that can encode the concept of a value being present or absent.

This enum is ``Option<T>``, and it is defined by the standard library as follows:
````rust
enum Option<T> {
    Some(t),
    None,
}
````

> that is f...ing awesome

The ``Option<T>`` enum is so useful that it’s even included in the prelude; you don’t need to bring it into scope explicitly.
In addition, so are its variants: you can use ``Some`` and ``None`` directly without the ``Option::`` prefix.
The ``Option<T>`` enum is still just a regular ``enum``, and ``Some(T)`` and ``None`` are still variants of type ``Option<T>``.
````rust
fn f() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
}
````

If we use ``None`` rather than ``Some``, we need to tell Rust what type of`` Option<T>`` we have, because
the compiler can’t infer the type that the Some variant will hold by looking only at a ``None`` value.

When we have a ``Some`` value, we know that a value is present and the value is held within the ``Some``.
When we have a ``None`` value, in some sense, it means the same thing as null: we don’t have a valid value.
So why is having ``Option<T>`` any better than having ``null``?

In short, because ``Option<T>`` and ``T`` (where ``T`` can be any type) are different types, the compiler won’t
let us use an ``Option<T>`` value as if it were definitely a valid value.
For example, this code won’t compile because it’s trying to add an ``i8`` to an ``Option<i8>``:
````rust
fn g() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y; // error[E0277]: cannot add `std::option::Option<i8>` to `i8`
        // ^ no implementation for `i8 + std::option::Option<i8>`
}
````

Intense! In effect, this error message means that Rust doesn’t understand how to add an ``i8`` and an ``Option<i8>``,
because they’re different types. When we have a value of a type like ``i8`` in Rust, the compiler will ensure that
we always have a valid value.

We can proceed confidently without having to check for ``null`` before using that value.
Only when we have an ``Option<i8>`` (or whatever type of value we’re working with) do we have to worry about possibly not
having a value, and the compiler will make sure we handle that case before using the value.

In other words, you have to convert an ``Option<T>`` to a ``T`` before you can perform ``T`` operations with it.
Generally, this helps catch one of the most common issues with null: assuming that something isn’t null when it actually is.

Not having to worry about incorrectly assuming a not-null value helps you to be more confident in your code.
In order to have a value that can possibly be ``null``, you must explicitly opt in by making the type of that value ``Option<T>``.

Then, when you use that value, you are required to explicitly handle the case when the value is ``null``.
Everywhere that a value has a type that isn’t an ``Option<T>``, you can safely assume that the value isn’t ``null``.
This was a deliberate design decision for Rust to limit null’s pervasiveness and increase the safety of Rust code.

So, how do you get the ``T`` value out of a ``Some`` variant when you have a value of type ``Option<T>`` so you can use that value?

In general, in order to use an ``Option<T>`` value, you want to have code that will handle each variant.
You want some code that will run only when you have a ``Some(T)`` value, and this code is allowed to use the inner ``T``.
You want some other code to run if you have a ``None`` value, and that code doesn’t have a ``T`` value available.
The ``match`` expression is a control flow construct that does just this when used with enums:
it will run different code depending on which variant of the enum it has,
and that code can use the data inside the matching value.

# The match Control Flow Operator

Rust has an extremely powerful control flow operator called ``match`` that allows you to compare a value
against a series of patterns and then execute code based on which pattern matches.

Patterns can be made up of literal values, variable names, wildcards, and many other things.

The power of ``match`` comes from the expressiveness of the patterns and the fact that the compiler
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

Let’s break down the ``match`` in the ``value_in_cents`` function.
First, we list the ``match`` keyword followed by an expression, which in this case is the value ``coin``.
This seems very similar to an expression used with ``if``, but there’s a big difference:
with ``if``, the expression needs to return a ``Boolean`` value, but here, it can be any type.
The type of ``coin`` in this example is the ``Coin`` enum that we defined on line 1.

Next are the match arms. An arm has two parts: a pattern and some code.
The first arm here has a pattern that is the value ``Coin::Penny`` and then the ``=>`` operator that separates
the pattern and the code to run.

When the ``match`` expression executes, it compares the resulting value against the pattern of each arm, in order.
If a pattern matches the value, the code associated with that pattern is executed.
If that pattern doesn’t match the value, execution continues to the next arm.

The code associated with each arm is an expression, and the resulting value of the expression in the
matching arm is the value that gets returned for the entire match expression.

Curly brackets typically aren’t used if the ``match`` arm code is short.
If you want to run multiple lines of code in a ``match`` arm, you can use curly brackets.
For example, the following code would print ``“Lucky penny!”`` every time the method was called with a ``Coin::Penny``
but would still return the last value of the block, ``1``:
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

Another useful feature of ``match`` arms is that they can bind to the parts of the values that ``match`` the pattern.
This is how we can extract values out of enum variants.

As an example, let’s change one of our enum variants to hold data inside it.
From 1999 through 2008, the United States minted quarters with different designs for each of the 50 states on one side.
No other coins got state designs, so only quarters have this extra value.
We can add this information to our enum by changing the ``Quarter`` variant to include a ``UsState`` value stored inside it.
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

In the ``match`` expression for this code, we add a variable called ``state`` to the pattern that matches values of
the variant ``Coin::Quarter``. When a ``Coin::Quarter`` matches, the ``state`` variable will bind to the value of that quarter’s state.
Then we can use ``state`` in the code for that arm, like so:
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


If we were to call ``value_in_cents(Coin::Quarter(UsState::Alaska))``, ``coin`` would be ``Coin::Quarter(UsState::Alaska)``.
When we compare that value with each of the match arms, none of them match until we reach ``Coin::Quarter(state)``.
At that point, the binding for ``state`` will be the value ``UsState::Alaska``.
We can then use that binding in the ``println!`` expression, thus getting the inner state value out of the ``Coin`` enum variant for ``Quarter``.

## Matching with Option<T>

In the previous section, we wanted to get the inner ``T`` value out of the ``Some`` case when using ``Option<T>``;
we can also handle Option<T> using match as we did with the Coin enum!

Let’s say we want to write a function that takes an ``Option<i32>`` and, if there’s a value inside, adds ``1`` to that value.
If there isn’t a value inside, the function should return the ``None`` value and not attempt to perform any operations.

This function is very easy to write, thanks to ``match``, and will look like this:
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

Combining ``match`` and enums is useful in many situations. You’ll see this pattern a lot in Rust code:
``match`` against an ``enum``, bind a variable to the data inside, and then execute code based on it.

## Matches Are Exhaustive

We must exhaust every last possibility in order for the code to be valid.
Especially in the case of ``Option<T>``, when Rust prevents us from forgetting to explicitly handle the ``None`` case,
it protects us from assuming that we have a value when we might have null,
thus making the billion-dollar mistake discussed earlier impossible.

## The _ Placeholder

Rust also has a pattern we can use when we don’t want to list all possible values.

For example, a ``u8`` can have valid values of ``0`` through ``255``.
If we only care about the values ``1``, ``3``, ``5``, and ``7``, we don’t want to have to list out ``0``, ``2``, ``4``, ``6``, ``8``, ``9`` all the way up to ``255``.
Fortunately, we don’t have to: we can use the special pattern ``_`` instead:
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

The ``_`` pattern will match any value. By putting it after our other arms, the ``_`` will match all the possible cases that aren’t
specified before it. The ``()`` is just the unit value, so nothing will happen in the ``_`` case.
As a result, we can say that we want to do nothing for all the possible values that we don’t list before the ``_`` placeholder.

# Concise Control Flow with if let

The ``match`` expression can be a bit wordy in a situation in which we care about only one of the cases.
For this situation, Rust provides ``if let``.

The ``if let`` syntax lets you combine ``if`` and ``let`` into a less verbose way to handle values that match
one pattern while ignoring the rest.

Consider the following program that matches on an ``Option<u8>`` value but only wants to execute code if the value is ``3``.
````rust
fn a() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
}
````

We want to do something with the ``Some(3)`` match but do nothing with any other ``Some<u8>`` value or the ``None`` value.
To satisfy the match expression, we have to add ``_ => ()`` after processing just one variant, which is a lot of boilerplate code to add.

Instead, we could write this in a shorter way using ``if let``.
````rust
fn a() {
    if let Some(3) = some_u8_value {
        println!("three");
    }
}
````

The syntax ``if let`` takes a pattern and an expression separated by an equal sign.
It works the same way as a ``match``, where the expression is given to the ``match`` and the pattern is its first arm.

Using ``if let`` means less typing, less indentation, and less boilerplate code.
However, you lose the exhaustive checking that ``match`` enforces.
Choosing between ``match`` and ``if let`` depends on what you’re doing in your particular situation and whether
gaining conciseness is an appropriate trade-off for losing exhaustive checking.

In other words, you can think of ``if let`` as syntax sugar for a ``match`` that runs code when the value matches
one pattern and then ignores all other values.

We can include an ``else`` with an ``if let``.
The block of code that goes with the ``else`` is the same as the block of code that would go with the ``_`` case in the
match expression that is equivalent to the i``f let`` and ``else``.
````rust
fn a() {
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
}
````

this is the same as

````rust
fn a() {
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
````
