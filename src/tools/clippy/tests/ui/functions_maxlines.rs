#![warn(clippy::too_many_lines)]

fn good_lines() {
    /* println!("This is good."); */
    // println!("This is good.");
    /* */ // println!("This is good.");
    /* */ // println!("This is good.");
    /* */ // println!("This is good.");
    /* */ // println!("This is good.");
    /* println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good."); */
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
    println!("This is good.");
}

fn bad_lines() {
    //~^ ERROR: this function has too many lines (102/100)
    //~| NOTE: `-D clippy::too-many-lines` implied by `-D warnings`
    println!("Dont get confused by braces: {{}}");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
    println!("This is bad.");
}

fn main() {}
