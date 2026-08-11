fn for1() {
    let name: Vec<char> = "luminol".to_string().chars().collect();
    let first_char = name[0];
    let mut end_char = name[0];

    for n in name {
       end_char = n;
    }

    if first_char == end_char {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn iter1() {
    let name: String = "luminol".to_string();

    let first = name.chars().next().unwrap();
    let last = name.chars().last().unwrap();

    if first == last {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn for2() {
    let name: Vec<char> = "aaabbbccc".to_string().chars().collect();
    let first_char = name[0];

    let mut count = 0;
    for n in name {
        let end_char = n;

        if first_char == end_char {
            count += 1;
        }
    }
    println!("{}", count);
}

fn iter2() {
    let name: String = "aaabbbccc".to_string();
    let first_char = name.chars().next().unwrap();

    let count = name
        .chars()
        //.map(|s| s == first_char) これだと判定回数をカウントしてしまう。つまり、　trueもfalseもカウントしてしまう
        .filter(|&s| s == first_char)
        .count();

        println!("{}", count);
}

fn for3() {
    let st: String = "iiateeaejjaajjaa".to_string();
    let st_vec: Vec<char> = st.chars().collect();

    let mut count = 0;
    for i in st_vec {
        match i {
            'i' | 'j' => count += 1,
            _         => (),
        }
    }

    println!("{}", count);
}

fn iter3() {
    let st: String = "iiateeaejjaajjaa".to_string();
    
    let count: u32 = st
        .chars()
        .map(|s| match s {
        'i' | 'j' => 1,
        _         => 0,
        })
        .sum();

    println!("{}", count);
}


fn main() {
    for1();
    iter1();
    for2();
    iter2();
    for3();
    iter3();
}
//0140_for_→_iter_3本