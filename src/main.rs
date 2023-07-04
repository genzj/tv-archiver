extern crate rust_fuzzy_search;

use std::fs::File;
use std::println;

use rust_fuzzy_search::fuzzy_search_best_n;
use serde_yaml::Mapping;

fn test() {
    let s = "[Lilith-Raws] Yuusha ga Shinda! - 11 [Baha][WEB-DL][1080p][AVC AAC][CHT][MP4]";
    let list: Vec<&str> = vec![
        "Questo mondo non mi renderà cattivo",
        "Jigokuraku",
        "Futoku no Guild",
        // "Yuusha ga Shinda!",
        "Yūsha ga Shinda!",
        "Kyokou Suiri",
    ];
    let n: usize = 2;
    let res: Vec<(&str, f32)> = fuzzy_search_best_n(s, &list, n);
    for (word, score) in res {
        if score <= 0f32 {
            continue;
        }
        println!("{} {:?}", word, score)
    }
}

fn test_conf() {
    let file = File::open("./config.example.yml").unwrap();
    let values:Mapping = serde_yaml::from_reader(file).unwrap();
    println!("{:?}", values);

    let dump = serde_yaml::to_string(&values).unwrap();
    println!("{dump}");
}

fn main() {
    test();
    test_conf();
}
