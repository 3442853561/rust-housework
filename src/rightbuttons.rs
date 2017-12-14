use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use std::io;

fn read_from_stdin(buf: &mut String) -> io::Result<()> {
    try!(io::stdin().read_line(buf));
    Ok(())
}

fn bar(input: &mut String) -> std::io::Result<()> {
	let foo = include_str!(file!{});;
	let mut file = File::create(input)?;
	file.write_all(foo.as_bytes())?;
	Ok(())
}

fn main() {
	loop {
		let mut input = String::new();
		read_from_stdin(&mut input);
		if input.trim() == "#".to_string() {
			break;
		}
		input = input.trim().to_string() + ".rs";
		let temp = "rustc ".to_string() + &*input;
		let _ = bar(&mut input);
		let output = if cfg!(target_os = "windows") {
			Command::new("cmd")
					.args(&["/C", &*temp])
					.output()
					.expect("failed to execute process")
		} else {
			Command::new("sh")
					.arg("-c")
					.arg(&*temp)
					.output()
					.expect("failed to execute process")
		};
		println!("You know, boys, a nuclear reactor is a lot like a woman. You just have to read the manual and press the right buttons. (by Homer Simpson)");
	}
}
