fn main() {
    let mut x = 5;
    { //this is called a block
        let y = &mut x; 
        *y = 6;
        println!("x = {}", x); //Output:6 
    }
    { //this is another block
        let z = &mut x; 
        *z = 7;
        println!("x = {}", x); //Output:7
    }    
}