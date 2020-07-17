// Concise Control Flow with if let

// The match expression can be a bit wordy in a situation in which we care about only one of the cases.
// For this situation, Rust provides if let.

// The if let syntax lets you combine if and let into a less verbose way to handle values that match
// one pattern while ignoring the rest.

// Consider the following program that matches on an Option<u8> value but only wants to execute code if the value is 3.
fn a() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
}

// We want to do something with the Some(3) match but do nothing with any other Some<u8> value or the None value.
// To satisfy the match expression, we have to add _ => () after processing just one variant, which is a lot of boilerplate code to add.

// Instead, we could write this in a shorter way using if let.
fn b() {
    if let Some(3) = some_u8_value {
        println!("three");
    }
}

// The syntax if let takes a pattern and an expression separated by an equal sign.
// It works the same way as a match, where the expression is given to the match and the pattern is its first arm.

// Using if let means less typing, less indentation, and less boilerplate code.
// However, you lose the exhaustive checking that match enforces.
// Choosing between match and if let depends on what you’re doing in your particular situation and whether
// gaining conciseness is an appropriate trade-off for losing exhaustive checking.

// In other words, you can think of if let as syntax sugar for a match that runs code when the value matches
// one pattern and then ignores all other values.

// We can include an else with an if let.
// The block of code that goes with the else is the same as the block of code that would go with the _ case in the
// match expression that is equivalent to the if let and else.

fn c() {
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
}
// this is the same as
fn d() {
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}