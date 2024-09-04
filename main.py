"""
Main file
"""
from filereader import readfile
import json

def main():
    """
        Main function
    """
    # Read File
    content = readfile("./data/ratings.csv")
    print("Hello World!")
    # print(content)
    data = csv_to_json(content)
    print(data)


def csv_to_json(data):
    """
        Converts csv data to json
    """
    item_list = []
    data = data.split("\n")
    # columns= data[0].split(",")
    for item in data:
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

#TODO: Average score per release yeaar
#TODO: Movies watched per release yeaar
#TODO: Average Director Score
#TODO: Average Actor/Actress Score
#TODO: Movie genres; pie chart?
#TODO: Movies per language









if __name__ == "__main__":
    main()
