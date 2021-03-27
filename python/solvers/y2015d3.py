locations, coords = set(), complex(0, 0)
for i in open('../inputs/y2015d3.txt').read():
    if i == '>':
        coords += 1
    elif i == '<':
        coords -= 1
    elif i == '^':
        coords += 1j
    elif i == 'v':
        coords -= 1j
    locations.add(coords)
print(f"Number of houses: {len(locations)}")
