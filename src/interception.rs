trait Trait {
	fn cut(&mut self, long: usize) -> Self;
	fn take(&mut self, start: usize, end: usize) -> Self;
	fn flashback(&mut self) -> Self;
}

impl<T> Trait for Vec<T> {
	fn cut(&mut self, long: usize) -> Self {
		let mut foo: Self = Vec::new();
		for _ in 0..long {
			foo.push(self.pop().unwrap());
		}
		foo
	}
	fn take(&mut self, start: usize, end: usize) -> Self {
		let cut = self.len() - end - 1;
		self.cut(cut);
		let mut foo: Self = Vec::new();
		for _ in start..end + 1 {
			foo.push(self.pop().unwrap());
		}
		foo
	}
	fn flashback(&mut self) -> Self {
		let mut foo: Self = Vec::new();
		for _ in 0..self.len() {
			foo.push(self.pop().unwrap());
		}
		foo
	}
}

fn main() {
	let mut foo: Vec<u8> = vec![0, 2, 1, 0, 3, 1];
	let mut end = foo.len() - 1;
	loop {
		if foo[end] == 1 {
			break;
		}
		end -= 1;
	}
	let mut start = end;
	loop {
		start -= 1;
		if foo[start] == 0 {
			break;
		}
		
	}
	println!("{:?}", foo.take(start, end).flashback());
	
}