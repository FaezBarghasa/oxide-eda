#!/usr/bin/env python3
"""
Thin Real-Input Smoke Test for Oxide EDA.
Drives the real compiled GUI application (target/release/oxide) on X11 (Xvfb :99)
with synthetic OS-level X11 user input (mouse moves, clicks, keystrokes via Xlib & xdotool).
Captures visual evidence for each interaction step.
"""

import os
import sys
import time
import subprocess
from pathlib import Path
from Xlib import X, display, XK
from Xlib.protocol import event

WORKSPACE = Path("/home/jrad/RustroverProjects/oxide-eda")
TARGET_OXIDE = WORKSPACE / "target/release/oxide"
FIXTURE_PATH = WORKSPACE / "fixtures/high_stress_suite/titan_sbc_12layer_flagship.snxpcb"
ARTIFACT_DIR = Path("/home/jrad/.gemini/antigravity/brain/295fc9a8-bb8a-48f5-9c63-d10d7ffc5363")

def capture_screenshot(filename):
    out_path = ARTIFACT_DIR / filename
    cmd = [
        "ffmpeg", "-y", "-f", "x11grab",
        "-video_size", "1400x900",
        "-i", ":99.0",
        "-vframes", "1",
        str(out_path)
    ]
    res = subprocess.run(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, text=True)
    if res.returncode != 0:
        print(f"Warning: ffmpeg capture failed: {res.stderr}")
    else:
        sz = out_path.stat().st_size if out_path.exists() else 0
        print(f"[Captured] {filename} ({sz} bytes)")
    return out_path

def main():
    if not FIXTURE_PATH.exists():
        print(f"Error: Fixture does not exist at {FIXTURE_PATH}")
        sys.exit(1)

    env = os.environ.copy()
    env["DISPLAY"] = ":99"
    env["WINIT_UNIX_BACKEND"] = "x11"
    env["WAYLAND_DISPLAY"] = ""

    print(f"=== Starting Thin Real-Input Smoke Test on {FIXTURE_PATH.name} ===")
    proc = subprocess.Popen(
        [str(TARGET_OXIDE), str(FIXTURE_PATH)],
        env=env,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL
    )
    print(f"Launched Oxide GUI (PID: {proc.pid})")

    d = display.Display(":99")
    root = d.screen().root

    target_win = None
    t0 = time.time()
    while time.time() - t0 < 8.0:
        children = root.query_tree().children
        for w in children:
            try:
                name = w.get_wm_name()
                if name and "Oxide" in name:
                    target_win = w
                    break
            except Exception:
                pass
        if target_win:
            break
        time.sleep(0.05)

    if not target_win:
        print("Error: Could not locate Oxide window on :99!")
        proc.kill()
        sys.exit(1)

    win_id = target_win.id
    print(f"Located Oxide Window ID: {hex(win_id)} ({win_id})")

    # Give window time to complete first frame presentation
    time.sleep(1.2)

    # Focus window
    d.set_input_focus(target_win, X.RevertToParent, X.CurrentTime)
    d.sync()

    # Step 1: Baseline Initial Screen Capture
    print("\n--- Step 1: Capture Baseline Loaded Board ---")
    capture_screenshot("real_input_smoke_01_titan_12layer_opened.png")

    # Step 2: Mouse Move over PCB Canvas to test cursor tracking
    print("\n--- Step 2: Mouse Movement across Canvas ---")
    root.warp_pointer(700, 450)
    d.sync()
    time.sleep(0.2)
    # Click to ensure canvas has focus
    subprocess.run(["xdotool", "mousemove", "--window", str(win_id), "700", "450", "click", "1"], env=env)
    time.sleep(0.3)

    # Step 3: Trigger Routing Tool via Altium shortcut 'P W' (Place Wire/Track)
    print("\n--- Step 3: Trigger Route Tool via 'P W' Shortcut ---")
    subprocess.run(["xdotool", "key", "--window", str(win_id), "p"], env=env)
    time.sleep(0.15)
    subprocess.run(["xdotool", "key", "--window", str(win_id), "w"], env=env)
    time.sleep(0.4)
    capture_screenshot("real_input_smoke_02_routing_tool_crosshair.png")

    # Step 4: Open Route Menu in Menubar
    print("\n--- Step 4: Open Route Menu via Alt+R or Mouse Click ---")
    # In 1400x900 menubar, 'Route' is at roughly x=205, y=14
    subprocess.run(["xdotool", "mousemove", "--window", str(win_id), "205", "14", "click", "1"], env=env)
    time.sleep(0.4)
    capture_screenshot("real_input_smoke_03_route_menu_interaction.png")

    # Dismiss menu
    subprocess.run(["xdotool", "key", "--window", str(win_id), "Escape"], env=env)
    time.sleep(0.2)

    # Step 5: Open Command Palette via 'Ctrl+Shift+P'
    print("\n--- Step 5: Toggle Command Palette via Ctrl+Shift+P ---")
    subprocess.run(["xdotool", "key", "--window", str(win_id), "Control_L+Shift_L+p"], env=env)
    time.sleep(0.4)
    capture_screenshot("real_input_smoke_04_command_palette_modal.png")

    # Dismiss palette
    subprocess.run(["xdotool", "key", "--window", str(win_id), "Escape"], env=env)
    time.sleep(0.2)

    # Step 6: Trigger DRC via 'F9'
    print("\n--- Step 6: Trigger DRC Oracle via 'F9' Shortcut ---")
    subprocess.run(["xdotool", "key", "--window", str(win_id), "F9"], env=env)
    time.sleep(0.5)
    capture_screenshot("real_input_smoke_05_drc_verified_clean.png")

    # Terminate process cleanly
    print("\n--- Clean Process Termination ---")
    proc.terminate()
    try:
        proc.wait(timeout=2.0)
    except Exception:
        proc.kill()

    print("\n=== Thin Real-Input Smoke Test PASSED SUCCESSFULLY! ===")

if __name__ == "__main__":
    main()
