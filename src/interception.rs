use std::convert::AsMut;
trait Trait {
	fn take(&mut self, start: usize, end: usize);
}

impl<T> Trait for Vec<T> {
	fn take(&mut self, start: usize, end: usize){
		let cut = self.len() - end - 1;
		for _ in 0..cut {
			self.pop();
		}
		let mut foo: Self = Vec::new();
		for _ in start..end + 1 {
			foo.push(self.pop().unwrap());
		}
		*self = Vec::new();
		for _ in 0..foo.len() {
			self.push(foo.pop().unwrap());
		}
	}
}

fn avaricious<T>(foo: &mut Vec<T>, start_token: T, end_token: T)
where T: PartialEq {
	let mut start = 0;
	loop {
		if foo[start] == start_token {
			break;
		}
		start += 1;
	}
	let mut end = foo.len() - 1;
	loop {
		if foo[end] == end_token {
			break;
		}
		end -= 1;
	}

	foo.take(start, end);
}

fn header<T>(foo: &mut Vec<T>, start_token: T, end_token: T)
where T: PartialEq {
	let mut start = 0;
	loop {
		if foo[start] == start_token {
			break;
		}
		start += 1;
	}
	let mut end = start + 1;
	loop {
		if foo[end] == end_token {
			break;
		}
		end += 1;
	}

	foo.take(start, end);
}

fn footer<T>(foo: &mut Vec<T>, start_token: T, end_token: T)
where T: PartialEq {
	let mut end = foo.len() - 1;
	loop {
		if foo[end] == end_token {
			break;
		}
		end -= 1;
	}
	let mut start = end - 1;
	loop {
		if foo[start] == start_token {
			break;
		}
		start -= 1;
	}
	foo.take(start, end);
}

fn main() {
	let mut foo: Vec<u8> = vec![0, 2, 1, 0, 3, 1];
	let mut bar: Vec<u8> = vec![0, 2, 1, 0, 3, 1];
	avaricious(foo.as_mut(), 0, 1);
	println!("{:?}", foo);
	header(foo.as_mut(), 0, 1);
	println!("{:?}", foo);
	footer(bar.as_mut(), 0, 1);
	println!("{:?}", bar);
}