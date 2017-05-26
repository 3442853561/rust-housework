fn read_clone<T>(read: &mut Vec<T>, start: usize, end: usize) -> Vec<T> 
where T: Clone {
	let mut el_read: Vec<T> = Vec::new();
	let mut el: Option<T> = None; 
	let mut foo: Vec<T> = Vec::new();
	for _ in 0..read.len() - start {
		el_read.push(read.pop().unwrap());
	}
	for _ in start..end + 1 {
		el = el_read.pop();
		foo.push(el.clone().unwrap());
		read.push(el.clone().unwrap());
	}
	for _ in 0..el_read.len() {
		read.push(el_read.pop().unwrap());
	}
	foo
}

fn lazy_output<T>(read:&Vec<T>) 
where T: std::fmt::Display {
	for i in read {
		print!("{} ", i);
	}
	println!("");
}

fn main() {
	let mut foo = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
	let bar = read_clone(&mut foo, 1, 3);
	lazy_output(&foo);
	lazy_output(&bar);
}