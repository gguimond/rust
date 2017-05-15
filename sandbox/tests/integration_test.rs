#[test]
#[should_panic(expected = "assertion failed")]
fn it_works() {
	assert!(false);
	assert_eq!("Hello", "world");
}

#[test]
#[ignore]
fn it_works2() {
	assert!(false);
	assert_eq!("Hello", "world");
}