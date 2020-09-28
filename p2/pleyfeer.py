import pprint
import collections


NUM_ROWS = 5

ru_letters = "абвгдежзиклмнопрстуфхцчшщъыэюя"

assert len(ru_letters) % NUM_ROWS == 0, "Invalid alphabet length"

NUM_COLS = len(ru_letters) // NUM_ROWS

word = input("Введите секретное слово: ").lower()
word = word.replace("й", "и")
word = word.replace("ь", "ъ")

for c in word:
    if word.count(c) > 1:
        raise RuntimeError("Повторяющиеся символы недопустимы")

key_s = word
for c in ru_letters:
    if c in word:
        word = word.replace(c, "")

        continue

    key_s += c

if word:
    raise RuntimeError(f"Символ вне алфавита: {word[0]}")

key = []

for i, c in enumerate(key_s):
    if i % NUM_COLS == 0:
        key.append([c])
    else:
        key[i // 6].append(c)

pprint.pp(key)

encoded = input("Enter encoded text: ").lower().replace(" ", "")

def find_pos(c):
    for y, row in enumerate(key):
        try:
            x = row.index(c)
        except ValueError:
            continue

        return x, y

decoded = ""

for a, b in list(zip(encoded[::2], encoded[1::2])):
    ax, ay = find_pos(a)
    bx, by = find_pos(b)

    if ay == by:
        new_a, new_b = key[ay][ax - 1], key[ay][bx - 1]
    elif ax == bx:
        new_a, new_b = key[ay - 1][ax], key[by - 1][ax]
    else:
        new_a, new_b = key[ay][bx], key[by][ax]

    decoded += f"{new_a}{new_b} "

print(decoded)
