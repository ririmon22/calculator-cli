use std::io::stdin;
fn main() {
    //TODO: 入力を受け付ける
    use std::io;

    let mut input = String::new();
    let operator;
    let parts: Vec<&str>;
    io::stdin().read_line(&mut input).expect("入力に失敗しました");
    let input = input.trim();
    if input.contains('+') {
        operator='+';
        parts = input.split('+').collect();
    }else if input.contains('-') {
        operator='-';
        parts = input.split('-').collect();
    }else if input.contains('*') {
        operator='*';
        parts = input.split('*').collect();
    }else if input.contains('/') {
        operator='/';
        parts = input.split('/').collect();
    }else {
        println!("対応していない演算子です。");
        return;
    }
    
    if parts.len() != 2 {
        println!("2項演算のみに対応しています。");
        return;
    }
    let n1 = parts[0].parse::<i32>().unwrap();
    let n2 = parts[1].parse::<i32>().unwrap();
    
    let result = match operator {
        '+' => n1+n2,
        '-' => n1-n2,
        '*' => n1*n2,
        '/' => n1/n2,
        _ => unreachable!(),
    };

    println!("計算結果：{}", result);
}
