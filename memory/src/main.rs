// Static Memory
//  Program Binary - Static Variables - String literals
//  * Know compilation size
//  Size: Fixed
//  Lifetime: Whole Program
//  Cleanup: When Program Terminates
static _Y: u32 = 13;
fn main() {
    hello(_Y);    
    let name = "Gabriel Luiz";
    hello(name)
}
// HEAP Memory
// Values that live beyond functions - shared across threads - large values - dynamic size values
// * Unknown compilation size
// Size: Dynamic
// * up to computer limit
// Lifetime: Defined by programmer or language 
// Cleanup: Manually or via GC or via RALI

//Stack Memory
// Function arguments - Local Variables - Each thread has an isolated stack 
// * Know compilation size
// Size: Dynamic 
// * Top Limit
// Lifetime: Function
// Cleanup: When function returns
fn hello<T: std::fmt::Display>(you: T){
    println!("Hello {}", you);    
}
