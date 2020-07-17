// Defining an Enum

// Let’s look at a situation we might want to express in code and see why enums are useful and more
// appropriate than structs in this case.

//  Say we need to work with IP addresses. Currently, two major standards are used for IP addresses:
// version four and version six. These are the only possibilities for an IP address that our program will come across:
// we can enumerate all possible variants, which is where enumeration gets its name.

// Any IP address can be either a version four or a version six address, but not both at the same time.
// That property of IP addresses makes the enum data structure appropriate, because enum values can only be one of its variants.
// Both version four and version six addresses are still fundamentally IP addresses, so they
// should be treated as the same type when the code is handling situations that apply to any kind of IP address.

// We can express this concept in code by defining an IpAddrKind enumeration and listing the possible kinds
// an IP address can be, V4 and V6.

enum IpAddrKind {
    V4,
    V6,
}

fn a() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
}

// Note that the variants of the enum are namespaced under its identifier, and we use a double colon to separate the two.

// The reason this is useful is that now both values IpAddrKind::V4 and IpAddrKind::V6 are of the same type: IpAddrKind.
// We can then, for instance, define a function that takes any IpAddrKind:
fn route(ip_kind: IpAddrKind) {}

// Using enums has even more advantages. Thinking more about our IP address type, at the moment we don’t have a way
// to store the actual IP address data; we only know what kind it is.
// Given that you just learned about structs in Chapter 5, you might tackle this problem as shown in the next example:

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn b() {
    let home = IpAddrKind {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

// Here, we’ve defined a struct IpAddr that has two fields: a kind field that is of type IpAddrKind (the enum we defined previously)
// and an address field of type String. We have two instances of this struct. The first, home, has the value IpAddrKind::V4 as
// its kind with associated address data of 127.0.0.1. The second instance, loopback, has the other variant of
// IpAddrKind as its kind value, V6, and has address ::1 associated with it. We’ve used a struct to bundle the
// kind and address values together, so now the variant is associated with the value.

// We can represent the same concept in a more concise way using just an enum, rather than an enum inside a struct,
// by putting data directly into each enum variant. This new definition of the IpAddr enum says that both V4 and V6
// variants will have associated String values:

// We attach data to each variant of the enum directly, so there is no need for an extra struct.
enum IpAddress {
    V4(String),
    V6(String),
}

fn c() {
    let home = IpAddress::V4(String::from("127.0.0.1"));

    let loopback = IpAddress::V6(String::from("::1"));
}

// There’s another advantage to using an enum rather than a struct: each variant can have different types and amounts of associated data.
// Version four type IP addresses will always have four numeric components that will have values between 0 and 255.
// If we wanted to store V4 addresses as four u8 values but still express V6 addresses as one String value, we wouldn’t
// be able to with a struct. Enums handle this case with ease:
enum IpAddress2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn d() {
    let home = IpAddress2::V4(127, 0, 0, 1);

    let loopback = IpAddress2::V6(String::from("::1"));

}

// We’ve shown several different ways to define data structures to store version four and version six IP addresses.
//
// However, as it turns out, wanting to store IP addresses and encode which kind they are is so common
// that the standard library has a definition we can use: (https://doc.rust-lang.org/std/net/enum.IpAddr.html)

// Let’s look at another example:
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// This enum has four variants with different types:

// - Quit has no data associated with it at all.
// - Move includes an anonymous struct inside it.
// - Write includes a single String.
// - ChangeColor includes three i32 values.

// There is one more similarity between enums and structs:
// just as we’re able to define methods on structs using impl, we’re also able to define methods on enums.
// Here’s a method named call that we could define on our Message enum:
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn e() {
    let m = Message::Write(String::from("hello"));
    m.call(); // WOW enum with methods ...
}

// The Option Enum and Its Advantages Over Null Values ---
// Let’s look at another enum in the standard library that is very common and useful: Option.

// This section explores a case study of Option, which is another enum defined by the standard library.

//  The Option type is used in many places because it encodes the very common scenario in which a value
// could be something or it could be nothing.

// Programming language design is often thought of in terms of which features you include, but the features you exclude are important too.
// Rust doesn’t have the null feature that many other languages have.
// Null is a value that means there is no value there.
// In languages with null, variables can always be in one of two states: null or not-null.

// The problem with null values is that if you try to use a null value as a not-null value, you’ll get an error of some kind.
// Because this null or not-null property is pervasive, it’s extremely easy to make this kind of error.

// However, the concept that null is trying to express is still a useful one:
// a null is a value that is currently invalid or absent for some reason.

// The problem isn’t really with the concept but with the particular implementation.
// As such, Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent.

// This enum is Option<T>, and it is defined by the standard library as follows:
enum Option<T> {
    Some(t),
    None,
}
// that  is f... awesome

// The Option<T> enum is so useful that it’s even included in the prelude; you don’t need to bring it into scope explicitly.
// In addition, so are its variants: you can use Some and None directly without the Option:: prefix.
// The Option<T> enum is still just a regular enum, and Some(T) and None are still variants of type Option<T>.

fn f() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
}

// If we use None rather than Some, we need to tell Rust what type of Option<T> we have, because
// the compiler can’t infer the type that the Some variant will hold by looking only at a None value.

// When we have a Some value, we know that a value is present and the value is held within the Some.
// When we have a None value, in some sense, it means the same thing as null: we don’t have a valid value.
// So why is having Option<T> any better than having null?

// In short, because Option<T> and T (where T can be any type) are different types, the compiler won’t
// let us use an Option<T> value as if it were definitely a valid value.
// For example, this code won’t compile because it’s trying to add an i8 to an Option<i8>:
fn g() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y; // error[E0277]: cannot add `std::option::Option<i8>` to `i8`
        // ^ no implementation for `i8 + std::option::Option<i8>`
}

// Intense! In effect, this error message means that Rust doesn’t understand how to add an i8 and an Option<i8>,
// because they’re different types. When we have a value of a type like i8 in Rust, the compiler will ensure that
// we always have a valid value.

// We can proceed confidently without having to check for null before using that value.
// Only when we have an Option<i8> (or whatever type of value we’re working with) do we have to worry about possibly not
// having a value, and the compiler will make sure we handle that case before using the value.

// In other words, you have to convert an Option<T> to a T before you can perform T operations with it.
// Generally, this helps catch one of the most common issues with null: assuming that something isn’t null when it actually is.

// Not having to worry about incorrectly assuming a not-null value helps you to be more confident in your code.
// In order to have a value that can possibly be null, you must explicitly opt in by making the type of that value Option<T>.

// Then, when you use that value, you are required to explicitly handle the case when the value is null.
// Everywhere that a value has a type that isn’t an Option<T>, you can safely assume that the value isn’t null.
// This was a deliberate design decision for Rust to limit null’s pervasiveness and increase the safety of Rust code.

// So, how do you get the T value out of a Some variant when you have a value of type Option<T> so you can use that value?

// In general, in order to use an Option<T> value, you want to have code that will handle each variant.
// You want some code that will run only when you have a Some(T) value, and this code is allowed to use the inner T.
// You want some other code to run if you have a None value, and that code doesn’t have a T value available.
// The match expression is a control flow construct that does just this when used with enums:
// it will run different code depending on which variant of the enum it has,
// and that code can use the data inside the matching value.