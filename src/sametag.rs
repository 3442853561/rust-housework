trait SameTag<T> {
    fn same_tag(&self, other: &Self) -> bool;
}

impl<T> SameTag<T> for Option<T> {
    #[allow(unused_assignments)]
    fn same_tag(&self, other: &Self) -> bool {
        let mut self_tag = 0;
        let mut other_tag = 0;
        match *self {  
            Some(_) => self_tag = 0,
            None => self_tag = 1,
        }
        match *other {  
            Some(_) => other_tag = 0,
            None => other_tag = 1,
        }
        self_tag == other_tag
    }
}

fn main() {
    println!("{}", Some(1).same_tag(&Some(2)));
}

// use std::mem;
// enum Foo { A(&'static str), B(i32), C(i32) }
// assert!(mem::discriminant(&Foo::A("bar")) == mem::discriminant(&Foo::A("baz")));
// assert!(mem::discriminant(&Foo::B(1))     == mem::discriminant(&Foo::B(2)));
// assert!(mem::discriminant(&Foo::B(3))     != mem::discriminant(&Foo::C(3)));
