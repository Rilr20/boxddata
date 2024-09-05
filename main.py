"""
Main file
"""
from filereader import readfile
import json
from bs4 import BeautifulSoup
import requests
import re
import lxml
import cchardet

def main():
    """
        Main function
    """
    # Read File
    content = readfile("./data/ratings.csv")
    data = csv_to_json(content)

    f = open("data/ratings.json", "w")
    f.write(data)
    data = json.loads(data)
    # print(data[0]["LetterboxdUri"])
    actors = {}
    language = {}
    directors = {}
    idx = 1
    requests_session = requests.Session()
    for item in data:
        scrape_additional_data(requests_session, item["LetterboxdUri"], item["Rating"], actors, language, directors)
        print(idx)
        idx+=1
        if idx == 5:
            break
    language = dict(sorted(language.items()))
    # scrape_additional_data(data[0]["LetterboxdUri"], data[0]["Rating"], actors, language, directors)
    average_year, films_per_year = get_year_data(data)
    
    average_year = dict(sorted(average_year.items()))
    films_per_year = dict(sorted(films_per_year.items())) 
    # print(average_year)
    # print(films_per_year)
    all_data = {}
    all_data["actors"] = actors
    all_data["language"] = language # sort by value 
    all_data["directors"] = directors
    all_data["average_year"] =  average_year# sort by key
    all_data["film_year"] = films_per_year# sort by key
    all_data = json.dumps(all_data, indent=4, ensure_ascii=False)
    f = open("data/data.json", "w", encoding='utf-8')
    f.write(all_data)
    # file = codecs.open("data/data.json", "w", "utf-8")
    # file.write(all_data)

    
def csv_to_json(data):
    """
        Converts csv data to json
    """
    item_list = []
    data = data.split("\n")
    # columns= data[0].split(",")
    for item in data[1:]:
        if (item != ""):
            split_item = item.split(",")
            if len(split_item) == 5:
                new_item = {}
                new_item["Name"] =split_item[1]
                new_item["Year"] =split_item[2]
                new_item["LetterboxdUri"] =split_item[3]
                new_item["Rating"] =split_item[4]
                item_list.append(new_item)
            else:
                split_item_again = item.split("\"")
                new_item = {}
                new_item["Name"] = split_item_again[1]
                second_half = split_item_again[2].split(",")
                new_item["Year"] = second_half[1]
                new_item["LetterboxdUri"] = second_half[2]
                new_item["Rating"] = second_half[3]
                item_list.append(new_item)

    json_object = json.dumps(item_list, indent=4)
    return json_object

def scrape_additional_data(request_session, url, score, actors, language, directors):
    """
    #TODO: Average Actor/Actress Score
    #TODO: Average Director Score
    #TODO: Movies per language
    """
    print(url)
    r =  request_session.get(url)
    html_content = r.text
    soup = BeautifulSoup(html_content, 'lxml')
    
    # Actors
    if (soup.find('div', attrs={'id': 'tab-cast'}) != None):
        section = soup.find('div', attrs={'id': 'tab-cast'}).find_all('a', attrs={"href": re.compile(r'/actor/.*')})
        score_per_actor(section, score, actors)
    # print(section)

    # Language
    section = soup.find('div', attrs={'id': 'tab-details'}).find_all('a', attrs={"href": re.compile(r'/films/language/.*')})
    movie_language(section, language)
    # print(section)

    # Director
    section = soup.find('div', attrs={'id': 'tab-crew'}).find_all('div')[0].find_all('p')
    score_per_director(section, score, directors)

def score_per_director(content, score, director):
    for item in content:
        # print(item.get_text())
        directorname = item.get_text().strip()
        if director.get(directorname) == None:
            director[directorname] = [float(score)]
        else:
            director[directorname].append(float(score))

def score_per_actor(content, score, actors):
    for item in content:
        actorname = item.get_text().strip()
        if actors.get(actorname) == None:
            actors[actorname] = [float(score)]
        else:
            actors[actorname].append(float(score))

def movie_language(content, language):
    """
    """
    content = convert_html_to_text(content)
    for item in content:
        if language.get(item.strip()) == None:
            language[item.strip()] = 1
        else:
            language[item.strip()] += 1

def get_year_data(data):
    """
    """
    avg_score = {}
    films = {}
    # score = get_average_score_per_year(data)
    # films = get_films_watched_per_year(data)
    for item in data:
        if avg_score.get(item["Year"]) == None:
            avg_score[item["Year"]] = [float(item["Rating"])]
        else:
            avg_score[item["Year"]].append(float(item["Rating"]))

        if films.get(item["Year"]) == None:
            films[item["Year"]] = 1
        else:
            films[item["Year"]] += 1

    return (avg_score, films)

def convert_html_to_text(html):
    text = []
    for item in html:
        text.append(item.get_text())
    res = []
    [res.append(x) for x in text if x not in res]
    return res
#TODO: Movie genres; pie chart?

if __name__ == "__main__":
    main()
