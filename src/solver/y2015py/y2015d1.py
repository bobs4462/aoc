from pathlib import Path
filepath = Path("inputs/y2015d1.txt")

with open(filepath, 'r') as input_file:
    for line in input_file:
        print("Floor #: " + str(len(line.split('('))-len((line.split(')')))))