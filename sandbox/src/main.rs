//! # Rust practice

fn main() {
	//variable
	let (a,b) = (1,2);
    println!("{} {}", a , b);

    let x:i32 = 5;
    println!("{}", x);

    let mut x = "foo";
    println!("{}", x);
    x = "bar";
    println!("{}", x);

    println!("{}", add(1,2));

    let f: fn(i32, i32) -> i32 = add;

    println!("{}", f(1,2));

    let x = true;
    let y: bool = false;

    let x = 'x';

    let slice = [0, 1, 2, 3, 4];
    let middle = &slice[1..4];

    println!("{}", middle[0]);

    let x: (i32, &str) = (1, "hello");

    let mut x = (1, 2); 
	let y = (2, 3);
	x = y;

	let (_x,_y) = x;
	println!("{}", _x);
	println!("{}", x.0);

	assert_eq!(6, add(5,1));
}

//function
/// assert_eq!(6, add(5,1));
/// # fn add(x :i32, y :i32) -> i32{
/// #     x + y
/// # }
fn add(x :i32, y :i32) -> i32{
	x + y
}

fn diverges() -> ! {
    panic!("This function never returns!");
}
