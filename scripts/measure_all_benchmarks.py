#!/usr/bin/env python3
"""
Measure reproducible benchmarks comparing Oxide EDA vs KiCad 10 on the local machine.
Altium Designer is strictly marked as 'n/a (not installed on Linux host)'.
"""

import os
import subprocess
import time
import json
from pathlib import Path

WORKSPACE = Path("/home/jrad/RustroverProjects/oxide-eda")
TARGET_OXIDE = WORKSPACE / "target/release/oxide"
TARGET_OXIDE_CLI = WORKSPACE / "target/release/oxide-cli"

def run_timed_cmd(cmd_list):
    """Run command with /usr/bin/time -v and parse wall time and max RSS."""
    full_cmd = ["/usr/bin/time", "-v"] + cmd_list
    t0 = time.perf_counter()
    res = subprocess.run(full_cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    t1 = time.perf_counter()
    wall_sec = t1 - t0
    
    max_rss_kb = 0
    for line in res.stderr.splitlines():
        if "Maximum resident set size" in line:
            parts = line.split(":")
            if len(parts) >= 2:
                max_rss_kb = int(parts[1].strip())
    
    return {
        "cmd": " ".join(cmd_list),
        "exit_code": res.returncode,
        "wall_sec": wall_sec,
        "max_rss_mb": max_rss_kb / 1024.0,
        "stdout": res.stdout,
        "stderr": res.stderr,
    }

def measure_gui_startup(cmd_list, window_search_str):
    """Launch GUI on Xvfb (:99) and measure time until window is visible."""
    env = os.environ.copy()
    env["DISPLAY"] = ":99"
    env["WINIT_UNIX_BACKEND"] = "x11"
    env["WAYLAND_DISPLAY"] = ""
    
    t0 = time.perf_counter()
    proc = subprocess.Popen(cmd_list, env=env, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    
    t_startup = None
    max_rss_kb = 0
    timeout = 10.0
    while time.perf_counter() - t0 < timeout:
        try:
            out = subprocess.check_output(
                ["xdotool", "search", "--name", window_search_str],
                env=env,
                stderr=subprocess.DEVNULL
            ).decode().strip()
            if out:
                t_startup = time.perf_counter() - t0
                break
        except Exception:
            pass
        time.sleep(0.02)
        
    try:
        with open(f"/proc/{proc.pid}/status") as f:
            for line in f:
                if line.startswith("VmRSS:"):
                    max_rss_kb = int(line.split()[1])
    except Exception:
        pass
        
    proc.terminate()
    try:
        proc.wait(timeout=2.0)
    except Exception:
        proc.kill()
        
    return {
        "cmd": " ".join(cmd_list),
        "startup_sec": t_startup if t_startup is not None else 0.0,
        "max_rss_mb": max_rss_kb / 1024.0,
    }

def main():
    results = {}
    
    print("=== 1. Measuring GUI Startup & RSS ===")
    ox_gui = measure_gui_startup(
        [str(TARGET_OXIDE), "fixtures/verifiable/real_analog_synth_2layer.snxpcb"],
        "Oxide"
    )
    print(f"Oxide GUI: startup = {ox_gui['startup_sec']:.3f}s, RSS = {ox_gui['max_rss_mb']:.1f} MB")
    
    kicad_gui = measure_gui_startup(
        ["pcbnew", "/usr/share/kicad/demos/royalblue54L_feather/RoyalBlue54L-Feather.kicad_pcb"],
        "PCB"
    )
    print(f"KiCad GUI: startup = {kicad_gui['startup_sec']:.3f}s, RSS = {kicad_gui['max_rss_mb']:.1f} MB")
    
    results["gui_startup"] = {
        "oxide": ox_gui,
        "kicad": kicad_gui,
        "altium": {"startup_sec": "n/a", "max_rss_mb": "n/a", "note": "not installed on Linux host"}
    }
    
    print("\n=== 2. Measuring Altium Binary Import Parity ===")
    # Oxide PcbDoc
    ox_pcb_import = run_timed_cmd([
        str(TARGET_OXIDE_CLI), "import-altium",
        "--input", "examples/General_LCD(faez)/LcdKey2V1.PcbDoc",
        "--out", "/tmp/bench_oxide_lcd.snxpcb"
    ])
    print(f"Oxide PcbDoc Import: {ox_pcb_import['wall_sec']:.3f}s, RSS = {ox_pcb_import['max_rss_mb']:.1f} MB")

    # KiCad PcbDoc
    kicad_pcb_import = run_timed_cmd([
        "kicad-cli", "pcb", "import", "--format", "altium",
        "--output", "/tmp/bench_kicad_lcd.kicad_pcb",
        "examples/General_LCD(faez)/LcdKey2V1.PcbDoc"
    ])
    print(f"KiCad PcbDoc Import: {kicad_pcb_import['wall_sec']:.3f}s, RSS = {kicad_pcb_import['max_rss_mb']:.1f} MB")

    # Oxide SchDoc
    ox_sch_import = run_timed_cmd([
        str(TARGET_OXIDE_CLI), "import-altium",
        "--input", "examples/General_LCD(faez)/LcdKey2V1.SchDoc",
        "--out", "/tmp/bench_oxide_lcd_sch.snxsch"
    ])
    print(f"Oxide SchDoc Import: {ox_sch_import['wall_sec']:.3f}s, RSS = {ox_sch_import['max_rss_mb']:.1f} MB")

    results["altium_import"] = {
        "oxide_pcb": ox_pcb_import,
        "kicad_pcb": kicad_pcb_import,
        "oxide_sch": ox_sch_import,
        "kicad_sch": {"wall_sec": "n/a (no CLI importer)", "max_rss_mb": "n/a"},
        "altium": {"wall_sec": "n/a", "max_rss_mb": "n/a", "note": "not installed on Linux host"}
    }

    print("\n=== 3. Measuring DRC Execution Duration & Peak RSS ===")
    # T1: 2-layer
    ox_drc_t1 = run_timed_cmd([
        str(TARGET_OXIDE_CLI), "drc",
        "--board", "fixtures/verifiable/real_analog_synth_2layer.snxpcb"
    ])
    kicad_drc_t1 = run_timed_cmd([
        "kicad-cli", "pcb", "drc",
        "/usr/share/kicad/demos/ecc83/ecc83-pp.kicad_pcb",
        "--output", "/tmp/bench_kicad_drc_t1.rpt"
    ])
    print(f"DRC T1 (2-Layer): Oxide = {ox_drc_t1['wall_sec']:.3f}s ({ox_drc_t1['max_rss_mb']:.1f} MB), KiCad = {kicad_drc_t1['wall_sec']:.3f}s ({kicad_drc_t1['max_rss_mb']:.1f} MB)")

    # T2: 4-layer
    ox_drc_t2 = run_timed_cmd([
        str(TARGET_OXIDE_CLI), "drc",
        "--board", "fixtures/verifiable/real_fpga_ddr5_4layer.snxpcb"
    ])
    kicad_drc_t2 = run_timed_cmd([
        "kicad-cli", "pcb", "drc",
        "/usr/share/kicad/demos/royalblue54L_feather/RoyalBlue54L-Feather.kicad_pcb",
        "--output", "/tmp/bench_kicad_drc_t2.rpt"
    ])
    print(f"DRC T2 (4-Layer): Oxide = {ox_drc_t2['wall_sec']:.3f}s ({ox_drc_t2['max_rss_mb']:.1f} MB), KiCad = {kicad_drc_t2['wall_sec']:.3f}s ({kicad_drc_t2['max_rss_mb']:.1f} MB)")

    # T3: 6-layer
    ox_drc_t3 = run_timed_cmd([
        str(TARGET_OXIDE_CLI), "drc",
        "--board", "fixtures/verifiable/real_bga_fanout_6layer.snxpcb"
    ])
    kicad_drc_t3 = run_timed_cmd([
        "kicad-cli", "pcb", "drc",
        "/usr/share/kicad/demos/multichannel/multichannel_mixer.kicad_pcb",
        "--output", "/tmp/bench_kicad_drc_t3.rpt"
    ])
    print(f"DRC T3 (6-Layer): Oxide = {ox_drc_t3['wall_sec']:.3f}s ({ox_drc_t3['max_rss_mb']:.1f} MB), KiCad = {kicad_drc_t3['wall_sec']:.3f}s ({kicad_drc_t3['max_rss_mb']:.1f} MB)")

    # Real Imported Altium Board DRC
    ox_drc_altium = run_timed_cmd([
        str(TARGET_OXIDE_CLI), "drc",
        "--board", "/tmp/bench_oxide_lcd.snxpcb"
    ])
    kicad_drc_altium = run_timed_cmd([
        "kicad-cli", "pcb", "drc",
        "/tmp/bench_kicad_lcd.kicad_pcb",
        "--output", "/tmp/bench_kicad_drc_altium.rpt"
    ])
    print(f"DRC Imported Altium: Oxide = {ox_drc_altium['wall_sec']:.3f}s ({ox_drc_altium['max_rss_mb']:.1f} MB), KiCad = {kicad_drc_altium['wall_sec']:.3f}s ({kicad_drc_altium['max_rss_mb']:.1f} MB)")

    print("\n=== 4. Measuring CAM Export Throughput & Peak RSS ===")
    os.makedirs("/tmp/bench_oxide_cam_t1", exist_ok=True)
    os.makedirs("/tmp/bench_kicad_cam_feather", exist_ok=True)
    os.makedirs("/tmp/bench_oxide_cam_altium", exist_ok=True)
    os.makedirs("/tmp/bench_kicad_cam_altium", exist_ok=True)

    ox_cam_t1 = run_timed_cmd([
        str(TARGET_OXIDE_CLI), "cam",
        "--board", "fixtures/verifiable/real_analog_synth_2layer.snxpcb",
        "--out-dir", "/tmp/bench_oxide_cam_t1"
    ])
    kicad_cam_feather = run_timed_cmd([
        "kicad-cli", "pcb", "export", "gerbers",
        "/usr/share/kicad/demos/royalblue54L_feather/RoyalBlue54L-Feather.kicad_pcb",
        "--output", "/tmp/bench_kicad_cam_feather/"
    ])
    ox_cam_altium = run_timed_cmd([
        str(TARGET_OXIDE_CLI), "cam",
        "--board", "/tmp/bench_oxide_lcd.snxpcb",
        "--out-dir", "/tmp/bench_oxide_cam_altium"
    ])
    kicad_cam_altium = run_timed_cmd([
        "kicad-cli", "pcb", "export", "gerbers",
        "/tmp/bench_kicad_lcd.kicad_pcb",
        "--output", "/tmp/bench_kicad_cam_altium/"
    ])
    print(f"CAM T1: Oxide = {ox_cam_t1['wall_sec']:.3f}s ({ox_cam_t1['max_rss_mb']:.1f} MB)")
    print(f"CAM Feather: KiCad = {kicad_cam_feather['wall_sec']:.3f}s ({kicad_cam_feather['max_rss_mb']:.1f} MB)")
    print(f"CAM Imported Altium: Oxide = {ox_cam_altium['wall_sec']:.3f}s ({ox_cam_altium['max_rss_mb']:.1f} MB), KiCad = {kicad_cam_altium['wall_sec']:.3f}s ({kicad_cam_altium['max_rss_mb']:.1f} MB)")

    out_json = WORKSPACE / "benchmark_results.json"
    with open(out_json, "w") as f:
        json.dump({
            "gui_startup": results["gui_startup"],
            "altium_import": results["altium_import"],
            "drc": {
                "t1_oxide": ox_drc_t1, "t1_kicad": kicad_drc_t1,
                "t2_oxide": ox_drc_t2, "t2_kicad": kicad_drc_t2,
                "t3_oxide": ox_drc_t3, "t3_kicad": kicad_drc_t3,
                "altium_oxide": ox_drc_altium, "altium_kicad": kicad_drc_altium,
            },
            "cam": {
                "t1_oxide": ox_cam_t1,
                "feather_kicad": kicad_cam_feather,
                "altium_oxide": ox_cam_altium,
                "altium_kicad": kicad_cam_altium,
            }
        }, f, indent=2)
    print(f"\nAll benchmark results saved to {out_json}")

if __name__ == "__main__":
    main()

