// 必要なクレートとモジュールをインポート
use std::env;
use std::fs::File;
use std::io::{ BufRead, BufReader };
use walkdir::WalkDir;
use regex::Regex;

fn main() {
    // コマンドライン引数の取得
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <folder> <function_name>", args[0]);
        std::process::exit(1);
    }
    let folder = &args[1];
    let func_name = &args[2];

    // 関数呼び出しをマッチする正規表現パターン
    let pattern = format!(r"\b{}\s*\(", regex::escape(func_name));
    let re = Regex::new(&pattern).expect("Invalid regex pattern");

    let mut total_count = 0;

    // ディレクトリを再帰的に走査（除外フォルダ: node_modules, .vscode, .angular, .git）
    for entry in WalkDir::new(folder)
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            !(name == "node_modules" || name == ".vscode" || name == ".angular" || name == ".git")
        })
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file()) {
        let path = entry.path();
        // .ts と .html ファイルのみ対象
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            if ext != "ts" && ext != "html" {
                continue;
            }
        } else {
            continue;
        }

        if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);
            for (idx, line) in reader.lines().enumerate() {
                if let Ok(content) = line {
                    if re.is_match(&content) {
                        total_count += 1;
                        // ファイルパスと行番号を出力
                        println!("{}:{}", path.display(), idx + 1);
                    }
                }
            }
        }
    }

    // 合計出現回数を出力
    println!("Total occurrences of '{}': {}", func_name, total_count);
}
