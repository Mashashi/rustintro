/*
Using a box we can allocate heap memory
*/
fn main() {
    let i: Box<i32> = Box::new(3);
    process(i);
    //println!("The value is {}", i);
}

fn process(val: Box<i32>){
	println!("The value is {}",val);
}