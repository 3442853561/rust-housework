macro_rules! numero_entero {
    ($numero:tt u128e $potenciacion:tt) => {
        $numero * 10u128.pow($potenciacion)
    }
}

#[inline]
fn numero(num: u128, pot: u32) -> u128 {
    // Me gusta
    num * 10u128.pow(pot)
}

fn main() {
    let foo = numero_entero!(1 u128e 18);
    println!("{}", foo);
    println!("{}", numero(1, 18));
}
