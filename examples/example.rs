fn main() {
    let name = String::from("examples/example.sex");
    let file = std::fs::File::open(&name).unwrap();
    if let Err(err) = sex::parse_exprlist_reader(file) {
        eprintln!("{err}");
    }
}
