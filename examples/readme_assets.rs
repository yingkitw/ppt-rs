//! Regenerate README sample deck + PNG previews (no PPTX import round-trip).
//!
//! ```bash
//! cargo run --release --example readme_assets --features markdown
//! ```

use ppt_rs::api::Presentation;
use ppt_rs::exc::PptxError;
use std::path::Path;

fn main() -> Result<(), PptxError> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let md_path = root.join("samples/readme-slides.md");
    let pptx_path = root.join("samples/readme-slides.pptx");
    let png_dir = root.join("docs/assets/readme-slides-render");

    let md = std::fs::read_to_string(&md_path)
        .map_err(|e| PptxError::Io(e))?;
    let slides = ppt_rs::parse_markdown(&md).map_err(PptxError::Generic)?;

    let mut pres = Presentation::with_title("ppt-rs sample");
    for slide in slides {
        pres = pres.add_slide(slide);
    }

    std::fs::write(&pptx_path, pres.build()?)?;
    if png_dir.exists() {
        let _ = std::fs::remove_dir_all(&png_dir);
    }
    pres.save_as_png(&png_dir)?;

    for i in 1..=3 {
        let from = png_dir.join(format!("slide-{i}.png"));
        let to = root.join(format!("docs/assets/readme-slide-{i}.png"));
        std::fs::rename(from, to)?;
    }
    let _ = std::fs::remove_dir_all(&png_dir);

    println!("Wrote {}", pptx_path.display());
    println!("Wrote docs/assets/readme-slide-{{1,2,3}}.png");
    Ok(())
}
