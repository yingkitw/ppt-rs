//! Slide-related types and functionality

mod slide;
mod slides;
mod iterator;
mod masters;
mod layouts;
mod background;
mod transition;
mod master;
mod layout;
mod slide_layouts;
mod slide_id;
mod animation;

pub use slide::Slide;
pub use slides::Slides;
pub use iterator::SlideIterator;
pub use masters::SlideMasters;
pub use layouts::SlideLayouts;
pub use background::SlideBackground;
pub use transition::{SlideTransition, TransitionType, TransitionDirection};
pub use master::SlideMaster;
pub use layout::{SlideLayout, LayoutType};
pub use slide_layouts::SlideLayouts as PredefinedLayouts;
pub use slide_id::{SlideId, SlideIdManager};
pub use animation::{Animation, AnimationType, AnimationManager, EntranceEffect, ExitEffect, EmphasisEffect};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parts::presentation::PresentationPart;

    #[test]
    fn test_slide_new() {
        let slide = Slide::new();
        assert_eq!(slide.name(), "");
        assert_eq!(slide.shapes().len(), 0);
    }

    #[test]
    fn test_slide_name() {
        let mut slide = Slide::new();
        slide.set_name("Test Slide".to_string());
        assert_eq!(slide.name(), "Test Slide");
    }

    #[test]
    fn test_slide_background() {
        let slide = Slide::new();
        assert!(slide.background().fill().fill_type() == crate::enums::dml::FillType::NoFill);
    }

    #[test]
    fn test_slide_background_solid() {
        let mut slide = Slide::new();
        use crate::dml::color::RGBColor;
        slide.background_mut().set_solid(RGBColor::new(255, 0, 0));
        assert!(slide.background().fill().fill_type() == crate::enums::dml::FillType::Solid);
    }

    #[test]
    fn test_slide_background_gradient() {
        let mut slide = Slide::new();
        use crate::dml::color::RGBColor;
        slide.background_mut().set_gradient_linear(
            RGBColor::new(255, 0, 0),
            RGBColor::new(0, 0, 255),
        ).unwrap();
        assert!(slide.background().fill().fill_type() == crate::enums::dml::FillType::Gradient);
    }

    #[test]
    fn test_slide_transition() {
        let slide = Slide::new();
        assert_eq!(slide.transition().transition_type(), TransitionType::None);
    }

    #[test]
    fn test_slide_transition_set() {
        let mut slide = Slide::new();
        slide.transition_mut().set_transition_type(TransitionType::Fade);
        assert_eq!(slide.transition().transition_type(), TransitionType::Fade);
    }

    #[test]
    fn test_slide_transition_duration() {
        let mut slide = Slide::new();
        slide.transition_mut().set_duration(1000).unwrap();
        assert_eq!(slide.transition().duration(), 1000);
    }

    #[test]
    fn test_slide_masters_new() {
        let part = PresentationPart::new().unwrap();
        let masters = SlideMasters::new(&part);
        assert_eq!(masters.len(), 0);
        assert!(masters.is_empty());
    }

    #[test]
    fn test_slide_layouts_new() {
        use crate::parts::slide::SlideMasterPart;
        use crate::opc::packuri::PackURI;
        
        let master_part = SlideMasterPart::new(PackURI::new("/ppt/slideMasters/slideMaster1.xml").unwrap()).unwrap();
        let layouts = SlideLayouts::new(&master_part);
        assert_eq!(layouts.len(), 0);
        assert!(layouts.is_empty());
    }

    #[test]
    fn test_slides_len_empty() {
        let mut part = PresentationPart::new().unwrap();
        let slides = Slides::new(&mut part);
        assert_eq!(slides.len(), 0);
        assert!(slides.is_empty());
    }

    #[test]
    fn test_slides_add_slide_with_package() {
        let mut part = PresentationPart::new().unwrap();
        let mut package = crate::opc::package::Package::new();
        
        use crate::parts::slide::SlidePart;
        use crate::opc::packuri::PackURI;
        use crate::opc::part::Part;
        // Create a slide part (not layout part) for testing - create before borrowing part
        let slide_part = SlidePart::new(PackURI::new("/ppt/slides/slide1.xml").unwrap(), &part as &dyn Part).unwrap();
        
        let mut slides = Slides::new(&mut part);
        let result = slides.add_slide(&slide_part, &mut package);
        assert!(result.is_ok());
        assert_eq!(slides.len(), 1);
    }

    #[test]
    fn test_slides_get_with_package() {
        let mut part = PresentationPart::new().unwrap();
        let mut package = crate::opc::package::Package::new();
        
        use crate::parts::slide::SlidePart;
        use crate::opc::packuri::PackURI;
        use crate::opc::part::Part;
        // Create a slide part (not layout part) for testing - create before borrowing part
        let slide_part = SlidePart::new(PackURI::new("/ppt/slides/slide1.xml").unwrap(), &part as &dyn Part).unwrap();
        
        let mut slides = Slides::new(&mut part);
        slides.add_slide(&slide_part, &mut package).unwrap();
        
        let slide = slides.get(0, &mut package);
        assert!(slide.is_some());
        let slide = slide.unwrap();
        assert_eq!(slide.name(), "");
    }

    #[test]
    fn test_slide_add_image() {
        use crate::parts::slide::SlideLayoutPart;
        use crate::opc::packuri::PackURI;
        use crate::parts::slide::SlidePart;
        use crate::opc::part::Part;
        
        let mut package = crate::opc::package::Package::new();
        let layout_part = SlideLayoutPart::new(PackURI::new("/ppt/slideLayouts/slideLayout1.xml").unwrap()).unwrap();
        let slide_part = SlidePart::new(PackURI::new("/ppt/slides/slide1.xml").unwrap(), &layout_part as &dyn Part).unwrap();
        let mut slide = Slide::with_part(slide_part);
        
        // Create a minimal PNG image (1x1 pixel PNG)
        let png_data = vec![
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
            0x00, 0x00, 0x00, 0x0D, // IHDR chunk length
            0x49, 0x48, 0x44, 0x52, // IHDR
            0x00, 0x00, 0x00, 0x01, // width: 1
            0x00, 0x00, 0x00, 0x01, // height: 1
            0x08, 0x02, 0x00, 0x00, 0x00, // bit depth, color type, etc.
            0x90, 0x77, 0x53, 0xDE, // CRC
            0x00, 0x00, 0x00, 0x0A, // IDAT chunk length
            0x49, 0x44, 0x41, 0x54, // IDAT
            0x78, 0x9C, 0x63, 0x00, 0x01, 0x00, 0x00, 0x05, 0x00, 0x01, // compressed data
            0x0D, 0x0A, 0x2D, 0xB4, // CRC
            0x00, 0x00, 0x00, 0x00, // IEND chunk length
            0x49, 0x45, 0x4E, 0x44, // IEND
            0xAE, 0x42, 0x60, 0x82, // CRC
        ];
        
        let result = slide.add_image(png_data, "png", &mut package);
        assert!(result.is_ok());
        
        // Verify image was added to package
        let image_uri = crate::opc::packuri::PackURI::new("/ppt/media/image1.png").unwrap();
        assert!(package.get_part(&image_uri).is_some());
        
        // Verify slide has a relationship to the image
        // Check both the slide's part and the package's part
        let slide_uri = crate::opc::packuri::PackURI::new("/ppt/slides/slide1.xml").unwrap();
        let mut found_image_rel = false;
        
        // First check the package's slide part
        if let Some(package_part) = package.get_part(&slide_uri) {
            use crate::opc::part::Part;
            use crate::opc::constants::RELATIONSHIP_TYPE;
            let rels = Part::relationships(package_part);
            found_image_rel = rels.iter().any(|(_, r)| r.rel_type == RELATIONSHIP_TYPE::IMAGE);
        }
        
        // Also check the slide's own part
        if !found_image_rel {
            if let Some(part) = slide.part() {
                use crate::opc::constants::RELATIONSHIP_TYPE;
                let rels = part.relationships();
                found_image_rel = rels.iter().any(|(_, r)| r.rel_type == RELATIONSHIP_TYPE::IMAGE);
            }
        }
        
        assert!(found_image_rel, "Slide should have image relationship");
    }

    #[test]
    fn test_slide_add_image_different_formats() {
        use crate::parts::slide::SlideLayoutPart;
        use crate::opc::packuri::PackURI;
        use crate::parts::slide::SlidePart;
        use crate::opc::part::Part;
        
        let mut package = crate::opc::package::Package::new();
        let layout_part = SlideLayoutPart::new(PackURI::new("/ppt/slideLayouts/slideLayout1.xml").unwrap()).unwrap();
        let slide_part = SlidePart::new(PackURI::new("/ppt/slides/slide1.xml").unwrap(), &layout_part as &dyn Part).unwrap();
        let mut slide = Slide::with_part(slide_part);
        
        // Test JPEG
        let jpeg_data = vec![0xFF, 0xD8, 0xFF, 0xE0]; // Minimal JPEG header
        let result = slide.add_image(jpeg_data.clone(), "jpg", &mut package);
        assert!(result.is_ok());
        
        // Test GIF
        let gif_data = vec![0x47, 0x49, 0x46, 0x38]; // GIF header
        let result = slide.add_image(gif_data.clone(), "gif", &mut package);
        assert!(result.is_ok());
        
        // Verify both images are in package
        let jpeg_uri = crate::opc::packuri::PackURI::new("/ppt/media/image1.jpg").unwrap();
        let gif_uri = crate::opc::packuri::PackURI::new("/ppt/media/image2.gif").unwrap();
        assert!(package.get_part(&jpeg_uri).is_some());
        assert!(package.get_part(&gif_uri).is_some());
    }

    #[test]
    fn test_slide_add_image_no_part() {
        let mut package = crate::opc::package::Package::new();
        let mut slide = Slide::new();
        let image_data = vec![0x89, 0x50, 0x4E, 0x47]; // PNG header
        
        let result = slide.add_image(image_data, "png", &mut package);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("Slide has no part"));
        }
    }

    #[test]
    fn test_slide_part_accessors() {
        use crate::parts::slide::SlideLayoutPart;
        use crate::opc::packuri::PackURI;
        use crate::parts::slide::SlidePart;
        use crate::opc::part::Part;
        
        let layout_part = SlideLayoutPart::new(PackURI::new("/ppt/slideLayouts/slideLayout1.xml").unwrap()).unwrap();
        let slide_part = SlidePart::new(PackURI::new("/ppt/slides/slide1.xml").unwrap(), &layout_part as &dyn Part).unwrap();
        let mut slide = Slide::with_part(slide_part);
        
        assert!(slide.part().is_some());
        assert!(slide.part_mut().is_some());
        
        // Test that we can modify through part_mut
        if let Some(part) = slide.part_mut() {
            let uri = part.uri();
            assert_eq!(uri.as_str(), "/ppt/slides/slide1.xml");
        }
    }
}

