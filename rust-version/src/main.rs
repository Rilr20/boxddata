use std::fs;
use soup;


fn main() {
    // println!("Hello, world!");
    let path:&str = "../data/ratings.csv";
    let mut file_content: String = read_file(path);
    let mut split_lines = file_content.lines();

    // println!("{:?}", split_lines)
}

fn read_file(path: &str) -> String {
    let content = fs::read_to_string(path)
        .expect("read content");
    return content;
}

fn csv_to_json() {}

fn scrape_additional_data() {}

fn score_per_director() {}

fn score_per_actor() {}

fn movie_language() {}

fn get_year_data() {}

fn convert_html_to_text() {}
