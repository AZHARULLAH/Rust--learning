enum Somethingornothing<T>
{
	Something(T),
	Nothing
}

type Positionornothing = Somethingornothing<usize>;

use self::Somethingornothing :: {Something, Nothing};

fn find_item(v:Vec<i32>, a:i32) -> Positionornothing
{
	for i in 0..v.len()-1
	{
		if a == v[i]
		{
			return Something(i);
		}
	}
	Nothing
}

pub fn main()
{
	let v = vec![5,6,7,8];
	let pos = find_item(v, 9);
	match pos
	{
		Something(e) => {println!("Position is {}",e);},
		Nothing => {println!("Position is not found");}
	}
	//println!("Position is {}",pos);
}
