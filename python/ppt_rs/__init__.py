"""Pure Rust PowerPoint (.pptx) generation for Python (PyO3 extension)."""

from __future__ import annotations

from pathlib import Path
from typing import Any

from ppt_rs._native import (
    create_presentation,
    extract_text,
    html_to_pptx,
    markdown_to_pptx,
    validate_pptx,
    version,
)

__version__ = version()

__all__ = [
    "create_presentation",
    "extract_text",
    "html_to_file",
    "html_to_pptx",
    "markdown_to_file",
    "markdown_to_pptx",
    "validate_pptx",
    "version",
]


def markdown_to_file(markdown: str, path: str | Path, title: str | None = None) -> Path:
    """Convert Markdown to a `.pptx` file on disk."""
    out = Path(path)
    out.write_bytes(markdown_to_pptx(markdown, title))
    return out


def html_to_file(html: str, path: str | Path, title: str | None = None) -> Path:
    """Convert HTML to a `.pptx` file on disk."""
    out = Path(path)
    out.write_bytes(html_to_pptx(html, title))
    return out


def slides_to_file(
    title: str,
    slides: list[dict[str, Any] | list[Any]],
    path: str | Path,
) -> Path:
    """Build a deck from slide dicts (`title` + optional `bullets`) and write `.pptx`."""
    out = Path(path)
    out.write_bytes(create_presentation(title, slides))
    return out
