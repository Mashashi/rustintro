fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    
    /*BLOCK 1*/
    /*
    { // If we hadn't this scope the program would not compile
    	let p: &mut Vec<_> = &mut vec;
    	take(p);
    }
    println!("The value is {}", vec[1]);
    */

	/*BLOCK 2 - Alternative to 1*/
	let p: &mut Vec<_> = &mut vec;
	take(p);
    println!("The value is {}", p[1]);

}

fn take(vec: &mut Vec<i32>){
	// Just take ownership
}