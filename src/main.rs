fn main() {
    use std::io;

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("入力に失敗しました");
    let input = input.trim();
    let mut found = None;
    
    for op in ['+', '-', '*', '/'] {
        let parts: Vec<&str> = input.split(op).collect();
        if parts.len() == 2 {
            found = Some((op, parts));
            break;
        }
    }
    
    let (operator, parts) = match found {
        Some(v) => v,
        None => {
            println!("2項演算のみ対応しています。");
            return;
        }
    };

    let numbers: Result<Vec<f64>, _> =
        parts.iter().map(|s| s.trim().parse::<f64>()).collect();

    let numbers = match numbers {
        Ok(nums) => nums,
        Err(e) => {
            println!("数に変換できません:{}",e);
            return;
        }
    };
    let n1 = numbers[0];
    let n2 = numbers[1];
    
    let result = match operator {
        '+' => n1+n2,
        '-' => n1-n2,
        '*' => n1*n2,
        '/' => {
            if n2 == 0.0 {
                println!("0で割ることはできません。");
                return;
            }
            n1/n2
        }
        _ => unreachable!(),
    };

    println!("計算結果：{}", result);
}
