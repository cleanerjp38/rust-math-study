enum Nortification {
    Email(String),
    SMS(String, String),
    None,
}

impl Nortification {
    fn send(&self) {
        match self {
            Nortification::Email(adress) => {
                println!("メール送信先{}", adress);
            }
            Nortification::SMS(number, messege) => {
                println!("{}宛にSMS送信:{}", number, messege);
            }
            Nortification::None => {
                println!("通知設定はオフです");
            }
        }
    }
}

fn main() {
    let mut my_notice = Nortification::SMS(
        String::from("11111111"),
        String::from("今から帰るよ"),
    );
    my_notice.send();

    my_notice =
        Nortification::Email(String::from("エル・プサイ・コングルゥ"));
    my_notice.send();

    my_notice = Nortification::None;
    my_notice.send();
}
//0029_Remake0003