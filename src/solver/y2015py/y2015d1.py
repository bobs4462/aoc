from pathlib import Path
filepath = Path("y2015d1_input")

print(filepath.name)

with open(filepath, 'r') as input_file:
    for line in input_file:
        print("Floor #: " + str(len(line.split('('))-len((line.split(')')))))