#[derive(Clone)]
struct Fucker {
    name: String,
}

impl Fucker {
    fn new<T: Into<String>>(name: T) -> Self {
        Fucker { name: name.into() }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn set_name<T: Into<String>>(&mut self, new_name: T) {
        self.name = new_name.into();
    }
}

struct BillyHerrington {
    extends: Fucker,
    philosophy: bool,
}

impl BillyHerrington {
    fn new(philosophy: bool) -> Self {
        BillyHerrington {
            extends: Fucker::new("Billy Herrington"),
            philosophy: philosophy,
        }
    }

    fn get_extends(&self) -> Fucker {
        self.extends.clone()
    }

    fn set_extends(&mut self, updata_extends: Fucker) {
        self.extends = updata_extends;
    }

    fn ref_mut_extends<'a>(&'a mut self) -> &'a mut Fucker {
        &mut self.extends
    }
}

fn main() {
    let mut my_king = BillyHerrington::new(true);
    my_king.ref_mut_extends().set_name("King of Fucker");
    println!(
        "Billy Herrington is {}",
        my_king.ref_mut_extends().get_name()
    );
}
