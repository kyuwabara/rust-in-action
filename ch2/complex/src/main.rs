use num::complex::Complex;

fn main() {
    let a = Complex{re: 2.1, im: -1.2}; // 全ての Rust 型にはリテラル構文がある
    let b = Complex::new(11.1, 22.2); // だいたいの型で new が実装されている
    let result = a + b;

    println!("{} + {}i", result.re, result.im); // フィールドにドットアクセス
}
