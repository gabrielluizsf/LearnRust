/*
    format!: write formatted text to String
    print!: same as format! but the text is printed to the console (io::stdout).
    println!: same as print! but a newline is appended.
    eprint!: same as print! but the text is printed to the standard error (io::stderr).
    eprintln!: same as eprint! but a newline is appended.
*/
fn main() {
    let name = "Gabriel Luiz";
    let number: f64 = 1.0;
    let width: usize = 5;
     // For Rust 1.58 and above, you can directly capture the argument from a
    // surrounding variable. Just like the above, this will output
    // "    1". 4 white spaces and a "1".
    println!("{number:>width$}");
     // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("Hello {}",name);
    println!("{} days", 31);

}
