#!/usr/bin/env python3
"""Cross-platform startup benchmark for sxmc."""

from __future__ import annotations

import os
import statistics
import subprocess
import sys
import time
from pathlib import Path


def median_ms(samples: list[float]) -> int:
    return int(round(statistics.median(samples)))


def time_command(command: list[str]) -> float:
    start = time.perf_counter()
    subprocess.run(
        command,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        check=True,
    )
    return (time.perf_counter() - start) * 1000.0


def main() -> int:
    default_binary = "target\\debug\\sxmc.exe" if os.name == "nt" else "target/debug/sxmc"
    sxmc = os.environ.get("SXMC")
    if not sxmc:
        sxmc = default_binary if Path(default_binary).exists() else "sxmc"
    runs = int(os.environ.get("RUNS", "5"))
    output = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("startup-benchmark.md")

    version_samples = [time_command([sxmc, "--version"]) for _ in range(runs)]
    help_samples = [time_command([sxmc, "--help"]) for _ in range(runs)]

    lines = [
        "# sxmc startup benchmark",
        "",
        f"- **When:** {time.strftime('%Y-%m-%dT%H:%M:%S%z')}",
        f"- **sxmc:** {sxmc}",
        f"- **Runs per timing:** {runs} (median ms)",
        "",
        "| Startup path | Median ms |",
        "|--------------|-----------|",
        f"| `sxmc --version` | {median_ms(version_samples)} |",
        f"| `sxmc --help` | {median_ms(help_samples)} |",
        "",
        "This startup-focused benchmark complements `scripts/benchmark_cli.sh`.",
        "Use it when you want to isolate process startup or platform-specific regressions.",
        "",
    ]

    output.write_text("\n".join(lines), encoding="utf-8")
    print(f"Wrote {output}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
