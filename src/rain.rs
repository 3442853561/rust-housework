fn main() {
    let foo = [2, 5, 1, 2, 3, 4, 7, 7, 6];
    let mut left = 0;
    let mut right = foo.len() - 1;
    let mut i = 0;
    let mut lim = 0;
    let mut sum = 0;
    while i < foo.len() {
        if foo[left] > foo[i] {
            break;
        } else {
            left = i;
        }
        i += 1;
    }
    i = foo.len() - 1;
    while i > 0 {
        if foo[right] > foo[i] {
            break;
        } else {
            right = i;
        }
        i -= 1;
    }
    if foo[left] < foo[right] {
        lim = foo[left];
    } else {
        lim = foo[right];
    }
    for i in left..right + 1 {
        if foo[i] < lim {
            sum += lim - foo[i];
        }
    }
    println!("{}", sum);
}
