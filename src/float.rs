fn main() {
    let mut sum = 0;
    let bit_exp: u32 = 4;
    let bit_frac: u32 = 3;
    for exp in 0..2u64.pow(bit_exp) - 1 {
        for frac in 0..2u64.pow(bit_frac) {
            let mut e: f64 = exp as f64;
            let mut m: f64 = 1.0;
            if exp == 0 {
                e = 1.0;
                m = 0.0;
            }
            e -= 2.0f64.powf(bit_exp as f64 - 1.0) - 1.0;
            let mut th = frac;
            for i in 0..bit_frac {
                m += (th % 2) as f64 * 2.0f64.powf(i as f64 - bit_frac as f64);
                th /= 2;
            }
            let foo: f64 = m * 2.0f64.powf(e);
            println!("{}", foo);
            if foo < 1.0 {
                sum += 1;
            }
        }
    }
    println!("{}", sum);
}
