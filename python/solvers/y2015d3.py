file = open('../inputs/y2015d3.txt')
a = 0
b = 0
homes = set()

#Делай раз!
for char in file.readlines()[0]:
    if (a, b) not in homes:
        homes.add((a, b))
    if char == '<':
        a -= 1
    elif char == '>':
        a += 1
    elif char == '^':
        b += 1
    elif char == 'v':
        b -= 1

print(len(homes))
file.close()
#Делай двас!
file = open('../inputs/y2015d3.txt')
SantaHome = set()
SantaRoboHome = set()
sha = 0
shb = 0
srbha = 0
srbhb = 0

for index, char in enumerate(file.readlines()[0]):
    a1 = 0
    b1 = 0

    if char == '<':
        a1 -= 1
    elif char == '>':
        a1 += 1
    elif char == '^':
        b1 += 1
    elif char == 'v':
        b1 -= 1

    if index % 2 == 0:
        srbha += a1
        srbhb += b1
        SantaRoboHome.add((srbha, srbhb))
    else:
        sha += a1
        shb += b1
        SantaHome.add((sha, shb))

print(len(SantaRoboHome | SantaHome))
file.close()