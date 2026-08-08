use std::io;

fn get_array() -> [u32; 6] {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut line = input.trim().split_whitespace();
    let array = [
        line.next().unwrap().parse::<u32>().unwrap(),
        line.next().unwrap().parse::<u32>().unwrap(),
        line.next().unwrap().parse::<u32>().unwrap(),
        line.next().unwrap().parse::<u32>().unwrap(),
        line.next().unwrap().parse::<u32>().unwrap(),
        line.next().unwrap().parse::<u32>().unwrap(),
    ];

    array
}

fn get_dice() -> [[u32; 6]; 3] {
    [
        get_array(),
        get_array(),
        get_array(),
    ]
}

fn sort_456() -> f64 {
    let dices = get_dice();
    let mut count = 0.0;

    for d1 in dices[0].iter() {
        for d2 in dices[1].iter() {
            for d3 in dices[2].iter() {
                let mut combination = [d1, d2, d3];
                combination.sort();

                match combination {
                    [4, 5, 6] => count += 1.0,
                    _     => (),
                }
            }
        }
    }

    let all_possibility = (6 * 6 * 6) as f64;

    count / all_possibility
}

fn main() {
    println!("{}", sort_456());
}
//0137_abc456_B_2回目