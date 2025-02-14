// intro2.rs
// Make the code print a greeting to the world.
// Execute `rustlings hint intro2` or use the `hint` watch subcommand for a hint.

fn main() {
    println!("Hello {}!", "world");
    println!("{number:>5}", number=1);

    println!("Base 10 repr:               {}",   69420);
    println!("Base 2 (binary) repr:       {:b}", 69420);
    println!("Base 8 (octal) repr:        {:o}", 69420);
    println!("Base 16 (hexadecimal) repr: {:x}", 69420);
    println!("Base 16 (hexadecimal) repr: {:X}", 69420);

    println!("{subject} {verb} {object}",
    object="the lazy dog",
    subject="the quick brown fox",
    verb="jumps over");
}
