fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    take(vec);
    println!("The value is {}", vec[1]);
}

fn take(vec: Vec<i32>){
	// Just take ownership
}