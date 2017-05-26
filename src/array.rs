macro_rules! arr {
	($type: ty, $value: expr, $long: expr) => {
		Arr::new([$value as $type; $long]) as Arr<$type, [$type; $long]>;
	}
}

macro_rules! gethead {
	($self:ident, $index: expr) => {
		$self.head[$index]
	}
}

macro_rules! headlen {
	($self:ident) => {
		$self.head.len()
	}
}

macro_rules! get {
	($self:ident, $index: expr) => {
		if $index < headlen!($self) {
			gethead!($self, $index)
		} else if $index < headlen!($self) + $self.follow.len() {
			$self.follow[$index - headlen!($self)]
		} else {
			panic!("Cross-border visit");
		}
	}
}

macro_rules! append {
	($self:ident, $value: expr) => {
		$self.follow.push($value);
	}
}

struct Arr<T, A> {
	head: A,
	follow: Vec<T>,
}

impl<T, A> Arr<T, A> {
	fn new(value: A) -> Self {
		Arr {
			head: value,
			follow: Vec::new(),
		}
	}
}


fn main() {
	let mut foo = arr!(u8, 1, 2);
	println!("{}", get!(foo, 1));
	append!(foo, 2);
	println!("{}", get!(foo, 2));
	println!("{}", get!(foo, 3));
}
