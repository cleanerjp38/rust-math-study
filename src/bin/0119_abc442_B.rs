use std::io;

fn music_player() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let q: u32 = buf.trim().parse().unwrap();

    let mut vol: i32 = 0;
    let mut sound = false; 
    for _ in 0..q {
        let mut buf_a = String::new();
        io::stdin().read_line(&mut buf_a).unwrap();
        let a_i: i32 = buf_a.trim().parse().unwrap();

        match a_i {
            1 => {
                vol += 1;
            }
            2 => {
                if vol >= 1 {
                    vol -= 1;
                }
            }
            3 => {
                if sound == false {
                    sound = true;
                } else {
                    sound = false;
                }
            }
            _ => ()
        }
        if vol >= 3 && sound == true {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn main() {
    music_player();
}
//0119_abc442_B