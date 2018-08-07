trait Inputable {
    fn input(arg: impl Into<String>) -> Self;
}

impl Inputable for i32 {
    fn input(arg: impl Into<String>) -> Self {
        arg.into().trim().parse().unwrap()
    }
}

macro_rules! leer_teclado {
    ($arg: tt: $type: ty) => {
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);
        let $arg: $type = Inputable::input(input);
    };
    ($arg: tt) => {
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);
        let $arg = input;
    };
}

fn main() {
    leer_teclado!(a);
    println!("{}", a);
    leer_teclado!(a: i32);
    println!("{}", a);
}
