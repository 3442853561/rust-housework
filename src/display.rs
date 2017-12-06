use std::fmt;

struct Example(f32, f32, f32, f32);

impl fmt::Display for Example {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.0, self.1, self.2, self.3)
    }
}

fn main() {
    let foo = Example(0.0, 0.1, 0.2, 0.3);
    println!("{}", foo);
}
