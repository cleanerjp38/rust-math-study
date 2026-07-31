enum Notification{
    Email(String),
    SMS(String, String),
    None
}

struct Notifier;

impl Notifier{
    fn perform(&self, notice: Notification){
        match notice{
        Notification::Email(address) => println!("メール送信先: {}", address),
        Notification::SMS(number, message) => println!("{}宛にSMS送信: {}", number, message),
        Notification::None => println!("通知設定はオフです"),
        }
    }
}

fn get_my_notice() -> Notification{
    Notification::SMS(
        String::from("090-1234-5678"),
        String::from("今から帰るぜ"),
    )
}

fn main(){
    let notice = get_my_notice();
    let worker = Notifier;
    
    worker.perform(notice);
}
//0010_写経_0003の修正