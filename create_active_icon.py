#!/usr/bin/env python3
"""
Create an active (green-tinted) version of the icon for system tray
"""
from PIL import Image, ImageEnhance, ImageFilter
import sys


def create_active_icon(input_path, output_path):
    # Open the original icon
    img = Image.open(input_path).convert('RGBA')

    # Get the image data
    pixels = img.load()
    width, height = img.size

    # Create a new image for the active state
    active_img = Image.new('RGBA', (width, height))
    active_pixels = active_img.load()

    # Apply green tint to non-transparent pixels
    for y in range(height):
        for x in range(width):
            r, g, b, a = pixels[x, y]
            if a > 0:  # If not fully transparent
                # Add green tint while maintaining some of the original color
                new_r = int(r * 0.3 + 76 * 0.7)   # Mix with green (#4CAF50)
                new_g = int(g * 0.3 + 175 * 0.7)
                new_b = int(b * 0.3 + 80 * 0.7)
                active_pixels[x, y] = (new_r, new_g, new_b, a)
            else:
                active_pixels[x, y] = (r, g, b, a)

    # Save the active icon
    active_img.save(output_path)
    print(f"✓ Created active icon: {output_path}")


if __name__ == "__main__":
    input_icon = "src-tauri/icons/icon.png"
    output_icon = "src-tauri/icons/icon-active.png"

    try:
        create_active_icon(input_icon, output_icon)
    except Exception as e:
        print(f"Error: {e}")
        print("Make sure Pillow is installed: pip install Pillow")
        sys.exit(1)
