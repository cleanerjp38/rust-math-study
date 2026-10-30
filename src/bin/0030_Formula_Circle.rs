struct Point {

    x: f64,

    y: f64,

}

struct Circle {

    center: Point,

    radius: f64,

}

impl Circle {

    fn contain(&self, p: &Point) -> bool {

        let dx = p.x - self.center.x;

        let dy = p.y - self.center.y;

        let distance_sq = dx * dx + dy * dy;

        let radius_sq = self.radius * self.radius;

        (distance_sq - radius_sq).abs() < 1e-9//指数表記

    }

}


fn main() {

    let my_circle = Circle{

        center: Point { x: 0.0, y: 0.0 },

        radius: 5.0,

    };

    println!("中心(0,0)、半径5の円を構築。");

    println!("点(3,4)はこの円の境界上に存在するか？");

    let target = Point {x: 3.0, y: 4.0 };//Pointに二回数値を入れられているのは、struct時点でPoint構造体をパクれているから

    if my_circle.contain(&target) {

        println!("→［事実］境界上に存在します。論理的欠陥はありません。");

    }else{

        println!("→［事実］存在しません。");

    }

}

//0030_Formula_Circle