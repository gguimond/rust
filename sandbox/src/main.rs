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

	//ownership
	let v2 = v;
	//println!("v[0] {}", v[0]);

	let own = 1;
	let own2 = own;
	println!("{}", own);

	fn sum_vec(v: &Vec<i32>) -> i32 {
        return v.iter().fold(0, |a, &b| a + b);
    }

    // Borrow two vectors and sum them.
    // This kind of borrowing does not allow mutation through the borrowed reference.
    fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
        // Do stuff with `v1` and `v2`.
        let s1 = sum_vec(v1);
        let s2 = sum_vec(v2);
        // Return the answer.
        s1 + s2
    }

    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];

    let answer = foo(&v1, &v2);
    println!("{}", answer);

    let mut x = 5;
	{
	    let y = &mut x;
	    *y += 1;
	}
	println!("{}", x);

	/*one or more references (&T) to a resource,
	exactly one mutable reference (&mut T).*/
	//let y: &i32;
	let x = 5;
	let y: &i32;
	y = &x;

	println!("{}", y);


	//lifetimes
	fn skip_prefix<'a, 'b>(line: &'a str, prefix: &'b str) -> &'a str {
	    return line;
	}

	let line = "lang:en=Hello World!";
	let lang = "en";

	let v;
	{
	    let p = format!("lang:{}=", lang);  // -+ `p` comes into scope.
	    v = skip_prefix(line, p.as_str());  //  |
	}                                       // -+ `p` goes out of scope.
	println!("{}", v);

	struct Foo<'a> {
	    x: &'a i32,
	}

	impl<'a> Foo<'a> {
	    fn x(&self) -> &'a i32 { self.x }
	}

    let y = &5; // This is the same as `let _y = 5; let y = &_y;`.
    let f = Foo { x: y };

    println!("{}", f.x);

	let x: &'static str = "Hello, world.";

	let mut x = 5;
	//mutable binding to a mutable ref
	let mut y = &mut x;

	use std::cell::RefCell;

	let x = RefCell::new(42);

	let y = x.borrow_mut();
	//let z = x.borrow_mut();

	struct Point {
	    x: i32,
	    y: i32,
	}

	struct PointRef<'a> {
	    x: &'a mut i32,
	    y: &'a mut i32,
	}

	let mut point = Point { x: 0, y: 0 };

    {
        let r = PointRef { x: &mut point.x, y: &mut point.y };

        *r.x = 5;
        *r.y = 6;
    }

    assert_eq!(5, point.x);
    assert_eq!(6, point.y);

    point = Point { x: 0, ..point};
    assert_eq!(6, point.y);

    struct Color(i32, i32, i32);
    let black = Color(17, 0, 0);
    let Color(r, _, _) = black;

    println!("{}", r);

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
