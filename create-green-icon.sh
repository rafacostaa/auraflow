#!/bin/bash

# Create a simple green-tinted version using Core Image filters
cd "$(dirname "$0")/src-tauri/icons"

# Use Python with PIL which might be available
python3 << 'EOF'
try:
    from PIL import Image, ImageEnhance
    
    # Open the icon
    img = Image.open('icon.png').convert('RGBA')
    
    # Create a green overlay
    green_overlay = Image.new('RGBA', img.size, (76, 175, 80, 100))
    
    # Composite the images
    result = Image.alpha_composite(img, green_overlay)
    
    # Save
    result.save('icon-active.png')
    print('✓ Created green-tinted icon successfully!')
    
except ImportError:
    print('PIL not available. Please manually color icon-active.png green.')
    print('You can use any image editor to tint it green (#4CAF50)')
except Exception as e:
    print(f'Error: {e}')
EOF
