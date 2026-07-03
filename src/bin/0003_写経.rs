enum Notification{
    Email(String),
    SMS(String,String),
    None,
}

fn main(){
    let my_notice = Notification::SMS(
    String::from("090-1234-5768"),
    String::from("今から帰るぜ"),
    );
    
    match my_notice{
        Notification::Email(adress) => {
            println!("メール送信先{}", adress);
        }
        Notification::SMS(number, messege) => {
            println!("{}宛にSMS送信: {}", number, messege);
        }
        Notification::None => {
            println!("通知設定はオフです");
        }
    }
}//0003_写経