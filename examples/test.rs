#[macro_use]
extern crate tini;
use tini::Ini;

static INPUT: &'static str = "./example.ini";
static OUTPUT: &'static str = "./test.ini";
static SPLIT: &'static str = "=----------------------------------------------------------=";

fn main() {
    let config = Ini::from_file(INPUT).unwrap();
    println!(">> readed `{}` config file\n{}\n{}\n{}",
             INPUT,
             SPLIT,
             config,
             SPLIT);
    let n1: u32 = get_or!(config, "section_one", "name1", 0);
    println!(">> entry `name1` from `section_one` = {}", n1);
    let n2: Vec<bool> = get_vec_or!(config, "section_three", "frst4", vec![false]);
    println!(">> entry `frst4` from `section_three` = {:?}", n2);
    let test = Ini::new()
                   .section("section_one")
                   .item("a", "1")
                   .item("b", "2")
                   .section("section_two")
                   .item("c", "3")
                   .item("d", "4")
                   .build();
    println!(">> builded `{}` config\n{}\n{}\n{}",
             OUTPUT,
             SPLIT,
             test,
             SPLIT);
    test.to_file(OUTPUT).unwrap();
}
