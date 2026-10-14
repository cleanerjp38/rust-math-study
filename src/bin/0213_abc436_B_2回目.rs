use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut squere: Vec<Vec<u32>> = vec![vec![0; n]; n];
    let mut r = 0;
    let mut c = (n - 1) / 2;
    let mut k = 1;
    squere[r][c] = k;

    for _ in 0..((n * n) - 1) {
        k += 1;
        let r_next = (r + n - 1) % n;
        let c_next = (c + 1) % n;
        
        if squere[r_next][c_next] == 0 {
            squere[r_next][c_next] = k;
            r = r_next;
            c = c_next;
        } else {
            r = (r + 1) % n ;
            squere[r][c] = k;
        }
    }

    for row in squere {
        for i in row {
            print!("{} ", i);
        }
        println!("");
    }
}
//0213_abc436_B_2回目