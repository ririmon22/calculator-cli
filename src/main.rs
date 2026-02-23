fn main(){
    if let Err(e) = run() {
        eprintln!("{}", e);
    }
}

/// 電卓のメインロジックを実行する。
///
/// 入力の取得から計算結果の出力までを担当する。
/// 各処理で発生したエラーは `Result` として呼び出し元へ返す。
///
/// # Returns
/// Ok(())        - 正常終了
/// Err(String)   - 入力・解析・計算時のエラー
fn run() -> Result<(), String> {
    let input = read_input()?;
    let (op, parts) = find_operator(&input)?;
    let (n1, n2) = parse_numbers(parts)?;
    let result = calculate(n1, op, n2)?;

    println!("計算結果：{}", result);
    Ok(())
}

/// ユーザーからの入力を取得する。
///
/// 標準入力から1行読み込み、trim を行った文字列を返す。
/// I/O に失敗した場合は Err を返す。
///
/// # Returns
/// Ok(String)  - 入力文字列（前後の空白除去済み）
/// Err(String) - 入力エラー時のメッセージ
fn read_input() -> Result<String, String> {
    use std::io;

    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|e| format!("入力に失敗しました:{}", e))?;
    
    Ok(input.trim().to_string())
}

/// 入力文字列から演算子と左右の式を抽出する。
///
/// '+', '-', '*', '/' のいずれかを検出し、
/// 2項演算であることを確認する。
///
/// # Arguments
/// * `input` - ユーザーから入力された式文字列
///
/// # Returns
/// Ok((operator, parts)) - 演算子と分割された2つの文字列
/// Err(String)           - 演算子が見つからない、または2項演算でない場合
fn find_operator(input: &str) -> Result<(char, Vec<&str>), String> {
    for op in ['+', '-', '*', '/'] {
        let parts: Vec<&str> = input.split(op).collect();
        if parts.len() >= 2 {
            return Ok((op, parts));
        }
    }

    Err("演算子が見つかりません。".to_string())
}

/// 文字列で与えられた2つの値を浮動小数に変換する。
///
/// 各要素に対して trim を行い、f64 へ変換する。
/// 変換に失敗した場合はエラーメッセージを返す。
///
/// # Arguments
/// * `parts` - 演算子で分割された2つの文字列
///
/// # Returns
/// Ok((n1, n2)) - 正常に変換できた場合
/// Err(String)  - 数値変換に失敗した場合
/// 現在は2項演算のみを前提としている。
/// parts の長さが2であることを前提とする。
fn parse_numbers(parts: Vec<&str>) -> Result<(f64, f64), String> {
    if parts.len() != 2 {
       return Err("2項演算のみに対応しています。".to_string())
    }
    let numbers: Result<Vec<f64>, _> =
        parts.iter().map(|s| s.trim().parse::<f64>()).collect();

    let numbers = numbers.map_err(|e| format!("数に変換できません: {}", e))?;
    
    Ok((numbers[0], numbers[1]))
}

/// 二項演算を実行する。
/// 
/// 与えられた2つの数値と演算子に基づいて計算を行う。
/// 0除算など数学的に不正な場合は Err を返す。
///
/// # Arguments
/// * `n1` - 左辺の数値
/// * `op` - 演算子（'+', '-', '*', '/'）
/// * `n2` - 右辺の数値
///
/// # Returns
/// Result<f64, String>
/// 現在は2項演算のみ対応している。
/// 複数演算子や優先順位は未対応。
fn calculate(n1: f64, op: char, n2: f64) -> Result<f64, String> {
    match op {
        '+' => Ok(n1 + n2),
        '-' => Ok(n1 - n2),
        '*' => Ok(n1 * n2),
        '/' => {
            if n2 == 0.0 {
                Err("0で割ることはできません".to_string())
            } else {
                Ok(n1 / n2)
            }
        }
        _ => Err("未対応の演算子です".to_string()),
    }
}