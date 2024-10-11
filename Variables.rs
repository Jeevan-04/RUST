fn main() {
//By default, variables are immutable − read only in Rust.
    let fees1 = 25_000;
    println!("fees1 is {} ",fees1);
    // fees1 = 35_000;
    // println!("fees1 changed is {}",fees1);

//Mutable
// let mut variable_name = value;
// let mut variable_name:dataType = value;
// Let us understand this with an example

let mut fees2:i32 = 25_000;
println!("fees2 is {} ",fees2);
fees2 = 35_000;
println!("fees changed is {}",fees2);
}