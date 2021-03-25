#сокращаем количество выводимой информации (поставь True чтобы увидеть ВСЁ)
verbose = False

total_paper = 0
total_ribbon = 0

#берем строку из инпута и переводим ее в более лицеприятный вид
for line in open('../inputs/y2015d2.txt'):
    if verbose:
        print("Данные из инпута:", line)
#выводим размер коробки, исходя из инфы инпута
    l, w, d = line.split('x')
    l, w, d = int(l), int(w), int(d)
    if verbose:
        print("Размеры:", l, w, d)
#считаем сколько бумаги нужно для упаковки 1 подарка
    s1 = l * w
    s2 = l * d
    s3 = w * d
    extra = min(s1, s2, s3)
    paper_needed = 2*s1 + 2*s2 + 2*s3 + extra
    if verbose:
        print("Бумаги для упаковки нужно:", paper_needed, '\n')
    total_paper += paper_needed

#считаем сколько ленточек нужно для упакованных подарков
    ribbon = min(l+l+w+w, l+l+d+d, w+w+d+d)
    bow = l * w * d
    total_ribbon += (ribbon + bow)

print("Всего бумаги нужно:", total_paper)
print("Всего ленточек нужно:", total_ribbon)