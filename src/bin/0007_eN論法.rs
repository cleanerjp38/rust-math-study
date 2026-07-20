use std::io::{self, Write};

// ==========================================
// Step 2: 「関所（Gateway）」の構築
// ==========================================
struct InputGateway;

impl InputGateway {
    /// 外界から「正の実数イプシロン」を安全にデリバリーする
    fn get_epsilon() -> f64 {
        loop {
            print!("許容誤差 ε (0 < ε < 1) を入力してください: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("読み込み失敗");

            match input.trim().parse::<f64>() {
                Ok(e) if e > 0.0 && e < 1.0 => return e,
                _ => println!("【Error】0より大きく1より小さい正の実数を入力せよ。"),
            }
        }
    }
}

// ==========================================
// Step 3: struct と impl による「物語」の記述
// ==========================================

/// 数列 a_n = 1/n を表す構造体
struct HarmonicSequence;

impl HarmonicSequence {
    /// 第n項の値を計算する
    fn value_at(&self, n: u64) -> f64 {
        if n == 0 { return 0.0; } // 定義域は自然数
        1.0 / (n as f64)
    }
}

/// 収束の検証を行う構造体
struct ConvergenceVerifier {
    sequence: HarmonicSequence,
    limit: f64,
    epsilon: f64,
}

/// 検証結果という「状態」を定義
struct ConvergenceReport {
    threshold_n: u64,
    final_value: f64,
    difference: f64,
}

impl ConvergenceVerifier {
    /// 新しい検証器を生成
    fn new(epsilon: f64) -> Self {
        Self {
            sequence: HarmonicSequence,
            limit: 0.0, // 今回は 0 への収束を検証
            epsilon,
        }
    }

    /// ε-N論法の定義に基づき、|a_n - L| < ε を満たす最小の N を探索する
    fn find_minimum_n(&self) -> ConvergenceReport {
        let mut n = 1;
        loop {
            let a_n = self.sequence.value_at(n);
            let diff = (a_n - self.limit).abs();

            if diff < self.epsilon {
                return ConvergenceReport {
                    threshold_n: n,
                    final_value: a_n,
                    difference: diff,
                };
            }
            n += 1;
        }
    }
}

// ==========================================
// main：外界の泥を排除した「聖域」
// ==========================================
fn main() {
    // 関所から純粋な型をデリバリー
    let eps = InputGateway::get_epsilon();

    // 構造体による「物語」の開始
    let verifier = ConvergenceVerifier::new(eps);
    let report = verifier.find_minimum_n();

    // 結果の出力（ここも本来は別名詞にしたいが、まずはシンプルに）
    println!("\n--- 収束検証報告書 ---");
    println!("目標極限値 L  : 0.0");
    println!("許容誤差 ε    : {}", eps);
    println!("発見された N  : {}", report.threshold_n);
    println!("a_N の値      : {}", report.final_value);
    println!("実際の誤差    : {}", report.difference);
    println!("----------------------");
    println!("∀n > N において、|a_n - 0| < {} が成立することを証明完了。", eps);
}
//0007_eN論法