use std::fmt::Debug;

#[derive(PartialEq, Eq)]
struct Variant {
    variant: String,
}

trait GetVariant where Self: Debug {
    fn get_variant(&self) -> Variant {
        let this_variant = format!("{:?}", self);
        let mut result = "".to_string();
        for i in this_variant.chars() {
            if i == '(' {
                break;
            }
            result.push(i);
        }
        Variant {
            variant: result,
        }
    }
}

impl<T: Debug> GetVariant for Option<T> {}

fn main() {
    println!("{:?}", Some(2));
    println!("{}", Some(1).get_variant() == Some(2).get_variant() );
}
