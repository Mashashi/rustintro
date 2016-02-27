//we did not need to annotate the types of arguments the closure takes or the values it returns
/*fn main(){
	let plus_two = |x| {
	    let mut result: i32 = x;

	    result += 1;
	    result += 1;

	    result
	};
}*/

// we can
/*
let plus_one = |x: i32| -> i32 { x + 1 };
assert_eq!(2, plus_one(1));
*/


/*
fn  plus_one_v1   (x: i32) -> i32 { x + 1 }
let plus_one_v2 = |x: i32| -> i32 { x + 1 };
let plus_one_v3 = |x: i32|          x + 1  ;
*/

fn main() {
    let mut num = 5;
    {
		let plus_num = |x: i32| x + num;
		// closure gets num as a shared borrow
	}
	let y = &mut num;
}



/*fn main(){
	let nums = vec![1, 2, 3];
	let takes_nums = || nums;
	println!("{:?}", nums);
}*/
/*
note: `nums` moved into closure environment here because it has type
  `[closure(()) -> collections::vec::Vec<i32>]`, which is non-copyable
let takes_nums = || nums;
                 ^~~~~~~
*/