// This is a comment.

fn main() {
    let x: i32 = -42;

    println!(
        "x is a signed integer named with an i in front of the count of bits used to represent it."
    );
    println!("Signed integers range from a negative to a positive range.");
    println!(
        "Displaying the difference between negative and positive numbers is done with a sign."
    );
    println!("'+' for positive which is usually ignored and '-' for negative.");
    println!("That is why they are called signed.");
    println!();
    println!("The range of signed values ranges from -2^(bits - 1) to (+2^(bits - 1) - 1 .");
    println!("For 8 bits this would mean a range from -128 to +127 .");
    println!("That can be tried out by assigning for example '-129' to a variable 'z: i8':");
    println!(
        "= note: the literal `-129` does not fit into the type `i8` whose range is `-128..=127`"
    );
    println!();

    println!("The value of signed integer x is: {}", x);

    let y: u64 = 100;
}
