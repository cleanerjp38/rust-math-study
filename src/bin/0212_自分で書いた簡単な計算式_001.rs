fn main() {
    let milk = (200.0, 35.0);
    let soy = (182.0, 36.0);
    let protein = (5480.0, 750.0);

    let milk_cost = milk.0 / milk.1;
    let soy_cost = soy.0 / soy.1;
    let protein_cost = protein.0 / protein.1;

    println!("牛乳: {:.2}円/g", milk_cost);
    println!("大豆: {:.2}円/g", soy_cost);
    println!("プロテイン: {:.2}円/g", protein_cost);

    let mut ans:f32 = f32::MAX;
    for cost in [milk_cost, soy_cost, protein_cost] {
        ans = ans.min(cost);
    }
    println!("最安単価: {:.2}円/g", ans);
}
//0212_自分で書いた簡単な計算式_001