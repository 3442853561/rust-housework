fn to_string<T: Into<i64>>(num: T) -> String {
    format!("{}", num.into())
}

fn main() {
   let foo = &*to_string(0);
   println!("{}", foo);
}
