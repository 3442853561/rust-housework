// fn main() {
//     let v: [u8; 4] = [2, 3, 4, 42];
//     let mut it = v.iter().filter(|a| **a > 3);
//
//     let s = it.by_ref().fold(0, |acc, i| acc + i);
//     println!("{:?}", s);
//
//     let s: Vec<&u8> = it.collect();
//     // I expect it output [4, 42]
//     println!("{:?}", s); 
// }

fn main() {
    let v: [u8; 4] = [2, 3, 4, 42];
    let mut it = v.iter().filter(|a| **a > 3);

    let s: Vec<&u8> = it.by_ref().collect();
    let txt = format!("{:?}", s);
    let s = s.iter().fold(0u8, |acc, i| acc + **i);
    println!("{:?}", s);
    println!("{}", txt);
}
