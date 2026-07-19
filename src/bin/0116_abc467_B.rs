use std::io::{self, BufRead};

//標準入力をread_line()で受けたからなのか、Runをしたらエラーが出た
//fn charges2() -> u32 {
    //let mut buf = String::new();
    //io::stdin().read_line(&mut buf).unwrap();
    //let n: u32 = buf.trim().parse().unwrap();

    //let mut count = 0;
    //for _ in 0..n {
        //let mut line = buf.trim().split_whitespace();
        //let a: u32 = line.next().unwrap().parse().unwrap();
        //let b: u32 = line.next().unwrap().parse().unwrap();
        //let t_k  = line.next().unwrap();

        //match t_k {
            //"take" => {
                //count += 0;
            //}
            //"keep" => {
                //count += b -a;
            //}
            //_      => (),
        //}
    //}
    //count
//} 

//標準入力をBufReadにしたら上手くいった
//入力を一気に受け取る必要があったからか？
fn charges() -> u32 {
    let stdin = io::stdin();
    let buf = io::BufReader::new(stdin.lock());
    let mut lines = buf.lines();
    let first_line = lines.next().unwrap().unwrap();
    let n: u32 = first_line.trim().parse().unwrap();

    let mut count = 0;
    for _ in 0..n {
        let next_line = lines.next().unwrap().unwrap();
        let mut abtk = next_line.trim().split_whitespace();
        let a: u32 = abtk.next().unwrap().parse().unwrap();
        let b: u32 = abtk.next().unwrap().parse().unwrap();
        //ここで&strを受け取る方法を試行錯誤していたら、この書き方で受け取れた
        let t_k  = abtk.next().unwrap();

        match t_k {
            "take" => {
                count += 0;
            }
            "keep" => {
                count += b -a;
            }
            _      => (),
        }
    }
    count
} 

fn main() {
    println!("{}", charges());
}
//0116_abc467_B