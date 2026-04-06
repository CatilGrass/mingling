fn main() {
    println!("{}", include_str!("./guide.txt").trim_end_matches("\n"));
}
