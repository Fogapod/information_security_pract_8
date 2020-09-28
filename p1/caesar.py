import string
import random
import collections


shift = int(input("Enter shift: "))
shift = shift % len(string.ascii_lowercase)

deque = collections.deque(string.ascii_lowercase)

for i in range(shift):
    deque.rotate()

key = "".join(list(deque))

nl = "\n"
print(f"Encrypting with key:\n{nl.join(f'{a} -> {b}' for a, b in zip(string.ascii_lowercase, key))}")

text = input("Enter text to encrypt: ").lower()

encoded = ""
for c in text:
    index = string.ascii_lowercase.find(c)
    if index == -1:
        encoded += c
    else:
        encoded += key[index]

print(f"Encrypted text: {encoded}")

decoded = ""
for c in encoded:
    index = key.find(c)
    if index == -1:
        decoded += c
    else:
        decoded += string.ascii_lowercase[index]

print(f"Decoded text: {decoded}")
