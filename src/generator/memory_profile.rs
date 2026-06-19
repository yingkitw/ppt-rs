//! Memory and performance profiling helpers for large presentations

use std::time::{Duration, Instant};

use super::slide_content::SlideContent;
use super::{create_pptx_lazy_to_writer, create_pptx_with_content, LazySlideSource};
use std::io::Cursor;

/// Metrics collected while generating a presentation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GenerationMetrics {
    pub slide_count: usize,
    pub output_bytes: usize,
    pub slide_payload_bytes: usize,
    pub duration: Duration,
}

impl GenerationMetrics {
    /// Output size per slide in bytes.
    pub fn bytes_per_slide(&self) -> usize {
        if self.slide_count == 0 {
            0
        } else {
            self.output_bytes / self.slide_count
        }
    }
}

/// Estimate in-memory slide payload size from slide text and notes.
pub fn estimate_slide_payload(slides: &[SlideContent]) -> usize {
    slides.iter().fold(0, |total, slide| {
        let title_bytes = slide.title.len();
        let bullet_bytes: usize = slide.bullets.iter().map(|b| b.text.len()).sum();
        let notes_bytes = slide.notes.as_ref().map(|n| n.len()).unwrap_or(0);
        total + title_bytes + bullet_bytes + notes_bytes
    })
}

/// Pre-size the output ZIP buffer to reduce reallocations during generation.
pub fn estimate_output_capacity(slide_count: usize, slides: Option<&[SlideContent]>) -> usize {
    const BASE_BYTES: usize = 32_768;
    const PER_SLIDE_BYTES: usize = 4_096;

    let mut cap = BASE_BYTES + slide_count.saturating_mul(PER_SLIDE_BYTES);

    if let Some(slides) = slides {
        cap += estimate_slide_payload(slides);
        for slide in slides {
            cap += slide.charts.len().saturating_mul(8_192);
            for image in &slide.images {
                if let Some(bytes) = image.get_bytes() {
                    cap += bytes.len();
                }
            }
        }
    }

    cap.max(8_192)
}

/// Profile an eager (all-slides-in-memory) generation path.
pub fn profile_eager_generation(title: &str, slides: Vec<SlideContent>) -> GenerationMetrics {
    let slide_count = slides.len();
    let slide_payload_bytes = estimate_slide_payload(&slides);
    let start = Instant::now();
    let output = create_pptx_with_content(title, slides).expect("pptx generation");
    let duration = start.elapsed();

    GenerationMetrics {
        slide_count,
        output_bytes: output.len(),
        slide_payload_bytes,
        duration,
    }
}

struct IndexedSlideSource {
    slides: Vec<SlideContent>,
}

impl LazySlideSource for IndexedSlideSource {
    fn slide_count(&self) -> usize {
        self.slides.len()
    }

    fn generate_slide(&self, index: usize) -> Option<SlideContent> {
        self.slides.get(index).cloned()
    }
}

/// Profile the lazy slide loading generation path.
pub fn profile_lazy_generation(title: &str, slides: Vec<SlideContent>) -> GenerationMetrics {
    let slide_count = slides.len();
    let slide_payload_bytes = estimate_slide_payload(&slides);
    let start = Instant::now();
    let cursor = Cursor::new(Vec::new());
    let source = Box::new(IndexedSlideSource { slides });
    let output = create_pptx_lazy_to_writer(cursor, title, source, None)
        .expect("lazy pptx generation")
        .into_inner();
    let duration = start.elapsed();

    GenerationMetrics {
        slide_count,
        output_bytes: output.len(),
        slide_payload_bytes,
        duration,
    }
}

/// Build simple title/bullet slides for profiling.
pub fn sample_slides(count: usize) -> Vec<SlideContent> {
    (0..count)
        .map(|index| {
            let title = format!("Slide {index}");
            let bullet_a = format!("Bullet A on slide {index}");
            let bullet_b = format!("Bullet B on slide {index}");
            SlideContent::new(&title)
                .add_bullet(&bullet_a)
                .add_bullet(&bullet_b)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estimate_slide_payload() {
        let slides = sample_slides(2);
        let bytes = estimate_slide_payload(&slides);
        assert!(bytes > 0);
    }

    #[test]
    fn test_profile_eager_10_slides() {
        let metrics = profile_eager_generation("Profile", sample_slides(10));
        assert_eq!(metrics.slide_count, 10);
        assert!(metrics.output_bytes > 1_000);
        assert!(metrics.duration < Duration::from_secs(5));
    }
}
