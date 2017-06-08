macro_rules! deconstruction {
    ($id:ident, $t:pat, $($p: expr),*) => {
        let mut bar = Vec::new(); 
        for i in $id {
            match i {
                $t => {$(bar.push($p);)*},
                _ => {},
            }
        }
        let mut $id = bar.clone();
    }
}

fn main() {
    let foo = vec![(1,((1,2),(3,4))),(1,((1,2),(3,4)))];
    deconstruction!(foo,(e,((f,g),(h,j))),e,f,g,h,j);
    let m = vec![vec![1], vec![2, 3], vec![4, 5, 6], vec![7, 8, 9, 10]];
    for i in m {
        for j in i {
            foo.push(j);
        }
    }
    for i in foo {
        print!("{} ", i);
    }
}
