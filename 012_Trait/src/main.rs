
use std::ptr;
use std::str::FromStr;

trait Constructable<T>{
	fn new() -> T;
}

struct Node<T : Constructable<T>>{
	value: T,
	next: *const Node<T>,
}

struct Computer{
	brand: String,
	model: String,
}

impl  Constructable<Computer> for Computer{
	fn new() -> Computer{
		Computer{
			brand: String::new(),
			model: String::new(),
		}
	}
}

fn main(){
	let v = Node::<Computer>{ 
		value: Computer{ 
			brand: String::from_str("sony").unwrap(), 
			model: String::from_str("vpc").unwrap(),
		},
		next: ptr::null(),
	};
}