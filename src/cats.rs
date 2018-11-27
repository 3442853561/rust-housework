#[derive(Clone, Debug)]
struct Cat {
    fuckable: bool,
}
type Cats = Vec<Cat>;

impl Cat {
    pub fn fuck(&self) -> Cats {
        if self.fuckable {
            return vec![
                Cat { fuckable: true },
                Cat { fuckable: false },
                Cat { fuckable: true },
                Cat { fuckable: true },
                Cat { fuckable: false },
            ];
        }
        vec![]
    }
    pub fn cut(&mut self) {
        self.fuckable = false;
    }
}

fn main() {
    let mut cat_of_fat_worship_guy = Cat { fuckable: true };
    if !cat_of_fat_worship_guy.fuck().is_empty() {
        cat_of_fat_worship_guy.cut();
    }
    let kids_of_cat_of_fat_worship_guy = cat_of_fat_worship_guy.fuck();
    {
        let mut temp = kids_of_cat_of_fat_worship_guy.iter();
        println!("{:?}", temp.next());
    }
}
