struct Chinko{
    stage: Status
}

#[derive(Debug)]
enum Status {
    Normal,
    Elect,
    Sage,
}

impl Chinko {
    fn new() -> Self{
        println!("お前はここにいる");
        Self {
            stage: Status::Normal,
        }
    }

    fn status(&self) {
        println!("いつも通り {:?} だ", self.stage);
    }

    fn baloon(&mut self) {
        match self.stage {
            Status::Normal => {
                self.stage = Status::Elect;
                println!("俺のドリルが天を突く！");
            }
            Status::Elect => {
                self.stage = Status::Sage;
                println!("燃え尽きたぜ、真っ白にな…");
            }
            Status::Sage => {
                println!("今は賢者タイムだ。そっとしておいてくれ。");
            }
        }
    }
}




fn main(){
    let mut  my_spirit = Chinko::new();

    my_spirit.status();
    my_spirit.baloon();
    my_spirit.status();
    my_spirit.baloon();
    my_spirit.status();

}
//0013_ちんこのコード