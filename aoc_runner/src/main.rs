fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: aoc_runner <year> <day>");
        std::process::exit(1);
    }

    let year = &args[1];
    let day = &args[2];

    match (year.as_str(), day.as_str()) {
        ("2025", "01") => y2025::day01::run(),
        _ => panic!("Day not implemented yet"),
    }
}

