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
