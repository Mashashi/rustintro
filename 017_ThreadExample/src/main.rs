use std::sync::mpsc::channel;
use std::thread;

fn main() { 
	let (tx,rx) = channel(); 
	let tx1 = tx.clone();  
	let _ = thread::spawn(move || { tx.send(1);}); //’spawn thread 
	let _ = thread::spawn(move || { tx1.send(3);}); //’spawn thread  
	rx.recv();
	rx.recv();
	//println!("Va1ue= {}", rx.recv()); 
	//println!("Va1ue= {}", rx.recv());
}