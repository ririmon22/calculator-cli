use std::io::stdin;
fn main() {
    //TODO: 入力を受け付ける
    use std::io;

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("入力に失敗しました");
    let input = input.trim();
    let parts: Vec<&str> = input.split('+').collect();
    let n1 = parts[0].parse::<i32>().unwrap();
    let n2 = parts[1].parse::<i32>().unwrap();
    println!("{}",n1+n2);
    

    //TODO: 式を振り分ける
    //TODO: 計算する
    //TODO: 結果を表示する
}
