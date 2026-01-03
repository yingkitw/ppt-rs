#[cfg(feature = "web2ppt")]
#[test]
fn test_image_from_url() {
    use ppt_rs::generator::Image;
    use ppt_rs::elements::Position;

    // Use a reliable placeholder image service
    let url = "https://via.placeholder.com/150.png";
    let img = Image::from_url(url, 1500000, 1500000, "PNG");
    
    // Check metadata
    assert_eq!(img.width, 1500000);
    assert_eq!(img.height, 1500000);
    assert_eq!(img.format, "PNG");
    
    // Check fetching (this requires network)
    // We only test if get_bytes runs without panic, 
    // and returns Some(...) if network is available and url is valid.
    // To avoid flaky tests in offline environment, we might skip the assertion or handle None.
    // But for verification now, let's try to see if it works.
    let bytes = img.get_bytes();
    if let Some(data) = bytes {
        assert!(!data.is_empty());
        println!("Successfully fetched {} bytes", data.len());
    } else {
        println!("Failed to fetch image or offline");
    }
}
