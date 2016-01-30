use part03::Somethingornothing::*;
use part03::{Somethingornothing,Print};

type Numberornothing = Somethingornothing<i32>;

trait Maximum
{
	fn max(self, o:Self) -> Self;
}

impl Maximum for i32
{
	fn max(self, o:Self) -> Self
	{
		if self > o
		{
			self
		}
		else
		{
			o
		}
	}
}

impl Maximum for f32
{
	fn max(self, o:Self) -> Self
	{
		if self > o
		{
			self
		}
		else
		{
			o
		}
	}
}

impl Print for Numberornothing
{
	fn print(self)
	{
		match self
		{
			Something(e) => {println!("Number {}",e);},
			Nothing => {println!("Nothing");}
		}
	}
}


fn find_max_generic<T:Maximum>(v:Vec<T>) -> Somethingornothing<T>
{
	let mut max = Nothing;
	for i in v
	{
		max = Something(match max 
		{
			Something(e) => i.max(e),
			Nothing => i
		});
	}
	max
}

pub fn main()
{
	let v = vec![5,6,7,9];
	let m = find_max_generic(v);
	m.print();
}
