"""
Main file
"""
from filereader import readfile
import json
from bs4 import BeautifulSoup
import requests
import re

def main():
    """
        Main function
    """
    # Read File
    content = readfile("./data/ratings.csv")
    print("Hello World!")
    # print(content)
    data = csv_to_json(content)
    
    f = open("data/ratings.json", "w")
    f.write(data)
    data = json.loads(data)
    # print(data[0]["LetterboxdUri"])
    actors = {}
    language = {}
    scrape_additional_data(data[0]["LetterboxdUri"], data[0]["Rating"], actors, language)
        

def csv_to_json(data):
    """
        Converts csv data to json
    """
    item_list = []
    data = data.split("\n")
    # columns= data[0].split(",")
    for item in data[1:]:
        if (item != ""):
            item = item.split(",")
            new_item = {}
            new_item["Name"] =item[1]
            new_item["Year"] =item[2]
            new_item["LetterboxdUri"] =item[3]
            new_item["Rating"] =item[4]
            item_list.append(new_item)
    json_object = json.dumps(item_list, indent=4)
    return json_object

def scrape_additional_data(url, score, actors, language):
    """
    #TODO: Average Actor/Actress Score
    #TODO: Average Director Score
    #TODO: Movies per language
    """
    response =  requests.get(url)
    html_content = response.text
    soup = BeautifulSoup(html_content, 'html.parser')
    section = soup.find_all('div', attrs={'class': 'cast-list'})[0].find_all("p")[0].find_all("a")
    score_per_actor(section, score, actors)
    # print(section)
    section = soup.find('div', attrs={'id': 'tab-details'}).find_all('div')[2].find_all('p')
    # print(section)
    movie_language(section, language)

def score_per_actor(content, score, actors):
    for item in content:
        if actors.get(item.get_text()) == None:
            actors[item.get_text()] = [int(score)]
        else:
            actors[item.get_text()].append(int(score))
        

def movie_language(content, language):
    """
    """
    for item in content:
        if language.get(item.get_text()) == None:
            language[item.get_text().strip()] = 1
        else:
            language[item.get_text().strip()] += 1

def get_average_score_per_year(data):
    """
    #TODO: Average score per release yeaar
    """
def get_films_watched_per_year(data):
    """
    #TODO: Movies watched per release yeaar
    """
#TODO: Movie genres; pie chart?

if __name__ == "__main__":
    main()
