fn main() {
    let v = [4, 5, -3, 0, 7];
    let w = &v[2..4];
    println!("{:?}", w);
    let x = &v[2..];
    println!("{:?}",x);
    let y = &v[..4];
    println!("{:?}",y);
}