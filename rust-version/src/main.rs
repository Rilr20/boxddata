use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use serde_json::{Value, Map};
use serde::{Serialize, Deserialize};
use reqwest::{self, Url};
use std::error::Error;
use scraper;
use std::cmp::Ordering;

#[derive(Serialize, Deserialize)]
struct Actors {
    actors: HashMap<String, Vec<f32>>,
}

#[derive(Serialize, Deserialize)]
struct Language {
    language: HashMap<String, i32>,
}

#[derive(Serialize, Deserialize)]
struct Directors {
    directors: HashMap<String, Vec<f32>>,
}

#[derive(Serialize, Deserialize)]
struct AverageYear {
    average_year: HashMap<String, Vec<f32>>,
}

#[derive(Serialize, Deserialize)]
struct FilmsPerYear {
    film_year: HashMap<String, i32>,
}

#[derive(Serialize, Deserialize)]
struct AllData {
    actors: HashMap<String, Vec<f32>>,
    directors: HashMap<String, Vec<f32>>,
    average_year: HashMap<String, Vec<f32>>,
    film_year: HashMap<String, i32>,
    language: HashMap<String, i32>,
}

fn write_to_json(all_data: &AllData, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(all_data)?;
    fs::write(filename, json)?;
    Ok(())
}

fn write_to_file(data:&Vec<Map<String,Value>>) -> std::io::Result<()> {
    let file = File::create("../data/rating.json")?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, &data)?;
    writer.flush()?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let path: &str = "../data/ratings.csv";
    let content: String = read_file(path);
    let data = csv_to_json(content);

    let _ = write_to_file(&data);
    
    let mut actors: HashMap<String, Vec<f32>> = HashMap::new();
    let mut language: HashMap<String, i32> = HashMap::new();
    let mut directors: HashMap<String, Vec<f32>> = HashMap::new();
    let mut average_year: HashMap<String, Vec<f32>> = HashMap::new();
    let mut film_year: HashMap<String, i32> = HashMap::new();

    let mut idx= 1;
    for item in data {
        println!("{}", idx);
        let _ = scrape_additional_data(item["LetterboxdUri"].to_string(), item["Rating"].to_string(), &mut actors, &mut language, &mut directors).await;
        get_year_data(&mut average_year, &mut film_year, item["Rating"].to_string(), item["Year"].to_string(),);
        idx+=1;
    }

    let all_data = AllData {
        actors: actors,
        directors: directors,
        average_year: average_year,
        film_year: film_year,
        language: language,
    };

    let _ = write_to_json(&all_data, "../data/rust-data.json");


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

async fn scrape_additional_data(url: String, score: String, actors:&mut HashMap<String, Vec<f32>>, language:&mut HashMap<String, i32>, directors:&mut HashMap<String, Vec<f32>>) -> Result<(), Box<dyn Error>> {
    let url_ending:Vec<&str> = url.split("/").collect();
    let base = Url::parse("https://boxd.it/")?;
    let full_url = format!("{}{}", "https://boxd.it/", url_ending[url_ending.len()-1]);
    let url = base.join(full_url.split("\"").collect::<Vec<&str>>()[0])?;

    
    let body = reqwest::get(url)
        .await?
        .text()
        .await?;
    let document = scraper::Html::parse_document(&body);

    let html_selector = scraper::Selector::parse("a.text-slug").unwrap();
    let html_casts = document.select(&html_selector);

    let mut no_duplicates: Vec<String> = Vec::new();

    for item in html_casts {

        let mut attr = "";
        if item.value().attr("href") != None {
            attr = item.value().attr("href").unwrap();
            let inner_html = item.text().collect::<String>();
            let html_text = &inner_html;
            let split_string: Vec<&str> = attr.split("/").collect();
            let split_score: Vec<&str> = score.split("\"").collect();
            let new_score: &str = split_score[1];
            let score_parsed: f32 = new_score.parse().expect("Failed to parse score");


            match split_string[1] {
                "actor"=> {
                    // println!("{}", inner_html);
                    actors.entry(inner_html.to_string()).or_insert(Vec::new()).push(score_parsed);
                },
                "director"=> {
                    // println!("{}", inner_html);
                    directors.entry(inner_html.to_string()).or_insert(Vec::new()).push(score_parsed);
                },
                "films"=> match split_string[2] {
                    "language" => {
                        // println!("{}", inner_html);
                        if !no_duplicates.contains(html_text) {
                            no_duplicates.push(inner_html.clone());
                            language.entry(inner_html.to_string()).and_modify(|count| *count += 1).or_insert(1);
                        }
                    },
                    _=> (),
                    },
                _ => (),
            };
        }
    }

    Ok(())
}


fn get_year_data(average_year: &mut HashMap<String, Vec<f32>>, films_per_year:&mut HashMap<String, i32>, score: String, year:String) {
    let true_year:String = serde_json::from_str(year.as_str()).unwrap();
    if true_year != "" {
        let split_score: Vec<&str> = score.split("\"").collect();
        let new_score: &str = split_score[1];
        let score_parsed: f32 = new_score.parse().expect("Failed to parse score");
            average_year.entry(true_year.clone()).or_insert(Vec::new()).push(score_parsed);
            films_per_year.entry(true_year.clone()).and_modify(|count| *count += 1).or_insert(1);
    }
}
