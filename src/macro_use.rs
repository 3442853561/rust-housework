#[macro_use]
mod bar {
    macro_rules! macro0 { () => (()) }
}


mod foo {
    macro_rules! macro1 { () => (()) }
}

fn main() {
    macro0!();
    // macro1! 是不能用的
}
