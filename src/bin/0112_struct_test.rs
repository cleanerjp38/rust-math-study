struct Student {
    id: u32,
    name: String,
}

struct HairetsuTest {
    //これでは生徒1人分の名簿にしかならない
    //meibo: Student, 
    meibo: Vec<Student>,
}

impl HairetsuTest {
    //まっさらな名簿を準備
    fn new() -> Self {
        Self { meibo:Vec::new() }
    }

    //meiboの中に構造体を入れていく
    fn add_student(&mut self, student: Student) {
        self.meibo.push(student);
    }
}

fn main() {
    let mut my_test = HairetsuTest::new();

    let student1 = Student {
        id: 1,
        name: String::from("Taro"),
    };

    let student2 = Student {
        id: 2,
        name: String::from("Jiro"),
    };

    my_test.add_student(student1);
    my_test.add_student(student2);
    println!("生徒数:{}",my_test.meibo.len());
    
    //for構文で、meiboの中にある構造体を参照できる
    for i in &my_test.meibo {
        println!("{}, {}", i.id, i.name);
    }
}
//0112_struct_test