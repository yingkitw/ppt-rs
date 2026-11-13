"""Smoke tests for the PyO3 extension (run after `maturin develop --features python`)."""

from __future__ import annotations

import ppt_rs


def test_version_matches() -> None:
    assert ppt_rs.version()
    assert ppt_rs.__version__ == ppt_rs.version()


def test_markdown_round_trip_bytes() -> None:
    md = "# Hello\n- From Python\n\n# Done\n- pip install ppt-rs"
    pptx = ppt_rs.markdown_to_pptx(md, title="Py test")
    assert pptx[:2] == b"PK"
    report = ppt_rs.validate_pptx(pptx)
    assert report["valid"] is True
    texts = ppt_rs.extract_text(pptx)
    assert len(texts) >= 2


def test_create_presentation_dict_slides() -> None:
    data = ppt_rs.create_presentation(
        "API",
        [
            {"title": "Intro", "bullets": ["pure Rust", "valid pptx"]},
            {"title": "Outro", "bullets": ["done"]},
        ],
    )
    assert data.startswith(b"PK")
