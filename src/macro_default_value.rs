// One parameter of this macro has a default value
macro_rules! make_a_awesome_function {
    ($id: ident) => {
        fn $id() {
            println!("You called a awesome function named {}!", stringify!($id));
        }
    };
    () => {
        make_a_awesome_function!(foo);
    };
}

make_a_awesome_function!(bar);
make_a_awesome_function!();

fn main() {
    foo();
    bar();
}
