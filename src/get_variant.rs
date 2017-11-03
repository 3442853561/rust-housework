use std::fmt::Debug;

#[derive(PartialEq, Eq)]
struct Variant {
    variant: String,
}

trait GetVariant<T> {
    fn get_variant(&self) -> Variant;
}

impl<T: Debug> GetVariant<T> for Option<T> {
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

fn main() {
    println!("{:?}", Some(2));
    println!("{}", Some(1).get_variant() == Some(2).get_variant() );
}
