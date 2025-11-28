import pyautogui
import time
import random
from datetime import datetime

# Configuration
IDLE_THRESHOLD_SECONDS = 120  # Start jiggling after this many seconds of inactivity
JIGGLE_INTERVAL = 60          # How often to jiggle when idle (in seconds)
MOVEMENT_PIXELS = 5           # Maximum pixels to move
CHECK_INTERVAL = 5            # How often to check for activity (in seconds)

# Enable failsafe - move mouse to top-left corner to abort
pyautogui.FAILSAFE = True

print("🖱️  Smart Mouse Jiggler Started")
print(f"   Idle threshold: {IDLE_THRESHOLD_SECONDS} seconds")
print(f"   Jiggle interval: {JIGGLE_INTERVAL} seconds")
print("   Press Ctrl+C to stop, or move mouse to top-left corner")
print("-" * 50)

last_position = pyautogui.position()
last_activity_time = time.time()
is_idle = False

try:
    while True:
        current_position = pyautogui.position()
        current_time = time.time()

        # Check if mouse has moved (user activity detected)
        if current_position != last_position:
            last_activity_time = current_time
            last_position = current_position

            if is_idle:
                timestamp = datetime.now().strftime("%H:%M:%S")
                print(f"[{timestamp}] 👤 Activity detected - pausing jiggler")
                is_idle = False

        # Calculate idle time
        idle_time = current_time - last_activity_time

        # Check if we should start jiggling
        if idle_time >= IDLE_THRESHOLD_SECONDS:
            if not is_idle:
                timestamp = datetime.now().strftime("%H:%M:%S")
                print(f"[{timestamp}] 😴 Idle detected - starting auto-jiggle")
                is_idle = True

            # Random small movement to avoid detection
            x_offset = random.randint(-MOVEMENT_PIXELS, MOVEMENT_PIXELS)
            y_offset = random.randint(-MOVEMENT_PIXELS, MOVEMENT_PIXELS)

            # Move mouse
            pyautogui.moveRel(x_offset, y_offset, duration=0.2)

            # Update position (ignore our own movement)
            last_position = pyautogui.position()

            # Log the jiggle
            timestamp = datetime.now().strftime("%H:%M:%S")
            idle_mins = int(idle_time // 60)
            print(
                f"[{timestamp}] 🖱️  Jiggled ({x_offset:+d}, {y_offset:+d}) - idle for {idle_mins}m")

            # Wait before next jiggle
            time.sleep(JIGGLE_INTERVAL)
        else:
            # Check more frequently when not idle
            time.sleep(CHECK_INTERVAL)

except KeyboardInterrupt:
    print("\n✓ Mouse Jiggler stopped by user")
except pyautogui.FailSafeException:
    print("\n⚠️  Failsafe triggered - mouse moved to corner")
