"""Thin CLI for the pip-installed `ppt-rs` package (wraps the Rust extension)."""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

from ppt_rs import html_to_file, markdown_to_file, version


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(
        prog="ppt-rs",
        description="Pure Rust PowerPoint (.pptx) — Markdown/HTML conversion",
    )
    parser.add_argument("--version", action="version", version=f"ppt-rs {version()}")
    sub = parser.add_subparsers(dest="command", required=True)

    md = sub.add_parser("md2ppt", help="Convert Markdown to PPTX")
    md.add_argument("input", type=Path, help="Markdown file")
    md.add_argument("output", type=Path, nargs="?", help="Output .pptx (default: input basename)")
    md.add_argument("--title", help="Presentation title")

    html = sub.add_parser("html2ppt", help="Convert HTML to PPTX")
    html.add_argument("input", type=Path, help="HTML file")
    html.add_argument("output", type=Path, nargs="?", help="Output .pptx")
    html.add_argument("--title", help="Presentation title")

    args = parser.parse_args(argv)

    if args.command == "md2ppt":
        text = args.input.read_text(encoding="utf-8")
        out = args.output or args.input.with_suffix(".pptx")
        markdown_to_file(text, out, args.title)
        print(out)
        return 0

    if args.command == "html2ppt":
        text = args.input.read_text(encoding="utf-8")
        out = args.output or args.input.with_suffix(".pptx")
        html_to_file(text, out, args.title)
        print(out)
        return 0

    parser.error("unknown command")
    return 1


if __name__ == "__main__":
    sys.exit(main())
