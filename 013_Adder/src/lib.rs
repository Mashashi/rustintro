/*
Echo %ERRORLEVEL%
*/
#[test]
//#[should_panic]
#[should_panic(expected = "assertion failed")]
fn it_works() {
	//assert!(false);
	assert_eq!("Hello", "world");
}

#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}