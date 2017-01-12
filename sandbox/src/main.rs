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
}

//function
fn add(x :i32, y :i32) -> i32{
	x + y
}

fn diverges() -> ! {
    panic!("This function never returns!");
}
