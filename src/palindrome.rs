trait Trait {
	fn to_letters(&self) -> String;
	fn palindrome(&self) -> bool;
	fn lazy_output(&self);
	
	fn mid(&self) -> Option<char> ;
	fn lazy_mid(&self) -> char;
}

impl Trait for str {
	fn to_letters(&self) -> String {
		let mut bar:Vec<u8> = Vec::new();
		for ch in self.chars() {
			match ch {
				'a'...'z' => bar.push(ch as u8),
				'A'...'Z' => bar.push(ch as u8 + 97 - 65),
				_ => {},
			}
		}
		String::from_utf8(bar).unwrap()
	}

	fn palindrome(&self) -> bool {
		let mut i = 0;
		while i < self.len() - i - 1 {
			if self.chars().nth(i) != self.chars().nth(self.len() - i - 1) {
				return false;
			}
			i += 1;
		}
		true
	}

	fn lazy_output(&self) {
		if self.to_letters().palindrome() {
			println!("{:?} is palindrome.", self);
		} else {
			println!("{:?} is not palindrome.", self);
		}
	}
	
	fn mid(&self) -> Option<char> {
		if self.len() % 2 == 0 {
			None
		}  else {
			self.chars().nth(self.len()/2)
		}
	}
	
	fn lazy_mid(&self) -> char {
		self.to_letters().mid().unwrap()
	}
}

fn main() {
	let foo = "A man, a plan, a cat, a canal, Panama.";
	foo.lazy_output();
	println!("{:?}", foo.lazy_mid());
}