use std::io;


//----数値の型を全てu32でやっていたので計算が合わなくなっていた
//fn get_hw() -> (u32, u32) {
fn get_hw() -> (f32, f32) {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut hw = buf.trim().split_whitespace();
    //let h: u32 = hw.next().unwrap().parse().unwrap();
    //let w: u32 = hw.next().unwrap().parse().unwrap();
    let h: f32 = hw.next().unwrap().parse().unwrap();
    let w: f32 = hw.next().unwrap().parse().unwrap();


    (h, w)
}

//fn obesity(h: u32, w: u32) {
fn obesity(h: f32, w: f32) {
    //ここでhがu32だと、小数点が切り捨てられてしまう
    let h_m = h / 100.0;
    //let bmi = w / h_m / h_m;
    let bmi = w / (h_m * h_m);
    if bmi >= 25.0 {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn main() {
    let (h, w) = get_hw();
    obesity(h, w);
}
//0115_abc467_A