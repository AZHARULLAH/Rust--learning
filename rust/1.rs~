fn add(a:i32, b:i32) -> i32
{
	a + b
}


fn acc(a:i32) -> i32
{
	let mut total = 0;
	for i in 1..a+1
	{
		total += i;
	}
	total
}

fn max(a:i32, b:i32) -> i32
{
	if a > b 
	{
		a
	}
	else 
	{
		b
	}
}

fn combine (a: i32, b:i32, o:&str) -> i32
{
	match o
	{
		"+" => a+b,
		"-" => a-b,
		"*" => a*b,
		_ => 0
	}
}

fn main()
{
	println!("Hello");
	let s = add(1, 10);
	println!("Sum is {}",s); 
	
	let s1 = acc(5);
	println!("Acc is {}",s1);
	
	let g = max(33,4);
	println!("Max is {}",g);
	
	let h = combine(2, 3, "+");
	let i = combine(2, 3, "-");
	let j = combine(2, 3, "*");
	println!("Combination is {},{},{}",h,i,j);
}
