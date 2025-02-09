fn main() {
    let mut x = 5;
    let y = &mut x; 
    let z = &mut x; // This is where the error is
    *y = 6;
    *z = 7; 
    println!("x = {}", x); //Output:7 
}