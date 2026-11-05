#!/usr/bin/env bash
set -euo pipefail

ARTIFACTS_DIR="/home/jrad/.gemini/antigravity/brain/295fc9a8-bb8a-48f5-9c63-d10d7ffc5363"
mkdir -p "$ARTIFACTS_DIR"

echo "=== 1. Starting Xvfb display :99 ==="
pkill -f "Xvfb :99" || true
pkill -f "./target/debug/oxide" || true
sleep 1

Xvfb :99 -screen 0 1920x1080x24 -ac +extension GLX +render -noreset > /dev/null 2>&1 &
XVFB_PID=$!
export DISPLAY=:99
unset WAYLAND_DISPLAY

for i in {1..20}; do
    if xdpyinfo >/dev/null 2>&1; then
        echo "Xvfb :99 ready"
        break
    fi
    sleep 0.2
done

echo "=== 2. Launching native Oxide desktop binary ==="
./target/debug/oxide > /tmp/oxide_desktop.log 2>&1 &
OXIDE_PID=$!

WIN_ID=""
for i in {1..40}; do
    WIN_ID=$(xdotool search --name "Oxide 1.5.0" 2>/dev/null || true)
    if [ -n "$WIN_ID" ]; then
        echo "Found native Oxide window: $WIN_ID"
        break
    fi
    sleep 0.3
done

sleep 2.5

# Test 1: Oberon-8 Poly Synth Review
echo "=== 3. Testing Project 1: Oberon-8 Analog Polyphonic Synth ==="
xdotool mousemove 700 15 click 1
sleep 0.4
xdotool type --delay 30 "Oberon-8 Polyphonic Synthesizer"
sleep 0.8
ffmpeg -y -f x11grab -video_size 1400x900 -i :99.0+0,0 -vframes 1 "${ARTIFACTS_DIR}/bench_01_oberon8_poly_synth.png"

# Test 2: Strata-FPGA DDR5 Routing & High-Speed Controls
echo "=== 4. Testing Project 2: Strata-FPGA DDR5 Kintex-UltraScale+ Platform ==="
xdotool key Escape
sleep 0.4
# Open Route Menu
xdotool mousemove 280 15 click 1
sleep 0.8
ffmpeg -y -f x11grab -video_size 1400x900 -i :99.0+0,0 -vframes 1 "${ARTIFACTS_DIR}/bench_02_strata_fpga_ddr5_routing.png"

# Test 3: Titan-SBC 12-Layer Board & 90-degree Crosshair
echo "=== 5. Testing Project 3: Titan-SBC 12-Layer Any-Layer HDI Stackup & Crosshair ==="
xdotool key Escape
sleep 0.3
# Hotkey ` toggles large 90 degree crosshair
xdotool key "grave"
sleep 0.4
# Interactive Route command Ctrl+W
xdotool key "ctrl+w"
sleep 0.8
ffmpeg -y -f x11grab -video_size 1400x900 -i :99.0+0,0 -vframes 1 "${ARTIFACTS_DIR}/bench_03_titan_sbc_12layer_crosshair.png"

# Test 4: Preferences Modal / High-Speed Clearance & Rules Config
echo "=== 6. Testing Preferences & High-Speed Rules ==="
xdotool key Escape
sleep 0.3
xdotool key "ctrl+comma"
sleep 1.0
ffmpeg -y -f x11grab -video_size 1400x900 -i :99.0+0,0 -vframes 1 "${ARTIFACTS_DIR}/bench_04_preferences_rules_config.png"

echo "=== 7. Cleanup ==="
kill $OXIDE_PID || true
kill $XVFB_PID || true

echo "=== Benchmark Visual Suite Completed ==="
