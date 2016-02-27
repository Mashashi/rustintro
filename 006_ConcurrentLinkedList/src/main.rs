use std::ptr;

trait Constructable<T>{
	fn new() -> T;
}
struct Node<T : Constructable<T>>{
	value: T,
	next: *mut Node<T>,
}

impl<T : Constructable<T>> Node<T>{
	fn new(input: T) -> Node<T>{
		Node::<T>{ value: input, next: ptr::null_mut(), }
	}
	fn setNext(&mut self, next: *mut Node<T>){
		self.next = next;
	}
}

struct Computer{
	brand: i32,
	model: i32,
}

impl  Constructable<Computer> for Computer{
	fn new() -> Computer{
		Computer{
			brand: 1,
			model: 2,
		}
	}
}

/*struct ConcurrentLinkedList<T>{
	last: *mut Node<T>,
}

impl<T : Constructable<T>> ConcurrentLinkedList<T>{
	fn new() -> ConcurrentLinkedList<T>{
		ConcurrentLinkedList{last: ptr::null_mut(),}
	}
	fn add(&mut self, value: T){
		let oldLast = self.last;
		self.last = Node::new(value);
		if oldLast.is_null() {
			oldLast.setNext(self.last);
		}
	}
}*/

fn main(){
	
	let v = Node::<Computer>{ 
		value: Computer::new(),
		next: ptr::null_mut(),
	};

}

/*impl<T> Node<T>{
	fn new(value: T) -> Node<T>{
		Node{ value:0, next: ptr::null()}
	}
	fn setNext(&self, next: &Node<T>){
		self.next = next;
	}
}

struct ConcurrentLinkedList<T>{
	last: Node<T>,
}

impl<T> ConcurrentLinkedList<T>{
	fn new() -> ConcurrentLinkedList<T>{
		ConcurrentLinkedList{last: ptr::null()}
	}
	fn add(&self, value: T){
		let oldLast: *const i32 = self.last;
		self.last = Node::new(value);
		if oldLast.is_null() {
			oldLast.setNext(self.last);
		}
	}
}

fn main() {
    println!("Hello, world!");
}*/