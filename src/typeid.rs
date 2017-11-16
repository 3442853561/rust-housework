use std::any::TypeId;
fn string() -> TypeId {
    TypeId::of::<String>()
}

fn foo(para_type: TypeId) {
    if TypeId::of::<String>() == para_type {
        println!("参数为类型 String");
    }
}

fn main() {
    foo(string());
}
