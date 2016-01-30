fn find_item(v:Vec<i32>, a:i32) -> usize
{
	for i in 0..v.len()-1
	{
		if a == v[i]
		{
			return i;
		}
	}
	0
}

pub fn main()
{
	let v = vec![5,6,7,8];
	let pos = find_item(v, 7);
	println!("Position is {}",pos);
}
