#  from pathlib import Path
#  filepath = Path("y2015d1_input")


#  with open(filepath, 'r') as input_file:
#      for line in input_file:
#          print("Floor #: " + str(len(line.split('('))-len((line.split(')')))))


# v1 Meh, somewhat ugly, and not efficient at all
with open('../inputs/y2015d1.txt') as f:
    line = f.read()
    print("Floor #: " + str(len(line.split('('))-len((line.split(')')))))

# v2 Most Efficient code
counter = 0
for c in open('../inputs/y2015d1.txt').read():
    if c == '(':
        counter += 1
    elif c == ')':
        counter -= 1
print(f"Floor #: {counter}")

# v3 If you want a oneliner
print("Floor #: {}".format(sum([1 if c == '(' else -1 for c in open('../inputs/y2015d1.txt').read()])))
