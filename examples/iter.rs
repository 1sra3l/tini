extern crate tini;
use tini::Ini;

fn main() {
    let config = Ini::new()
        .section("names")
        .item("first", "John")
        .item("second", "Peter")
        .item("third", "Emily")
        .section("languages")
        .item("list", "c, c++, rust");

    // iterate over config
    for (name, section_iter) in config.iter() {
        println!("section {} with items:", name);
        for (_, value) in section_iter {
            println!("  - {}", value);
        }
    }

    println!("\n--- serialize to ini ---\n{}---", config);
}
