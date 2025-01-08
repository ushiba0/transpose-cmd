use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

fn print_usage_and_exit(args: &[String]) {
    eprintln!("Usage:");
    eprintln!("\t{} [INPUT_FILE] > [OUTPUT_FILE]", args[0]); // 引数 1 つの場合、[INPUT_FILE] は引数で、出力は stdout.
    eprintln!("\t{} [INPUT_FILE] [OUTPUT_FILE]", args[0]); // 引数 2 つの場合、第一引数が [INPUT_FILE], 第二引数が [OUTPUT_FILE].
    eprintln!("\tcat [INPUT_FILE] | {} > [OUTPUT_FILE]", args[0]); // 引数 0 個の場合、stdin が入力、stdout が出力.
    std::process::exit(1);
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut matrix: Vec<Vec<String>> = Vec::new();

    // ヘルプを表示。
    if args.iter().any(|arg| arg == "-h" || arg == "--help") {
        print_usage_and_exit(&args);
    }

    // 引数チェック
    if !matches!(args.len(), 1..=3) {
        print_usage_and_exit(&args);
    }

    // 入力ファイルと出力ファイルを開く。
    let (input_reader, mut output_writer):(Box<dyn BufRead>,Box<BufWriter<dyn Write>>) = match args.len() {
        1 => {
            let reader = BufReader::new(std::io::stdin());
            let writer = BufWriter::new(std::io::stdout());
            (Box::new(reader), Box::new(writer))
        }
        2 => {
            let input_file = File::open(Path::new(&args[1]))?;
            let reader = BufReader::new(input_file);
            let writer = BufWriter::new(std::io::stdout());
            (Box::new(reader), Box::new(writer))
        }
        3 => {
            let input_file = File::open(Path::new(&args[1]))?;
            let reader = BufReader::new(input_file);
            let output_file = File::open(Path::new(&args[2]))?;
            let writer = BufWriter::new(output_file);
            (Box::new(reader), Box::new(writer))
        }
        _=> unreachable!()
    };

    // CSVを2次元配列に読み込む
    for line in input_reader.lines() {
        let line = line?;
        let row: Vec<String> = line.split(',').map(|s| s.trim().to_string()).collect();
        matrix.push(row);
    }

    // 行列のサイズを取得。
    let num_rows = matrix.len();
    let num_cols = matrix[0].len();

    // すべての行の長さが同じかチェック
    if !matrix.iter().all(|row| row.len() == num_cols) {
        eprintln!("Error: All rows must have the same number of columns");
        std::process::exit(1);
    }

    // 結果を出力。
    for i in 0..num_cols {
        let mut new_row: Vec<&str> = Vec::new();
        #[allow(clippy::needless_range_loop)]
        for j in 0..num_rows {
            new_row.push(&matrix[j][i]);
        }
        writeln!(output_writer, "{}", new_row.join(","))?;
    }

    Ok(())
}
