import sys
import math
import string

from PIL import Image


SUPPORTED_CHARS = string.printable
BITS_PER_LETTER = 8


def main():
    if len(sys.argv) < 3:
        print("Not enough arguments provided")
        sys.exit(1)

    src_image_path = sys.argv[1]
    text = " ".join(sys.argv[2:])

    if (unsupported := [c for c in text if c not in SUPPORTED_CHARS]) :
        print(f"Unsupported letters: {''.join(unsupported)}")
        sys.exit(1)

    if not src_image_path.lower().endswith(".bmp"):
        print("Please, provide a BMP image")
        sys.exit(1)

    try:
        src_image = Image.open(src_image_path).convert("RGBA")
    except Exception as e:
        print(f"Error opening image: {e}")
        sys.exit(1)

    src_width, src_height = src_image.size

    if src_width * src_height < len(text) * BITS_PER_LETTER:
        print("Not enough pixels to encode data")

        sys.exit(1)

    x = 0
    y = 0

    # R - 0
    # G - 1
    # B - 2
    current_color = 0

    for char in text:
        char_value = ord(char)

        for bit_pos in range(BITS_PER_LETTER):
            bit = bool(char_value & (0b1 << bit_pos))

            pixel = list(src_image.getpixel((x, y)))

            def hide_bit(color):
                # if bit is set, color value should not be even

                is_even = color % 2 == 0

                if bit:
                    if is_even:
                        # cannot overflow
                        color += 1
                else:
                    if not is_even:
                        if color == 255:
                            color -= 1
                        else:
                            color += 1

                return color

            pixel[current_color] = hide_bit(pixel[current_color])
            src_image.putpixel((x, y), tuple(pixel))

            if current_color == 2:
                # advance coords when we are at BLUE value (3rd)
                x += 1
                if x == src_width:
                    x = 0
                    y += 1

                current_color = 0
            else:
                current_color += 1

    new_image_path = f"{src_image_path.partition('.')[0]}_enc.bmp"
    src_image.save(new_image_path)

    print(f"Wrote {len(text)} characters to {new_image_path}")


if __name__ == "__main__":
    main()
