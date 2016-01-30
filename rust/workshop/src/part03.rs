pub enum Somethingornothing<T>
{
	Something(T),
	Nothing
}

pub trait Print
{
	fn print(self);
}

trait Is
{
	fn is(self, o:Self) ->  bool;
}

impl Is for i32
{
	fn is(self, o:Self) -> bool
	{
		self == o 
		
	}
}

pub type Positionornothing = Somethingornothing<usize>;

impl Print for Positionornothing
{
	fn print(self)
	{
		match self
		{
			Something(e) => {println!("Position is {}",e);},
			Nothing => {println!("Position is not found");}	
		}
	}
}

use self::Somethingornothing :: {Something, Nothing};

pub fn find_item(v:Vec<i32>, a:i32) -> Positionornothing
{
	for i in 0..v.len()
	{
		if v[i].is(a)
		{
			return Something(i);
		}
	}
	Nothing
}

pub fn main()
{
	let v = vec![5,6,7,8];
	let pos = find_item(v, 8);
	pos.print();
}
