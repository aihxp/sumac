#!/usr/bin/env python3
"""Summarize the portable validation fixtures used by Sumac."""

from __future__ import annotations

import argparse
import json
from pathlib import Path


def collect_inventory(root: Path) -> dict:
    skills = []
    resources = []
    scripts = []
    stateful_servers = []

    for entry in sorted(root.iterdir()):
        if entry.is_dir():
            skill_md = entry / "SKILL.md"
            if skill_md.exists():
                skills.append(
                    {
                        "name": entry.name,
                        "path": str(skill_md),
                        "has_references": (entry / "references").exists(),
                        "has_scripts": (entry / "scripts").exists(),
                    }
                )
                if (entry / "references").exists():
                    resources.extend(
                        str(path)
                        for path in sorted((entry / "references").rglob("*"))
                        if path.is_file()
                    )
                if (entry / "scripts").exists():
                    scripts.extend(
                        str(path)
                        for path in sorted((entry / "scripts").rglob("*"))
                        if path.is_file()
                    )
        elif entry.is_file() and entry.name.endswith("_mcp_server.py"):
            stateful_servers.append(str(entry))

    return {
        "fixture_root": str(root),
        "skill_count": len(skills),
        "skills": skills,
        "resource_count": len(resources),
        "resources": resources,
        "script_count": len(scripts),
        "scripts": scripts,
        "stateful_server_count": len(stateful_servers),
        "stateful_servers": stateful_servers,
    }


def to_markdown(inventory: dict) -> str:
    lines = [
        "# Fixture Inventory",
        "",
        f"- Fixture root: `{inventory['fixture_root']}`",
        f"- Skills: `{inventory['skill_count']}`",
        f"- Reference files: `{inventory['resource_count']}`",
        f"- Script files: `{inventory['script_count']}`",
        f"- Stateful MCP fixtures: `{inventory['stateful_server_count']}`",
        "",
        "## Skills",
        "",
        "| Skill | References | Scripts | Path |",
        "|---|---:|---:|---|",
    ]
    for skill in inventory["skills"]:
        lines.append(
            f"| `{skill['name']}` | "
            f"{'yes' if skill['has_references'] else 'no'} | "
            f"{'yes' if skill['has_scripts'] else 'no'} | "
            f"`{skill['path']}` |"
        )

    if inventory["resources"]:
        lines.extend(["", "## Reference Files", ""])
        for path in inventory["resources"]:
            lines.append(f"- `{path}`")

    if inventory["scripts"]:
        lines.extend(["", "## Script Files", ""])
        for path in inventory["scripts"]:
            lines.append(f"- `{path}`")

    if inventory["stateful_servers"]:
        lines.extend(["", "## Stateful MCP Fixtures", ""])
        for path in inventory["stateful_servers"]:
            lines.append(f"- `{path}`")

    lines.append("")
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "root",
        nargs="?",
        default="tests/fixtures",
        help="Fixture root directory (default: tests/fixtures)",
    )
    parser.add_argument(
        "--format",
        choices=["json", "markdown"],
        default="markdown",
        help="Output format",
    )
    parser.add_argument("--output", help="Write output to a file instead of stdout")
    args = parser.parse_args()

    root = Path(args.root)
    if not root.exists():
        raise SystemExit(f"Fixture root not found: {root}")

    inventory = collect_inventory(root)
    if inventory["skill_count"] == 0:
        raise SystemExit("No skill fixtures found.")

    rendered = (
        json.dumps(inventory, indent=2)
        if args.format == "json"
        else to_markdown(inventory)
    )

    if args.output:
        Path(args.output).write_text(rendered + ("\n" if not rendered.endswith("\n") else ""), encoding="utf-8")
    else:
        print(rendered)

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
