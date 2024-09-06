// use soup;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use serde_json::{Value, Map};


fn write_to_file(data:&Vec<Map<String,Value>>) -> std::io::Result<()> {
    let file = File::create("./rating.json")?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, &data)?;
    writer.flush()?;
    Ok(())
}

fn main() {
    let path: &str = "../data/ratings.csv";
    let content: String = read_file(path);
    let mut data = csv_to_json(content);

    let _ = write_to_file(&data);
}

fn read_file(path: &str) -> String {
    let content = fs::read_to_string(path).expect("read content");
    return content;
}

fn csv_to_json(data: String) -> Vec<Map<String,Value>> {
    let split_lines = data.lines();

    let mut item_list = vec![];

    for item in split_lines.skip(1) {
        if item != "" {
            let split_item: Vec<&str> = item.split(",").collect();
            if split_item.len() == 5 {
                let mut new_item = Map::new();
                
                new_item.insert("Name".to_string(), Value::String(split_item[1].to_string()));
                new_item.insert("Year".to_string(), Value::String(split_item[2].to_string()));
                new_item.insert("LetterboxdUri".to_string(),  Value::String(split_item[3].to_string()));
                new_item.insert("Rating".to_string(), Value::String(split_item[4].to_string()));
 
                item_list.push(new_item);
            } else {
                let split_item_again: Vec<&str> = item.split("\"").collect();
                let second_half: Vec<&str> = split_item_again[2].split(",").collect();

                let mut new_item = Map::new();

                // let new_item =  {
                //     name: split_item_again[1].to_string(),
                //     year: second_half[1].to_string(),
                //     letterboxd_uri: second_half[2].to_string(),
                //     rating: second_half[3].parse::<f32>().unwrap(),
                // };
                new_item.insert("Name".to_string(), Value::String(split_item_again[1].to_string()));
                new_item.insert("Year".to_string(), Value::String(second_half[1].to_string()));
                new_item.insert("LetterboxdUri".to_string(),  Value::String(second_half[2].to_string()));
                new_item.insert("Rating".to_string(), Value::String(second_half[3].to_string()));
                item_list.push(new_item);
            }
        }
    }
    return item_list;
}

fn scrape_additional_data() {}

fn score_per_director() {}

fn score_per_actor() {}

fn movie_language() {}

fn get_year_data() {}

fn convert_html_to_text() {}
