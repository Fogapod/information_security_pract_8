import sys
import math
import string

from PIL import Image


SUPPORTED_CHARS = string.printable
BITS_PER_LETTER = 8


def main():
    while True:
        text = input("Enter text to encode: ")
        if not text:
            print("No text")

            continue

        if (unsupported := [c for c in text if c not in SUPPORTED_CHARS]) :
            print(f"Unsupported letters: {''.join(unsupported)}")
        else:
            break

    while True:
        src_image_path = input("Enter image path: ")
        if not src_image_path.lower().endswith(".bmp"):
            print("Please, provide a BMP image")

            continue

        try:
            src_image = Image.open(src_image_path).convert("RGBA")
        except Exception as e:
            print(f"Error openning image: {e}")
        else:
            break

    src_width, src_height = src_image.size

    if src_width * src_height < len(text) * BITS_PER_LETTER:
        print("Not enough pixels to encode data")

        sys.exit(1)

    current_pixel = 0
    current_color = 0

    for char in text:
        char_value = ord(char)

        for bit_pos in range(BITS_PER_LETTER):
            bit = char_value & (0b1 << bit_pos)

            x = current_pixel % src_width
            y = current_pixel // src_height

            pixel = list(src_image.getpixel((x, y)))

            def encode_bit(color):
                is_even = color % 2 == 0

                if bit:
                    if is_even:
                        if color == 255:
                            color -= 1
                        else:
                            color += 1
                else:
                    if not is_even:
                        # cannot overflow
                        color += 1

                return color

            pixel[current_color] = encode_bit(pixel[current_color])
            src_image.putpixel((x, y), tuple(pixel))

            if current_color == 2:
                # advance pixel every 3 iterations
                current_pixel += 1

                current_color = 0
            else:
                current_color += 1

    new_image_path = f"{src_image_path.partition('.')[0]}_enc.bmp"
    src_image.save(new_image_path)

    print(f"Wrote {len(text)} characters to {new_image_path}")


if __name__ == "__main__":
    main()
