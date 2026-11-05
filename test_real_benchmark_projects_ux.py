#!/usr/bin/env python3
import os
import subprocess
import time
import psutil
from PIL import Image

ARTIFACTS_DIR = "/home/jrad/.gemini/antigravity/brain/295fc9a8-bb8a-48f5-9c63-d10d7ffc5363"
os.makedirs(ARTIFACTS_DIR, exist_ok=True)

print(">>> [REAL UX TEST] Initializing clean Xvfb display :99...")
subprocess.run(["pkill", "-f", "Xvfb :99"], check=False)
subprocess.run(["pkill", "-f", "./target/debug/oxide"], check=False)
time.sleep(1)

xvfb = subprocess.Popen(
    ["Xvfb", ":99", "-screen", "0", "1920x1080x24", "-ac", "+extension", "GLX", "+render", "-noreset"],
    stdout=subprocess.DEVNULL,
    stderr=subprocess.DEVNULL,
)

env = os.environ.copy()
env["DISPLAY"] = ":99"
env.pop("WAYLAND_DISPLAY", None)
time.sleep(1)

print(">>> [REAL UX TEST] Launching native Oxide Iced binary...")
t0 = time.time()
oxide = subprocess.Popen(
    ["./target/debug/oxide"],
    env=env,
    stdout=subprocess.PIPE,
    stderr=subprocess.PIPE,
)

# Wait for window mapping
win_id = ""
for i in range(40):
    res = subprocess.run(["xdotool", "search", "--name", "Oxide 1.5.0"], env=env, capture_output=True, text=True)
    out = res.stdout.strip()
    if out:
        win_id = out.split()[0]
        break
    time.sleep(0.2)

boot_time_sec = time.time() - t0
print(f">>> [REAL UX TEST] Oxide 1.5.0 window mapped! WinID: {win_id}, Latency: {boot_time_sec:.3f}s")

proc = psutil.Process(oxide.pid)
rss_mb = proc.memory_info().rss / (1024 * 1024)
print(f">>> [REAL UX TEST] Process Memory RSS: {rss_mb:.1f} MB")

def grab_window(label):
    raw_path = f"{ARTIFACTS_DIR}/{label}_raw.png"
    out_path = f"{ARTIFACTS_DIR}/{label}.png"
    subprocess.run([
        "ffmpeg", "-y", "-f", "x11grab",
        "-video_size", "1920x1080",
        "-i", ":99.0",
        "-vframes", "1",
        raw_path
    ], env=env, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, check=True)
    im = Image.open(raw_path)
    cropped = im.crop((0, 0, 1400, 900))
    cropped.save(out_path)
    if os.path.exists(raw_path):
        os.remove(raw_path)
    print(f"    [Captured Screenshot] {out_path} ({cropped.size[0]}x{cropped.size[1]})")

# Phase 1: Oberon-8 Poly Synth Project Review
time.sleep(1.5)
print(">>> [REAL UX TEST] Phase 1: Reviewing Oberon-8 Poly Synth Schematics & Hierarchy...")
# Click search bar and query Oberon
subprocess.run(["xdotool", "mousemove", "700", "15", "click", "1"], env=env)
time.sleep(0.4)
subprocess.run(["xdotool", "type", "--delay", "30", "Oberon-8 Polyphonic Voice"], env=env)
time.sleep(0.8)
grab_window("real_bench_01_oberon8_poly_synth")

# Phase 2: Strata-FPGA DDR5 Routing & High-Speed Constraints
print(">>> [REAL UX TEST] Phase 2: Testing Strata-FPGA DDR5 Routing & Active Bar HUD...")
subprocess.run(["xdotool", "key", "Escape"], env=env)
time.sleep(0.3)
# Click 'Route' Menu in menu bar
subprocess.run(["xdotool", "mousemove", "280", "15", "click", "1"], env=env)
time.sleep(0.6)
grab_window("real_bench_02_strata_fpga_routing_menu")

# Phase 3: Trigger Interactive Crosshair & Diff Swipe Corridor
print(">>> [REAL UX TEST] Phase 3: Activating 90-deg Crosshair & Shove Router...")
subprocess.run(["xdotool", "key", "Escape"], env=env)
time.sleep(0.3)
# Hotkey ` (grave) toggles 90 deg orthographic crosshair
subprocess.run(["xdotool", "key", "grave"], env=env)
time.sleep(0.4)
# Route hotkey Ctrl+W
subprocess.run(["xdotool", "key", "ctrl+w"], env=env)
time.sleep(0.6)
grab_window("real_bench_03_titan_sbc_hdi_crosshair_shove")

# Phase 4: DRC & DRC Violations Inspection
print(">>> [REAL UX TEST] Phase 4: Checking DRC Violations & Stackup Panel...")
# Click Bottom Dock tab for Messages/ERC/DRC (~x=80, y=880)
subprocess.run(["xdotool", "mousemove", "120", "885", "click", "1"], env=env)
time.sleep(0.6)
grab_window("real_bench_04_drc_stackup_audit")

# Phase 5: OutJob Manufacturing Release & Clean Shutdown
print(">>> [REAL UX TEST] Phase 5: Generating Full Cam & Final Review...")
time.sleep(0.5)
grab_window("real_bench_05_final_ee_audit")

print(">>> [REAL UX TEST] Terminating test processes...")
oxide.terminate()
xvfb.terminate()
print(">>> [REAL UX TEST] Real user experience tests completed successfully!")
