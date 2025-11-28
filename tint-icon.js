const fs = require('fs');
const { createCanvas, loadImage } = require('canvas');

async function tintIcon() {
  const img = await loadImage('src-tauri/icons/icon.png');
  const canvas = createCanvas(img.width, img.height);
  const ctx = canvas.getContext('2d');
  
  // Draw original image
  ctx.drawImage(img, 0, 0);
  
  // Apply green tint overlay
  ctx.globalCompositeOperation = 'source-atop';
  ctx.fillStyle = 'rgba(76, 175, 80, 0.6)'; // Green with 60% opacity
  ctx.fillRect(0, 0, canvas.width, canvas.height);
  
  // Save the tinted image
  const buffer = canvas.toBuffer('image/png');
  fs.writeFileSync('src-tauri/icons/icon-active.png', buffer);
  console.log('✓ Created green-tinted active icon');
}

tintIcon().catch(err => {
  console.error('Error:', err.message);
  console.log('Install canvas with: npm install canvas');
});
