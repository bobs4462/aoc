from pathlib import Path
filepath = Path("/Users/dinislam/PycharmProjects/aoc/inputs/y2015d1.txt")

print(filepath.name)

with open(filepath, 'r') as input_file:
    for line in input_file:
        print("Floor #: " + str(len(line.split('('))-len((line.split(')')))))