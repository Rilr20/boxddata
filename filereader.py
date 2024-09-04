"""
Reads the file file
"""

def readfile(filepath):
    """
    readfile function
    """
    f = open(filepath, "r")
    return f.read()