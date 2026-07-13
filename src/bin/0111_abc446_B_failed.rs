use std::io::{self, BufRead};

struct Drink {
    juice: Vec<u32>,
    kyaku: u32,
}

impl Drink {
    fn kubaru() {
        let std = io::stdin();
        let buf = io::BufReader::new(std.lock());
        let mut line = buf.lines();
        let line_1 = line.next().expect("INPUT NULL").expect("INPUT Error");

        let mut nm = line_1.trim().split_whitespace();
        let n: u32 = nm.next().unwrap().parse().unwrap();

        let mut juice: Vec<u32> = Vec::new();
        for i in 0..n {
            let line_i = line.next().expect("INPUT NULL").expect("INPUT Error");
            let kyaku: u32 = line_i.trim().parse().unwrap();
            let line_i2 = line.next().expect("INPUT NULL").expect("INPUT Error");
            let mut hoshii: Vec<u32> = line_i2.split_whitespace().map(|j| j.parse().unwrap()).collect();
            
            if kyaku == hoshii.iter().filter(|&h| hoshii.contains(h)){

            }
        }
    }
}
//0111_abc446_B_failed