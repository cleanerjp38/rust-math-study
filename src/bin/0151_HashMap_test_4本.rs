use std::collections::HashMap;

fn hashmap1() {
    let input: String = "apple banana apple cherry banana apple".to_string();
    let s: Vec<&str> = input.split_whitespace().collect();

    let mut count_map: HashMap<&str, usize> = HashMap::new();
    for &c in &s {
    //for c in input.split_whitespace {　でも良いらしい。余計な配列を作らなくて済む
        *count_map.entry(c).or_insert(0) += 1;
    }

    println!("{:?}", count_map);

}

fn hashmap2() {
    let data = vec![
    ("Dev", "Alice"),
    ("Sales", "Bob"),
    ("Dev", "Charlie"),
    ("HR", "Dave"),
    ("Sales", "Eve"),
    ];

    let mut group_map: HashMap<&str, Vec<&str>> = HashMap::new();

    for (dept, name) in data {
        //今回*group_mapでなくていいのは、push()というメソッドを通る時に、自動で参照が外れるから
        group_map
            .entry(dept)
            .or_insert(vec![])
            .push(name);
    }

    println!("{:?}", group_map);
}

fn hashmap3() {
    //arrayの詰め方、当てずっぽうで書いたけど当たってたみたいだ
    let shops = [
        [("notebook", 10), ("pen", 5)],
        [("pen", 15), ("eraser", 8)],
    ];

    let mut items_map: HashMap<&str, u32> = HashMap::new();

    //店の配列を一つ一つ取り出して、その一つの店についてforで回していく
    for shop in shops {
        for (item, count) in shop {
            *items_map.entry(item).or_insert(0) += count;
        }
    }

    println!("{:?}", items_map);
}

fn hashmap4() {
    let answers: [(&str, u32); 3] = [("Alice", 85), ("Bob", 92), ("Charlie", 78)];
    let mut answers_map: HashMap<&str, u32> = HashMap::new();

    for (name, score) in answers {
        answers_map.insert(name, score);
    }

    let  result = answers_map.iter().max_by_key(|&(_, score)| score).unwrap();
    
    println!("{:?}", result);
}

fn main() {
    hashmap1();
    hashmap2();
    hashmap3();
    hashmap4();
}
//0151_HashMap_test_4本