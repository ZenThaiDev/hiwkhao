use parser::Parser;

const SCANNER_DEFAULT_OUTPUT_FILE: &str = "hiwkhao.tok";

fn scanner(input: &String) {
    let result = scanner_lib::run_scanner(&input);
    //println!("{}", result.join("\n"));

    let output_file = std::env::args()
        .nth(2)
        .unwrap_or(SCANNER_DEFAULT_OUTPUT_FILE.to_string());

    std::fs::write(output_file, result.join("\n")).unwrap();
}

fn main() {
    let input = if let Some(file_path) = std::env::args().nth(1) {
        std::fs::read_to_string(file_path).unwrap()
    } else {
        eprintln!("No input file provided.");
        std::process::exit(1);
    };

    scanner(&input);

    let tokens = scanner_lib::tokenize(&input);
    let mut parser = Parser::new(vec![]);
    println!("{:?}", tokens);
    let result = parser.parse_tokens(tokens);

    println!("{:?}", result);

    //parser = Parser::new(vec![]);
    //result = parser.parse_file(input);

    //println!("{:?}", result);
}
