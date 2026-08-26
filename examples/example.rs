fn main() {
    let name = String::from("examples/example.sex");
    let file = std::fs::File::open(&name).unwrap();
    match sex::parse_exprlist_reader(file, Some(name)) {
        Ok(atom) => {
            println!("{atom:?}");
        }
        Err(err) => {
            eprintln!("{err}");
        }
    }
}
