//! # Rust practice

extern crate phrases;
pub use phrases::english::greetings::hello as hi;

mod test;


pub mod sandbox {	
	//function
	/// assert_eq!(6, add(5,1));
	/// # fn add(x :i32, y :i32) -> i32{
	/// #     x + y
	/// # }
	pub fn add(x :i32, y :i32) -> i32{
		x + y
	}

	pub fn diverges() -> ! {
	    panic!("This function never returns!");
	}
}

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

    println!("{}", sandbox::add(1,2));

    let f: fn(i32, i32) -> i32 = sandbox::add;

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

	assert_eq!(6, sandbox::add(5,1));

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

    enum Message {
	    Quit,
	    ChangeColor(i32, i32, i32),
	    Move { x: i32, y: i32 },
	    Write(String),
	}

	let v = vec!["Hello".to_string(), "World".to_string()];

	let v1: Vec<Message> = v.into_iter().map(Message::Write).collect();

	let x = 5;

	match x {
	    1 => println!("one"),
	    2 => println!("two"),
	    3 => println!("three"),
	    4 => println!("four"),
	    5 => println!("five"),
	    6 | 7 => println!("six or seven"),
	    _ => println!("something else"),
	}

	let number = match x {
	    1 => "one",
	    2 => "two",
	    3 => "three",
	    4 => "four",
	    5 => "five",
	    _ => "something else",
	};

	let message = Message::Quit;

	match message {
        Message::Quit => println!("quit"),
        Message::ChangeColor(r, g, b) => println!("color"),
        Message::Move { x, y: new_name_for_y } => println!("move"),
        Message::Write(s) => println!("write"),
    };

    let x = 1;
	let c = 'c';

	match c {
	    x => println!("x: {} c: {}", x, c),
	}

	println!("x: {}", x);

	let origin = Point { x: 0, y: 0 };
	let Point { x, y } = origin;

	let tuple = (5, String::from("five"));
	let (x, _) = tuple;
	//string is not moved thanks to _
	println!("Tuple is: {:?}", tuple);

	let (x, ..) = tuple;

	let mut x = 5;

	match x {
		ref name @ 1 ... 5 if *name < 5 => println!("one through four {}", name),
		ref name @ 1 ... 5 if *name >= 5 => println!("five {}", name),
	    ref mut mr => println!("Got a mutable reference to {}", mr),
	}

	struct Circle {
	    x: f64,
	    y: f64,
	    radius: f64,
	}

	impl Circle {
	    fn area(&self) -> f64 {
	        std::f64::consts::PI * (self.radius * self.radius)
	    }
	    fn reference(&self) ->  &Circle{
	       println!("taking self by reference!");
	       self
	    }

	    fn mutable_reference(&mut self) {
	       println!("taking self by mutable reference!");
	    }

	    fn takes_ownership(self) {
	       println!("taking ownership of self!");
	    }

	    fn new(x: f64, y: f64, radius: f64) -> Circle {
	        Circle {
	            x: x,
	            y: y,
	            radius: radius,
	        }
	    }
	}

	struct CircleBuilder {
	    x: f64,
	    y: f64,
	    radius: f64,
	}

	let mut c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
	c = Circle::new(0.0, 0.0, 2.0);
    println!("{}", c.reference().area());

	impl CircleBuilder {
	    fn new() -> CircleBuilder {
	        CircleBuilder { x: 0.0, y: 0.0, radius: 1.0, }
	    }

	    fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
	        self.x = coordinate;
	        self
	    }

	    fn y(&mut self, coordinate: f64) -> &mut CircleBuilder {
	        self.y = coordinate;
	        self
	    }

	    fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
	        self.radius = radius;
	        self
	    }

	    fn finalize(&self) -> Circle {
	        Circle { x: self.x, y: self.y, radius: self.radius }
	    }
	}

	c = CircleBuilder::new().x(1.0)
                .y(2.0)
                .radius(2.0)
                .finalize();;
    println!("{}", c.reference().area());
		
	let greeting = "Hello there."; // greeting: &'static str
	let mut s = "Hello".to_string(); // mut s: String
	fn takes_slice(slice: &str) {
	    println!("Got: {}", slice);
	}

    takes_slice(&s);

    for c in s.chars() {
	    print!("{}, ", c);
	}

	let c = s.chars().nth(0);

    let sl = {
    	let tmp = &s[0..5];
    	println!("{}", tmp);
    };
    

    let mut concat = s + "foo";
    println!("{}", concat);
    let concat2 = "bar".to_string() + &concat;
    println!("{}", concat2);

	let x: Option<i32> = Some(5);

	fn takes_anything<T>(x: T) {
	    // Do something with `x`.
	}
	takes_anything(concat2);

	struct PointGeneric<T> {
	    x: T,
	    y: T,
	}

	impl<T> PointGeneric<T> {
	    fn swap(&mut self) {
	        std::mem::swap(&mut self.x, &mut self.y);
	    }
	}

	let int_origin = PointGeneric { x: 0, y: 0 };
	let float_origin = PointGeneric { x: 0.0, y: 0.0 };

	trait HasArea {
		fn area(&self) -> f64;
		fn is_larger(&self, &Self) -> bool;
	}

	impl HasArea for Circle {
		fn area(&self) -> f64 {
			std::f64::consts::PI * (self.radius * self.radius)
		}
		fn is_larger(&self, other: &Self) -> bool {
	        self.area() > other.area()
	    }
	}

	use std::fmt::Debug;

	fn print_area<T: HasArea>(shape: T) {
	    println!("This shape has an area of {}", shape.area());
	}

	fn test <T: HasArea + Debug>(){

	}

	fn test2 <T>() where T : HasArea + Debug{
		
	}

	let c = Circle {
        x: 0.0f64,
        y: 0.0f64,
        radius: 1.0f64,
    };

    print_area(c);

    trait bar : HasArea {
	    fn is_valid(&self) -> bool;

	    fn is_invalid(&self) -> bool { !self.is_valid() }
	}

	#[derive(Debug)]
	struct deriving;

	impl Drop for Circle {
	    fn drop(&mut self) {
	        println!("Dropping!");
	    }
	}

	fn main() {
	    let x = Circle { x: 0.0, y: 0.0, radius: 2.0 };

	    // Do stuff.

	}

	let option: Option<i32> = Some(5);

	match option {
	    Some(x) => { println!("match!"); },
	    None => {},
	}

	if option.is_some() {
	    let x = option.unwrap();
	    println!("match!");
	}

	if let Some(x) = option {
	    println!("match!");
	}

	trait FooBar {
	    fn method(&self) -> String;
	}

	impl FooBar for u8 {
	    fn method(&self) -> String { format!("u8: {}", *self) }
	}

	impl FooBar for String {
	    fn method(&self) -> String { format!("string: {}", *self) }
	}

	fn do_something<T: FooBar>(x: T) {
	    x.method();
	}

	let x = 5u8;
    let y = "Hello".to_string();

	do_something(x);
    do_something(y);

    fn do_something2(x: &FooBar) {
	    x.method();
	}

	let x = 5u8;
    //casting
    do_something2(&x as &FooBar);
    //coercing
    do_something2(&x);

    let add = |x| x + 1;
    println!("{}", add(2));

    let mut num = 5;

	{
	    let mut add_num = |x: i32| num += x;

	    add_num(5);
	}

	assert_eq!(10, num);

	//move closure
	let mut num = 5;

	{
	    let mut add_num = move |x: i32| num += x;

	    add_num(5);
	}

	assert_eq!(5, num);

	fn call_with_one<F>(closure : F) -> i32
		where F: Fn(i32) -> i32{
			closure(1)
	}

	let answer = call_with_one(|x| x + 2);
	assert_eq!(3, answer);

	fn call_with_one2(some_closure: &Fn(i32) -> i32) -> i32 {
	    some_closure(1)
	}

	let answer = call_with_one2(&|x| x + 2);

	assert_eq!(3, answer);

	fn call_with_ref<F>(some_closure:F) -> i32
	    where F: for<'a> Fn(&'a i32) -> i32 {

	    let value = 0;
	    some_closure(&value)
	}

	fn add_one(i: i32) -> i32 {
	    i + 1
	}
	let f = add_one;

	call_with_one2(&f);

	fn factory() -> Box<Fn(i32) -> i32> {
	    let num = 5;

	    Box::new(move  |x| x + num)
	}

	let f = factory();

	let answer = f(1);
	assert_eq!(6, answer);

	trait Foo2 {
	    fn f(&self);
	}

	trait Bar2 {
	    fn f(&self);
	}

	struct Baz;

	impl Foo2 for Baz {
	    fn f(&self) { println!("Baz’s impl of Foo"); }
	}

	impl Bar2 for Baz {
	    fn f(&self) { println!("Baz’s impl of Bar"); }
	}

	let b = Baz;

	Foo2::f(&b);
	Bar2::f(&b);

	println!("Hello in English: {}", phrases::english::greetings::hello());
	println!("Hello in English: {}", hi());

	//inline, several memory address
	//better than static
	const TOTO: i32 = 12;

	//same address for all use
	static mut TOTO2: i32 = 12;
	unsafe {
		TOTO2 = 2;
	}

	#[test]
	fn check() {
	    assert_eq!(2, 1 + 1);
	}

	#[cfg(target_os = "macos")]
	mod macos_only {

	}

	type mytype = String;

	let s:mytype = "toto".to_string();	

	use std::result;

	enum ConcreteError {
	    Foo,
	    Bar,
	}

	type Result<T> = result::Result<T, ConcreteError>;

	let casty = TOTO as i64;

	use std::mem;

	unsafe {
		let a = [0u8, 1u8, 0u8, 0u8];
		let b = mem::transmute::<[u8; 4], u32>(a);
		println!("{}", b);
	}

	trait Graph {
	    type N;
	    type E;

	    fn has_edge(&self, &Self::N, &Self::N) -> bool;
	    fn edges(&self, &Self::N) -> Vec<Self::E>;
	}
	
	struct Node;

	struct Edge;

	struct MyGraph;

	impl Graph for MyGraph {
	    type N = Node;
	    type E = Edge;

	    fn has_edge(&self, n1: &Node, n2: &Node) -> bool {
	        true
	    }

	    fn edges(&self, n: &Node) -> Vec<Edge> {
	        Vec::new()
	    }
	}

	let graph = MyGraph;
	let obj = Box::new(graph) as Box<Graph<N=Node, E=Edge>>;

	struct FooUnsized<T: ?Sized> {
	    f: T,
	}

	fn testUnsized(){
		println!("unsized");
	}

	let mut fooUnsized = FooUnsized { f: testUnsized };

	use std::ops::Add;

	impl Add<i32> for Point {
	    type Output = f64;

	    fn add(self, rhs: i32) -> f64 {
	        // Add an i32 to a Point and get an f64.
	        50f64
	    }
	}

	let xa: f64 = point +  2;
	println!("{}", xa);

	use std::rc::Rc;

	fn borrow(s: &str) {
	    // Borrow a string for a second.
	}

	// String implements Deref<Target=str>.
	let owned = "Hello".to_string();
	let counted = Rc::new(owned);

	// Therefore, this works:
	borrow(&counted);

	macro_rules! macroTest {
	    (x => $e:expr) => (println!("mode X: {}", $e));
	    (y => $e:expr) => (println!("mode Y: {}", $e));
	}

	macroTest! (x=>3);

	macro_rules! macroTest2 {
	    (x=> $($e:expr),*) => {{
	    	let mut temp_vec = Vec::new();
	    	$(
	    		//println!("mode X: {}", $e)
	    		temp_vec.push($e);
	    	)*
	   	}};
	}

	macroTest2!(x=>[3,4]);

	let x: Option<i32> = None;

	match x {
	    Some(_) => unreachable!(),
	    None => println!("I know x is None!"),
	}

	let x = 5;
	let raw = &x as *const i32;

	let mut y = 10;
	let raw_mut = &mut y as *mut i32;

	let points_at = unsafe { *raw };
	println!("raw points at {}", points_at);

	unsafe{
		let ref_raw = &*raw;
	}
}
