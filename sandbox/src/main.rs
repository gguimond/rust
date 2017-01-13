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

	let x = 5;

	if x == 5 {
	    println!("x is five!");
	} else if x == 6 {
	    println!("x is six!");
	} else {
	    println!("x is not five or six :(");
	}

	let y = if x == 5 { 10 } else { 15 };
	println!("{}", y);

	let mut done = false;
	while !done {
		println!("loop");
		done = true;
	}

	for x in 0..10 {
	    println!("{}", x);
	}

	for (index, value) in (5..10).enumerate() {
	    println!("index = {} and value = {}", index, value);
	}

	let lines = "hello\nworld".lines();

	for(n, line) in lines.enumerate(){
		println!("{} : {}", n, line);
	}

	'loop1: loop{
		'loop2: loop{
			println!("loop infinite");
			break 'loop1;
		}
	}

	let v = vec![1, 2, 3, 4, 5];
	println!("The third element of v is {}", v[2]);
	match v.get(7) {
	    Some(x) => println!("Item 7 is {}", x),
	    None => println!("Sorry, this vector is too short.")
	}

	for i in &v {
	    println!("This is a reference to {}", i);
	}

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
