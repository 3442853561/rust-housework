extern crate regex;

fn get_base(s: &str) -> (u128, u32) {
    use regex::Regex;
    if Regex::new(r"\.").unwrap().is_match(format!("{}", s).trim()) {
        (
            Regex::new(r"(?P<a>\d*)\.(?P<b>\d*)")
                .unwrap()
                .replace_all(format!("{}", s).trim(), "$a$b")
                .parse()
                .unwrap(),
            format!("{}", s).trim().len() as u32
                - Regex::new(r"\.")
                    .unwrap()
                    .find(format!("{}", s).trim())
                    .unwrap()
                    .start() as u32
                - 1,
        )
    } else {
        (format!("{}", s).trim().parse().unwrap(), 0)
    }
}

fn str_to_u32(s: &str) -> u32 {
    format!("{}", s).trim().parse().unwrap()
}

fn to_u128(s: String) -> u128 {
    use regex::Regex;
    let m = Regex::new(r"u128e").unwrap().find(&*s).unwrap();
    let (base, unpow) = get_base(&s[0..m.start()]);
    base * 10u128.pow(str_to_u32(&s[m.end()..])) / 10u128.pow(unpow)
}

macro_rules! numero_entero {
    (let $numero:ident = $texto:tt) => {
        let $numero = to_u128(stringify!($texto).to_string());
    };
    (let mut $numero:ident = $texto:tt) => {
        let mut $numero = to_u128(stringify!($texto).to_string());
    };
    ($numero:ident = $texto:tt) => {
        $numero = to_u128(stringify!($texto).to_string());
    };
}

fn main() {
    numero_entero!(let foo = 1.1u128e18);
    println!("{}", foo);
    numero_entero!(let mut bar = 2u128e18);
    println!("{}", bar);
    numero_entero!(bar = 3.2u128e5);
    println!("{}", bar);
}
