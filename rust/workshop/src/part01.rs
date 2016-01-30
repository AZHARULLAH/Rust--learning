enum Positionornothing 
{
	Position(usize),
	Nothing
}

fn find_item(v:Vec<i32>, a:i32) -> Positionornothing
{
	for i in 0..v.len()-1
	{
		if a == v[i]
		{
			return Positionornothing :: Position(i);
		}
	}
	Positionornothing :: Nothing
}

pub fn main()
{
	let v = vec![5,6,7,8];
	let pos = find_item(v, 9);
	match pos
	{
		Positionornothing :: Position(e) => {println!("Position is {}",e);},
		Positionornothing :: Nothing => {println!("Position is not found");}
	}
	//println!("Position is {}",pos);
}
