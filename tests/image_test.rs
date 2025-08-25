use std::fs;

#[test]
fn test_png_image() {
    let path = "./tests/fixtures/1_image.png";
    let image_bytes = fs::read(path).unwrap();

    let image = image_data::get_image(image_bytes.as_slice());

    assert_eq!(image.width(), 800);
    assert_eq!(image.height(), 600);
    assert!(!image.data().is_empty());
}

#[test]
fn test_jpg_image() {
    let path = "./tests/fixtures/2_image.jpg";
    let image_bytes = fs::read(path).unwrap();

    let image = image_data::get_image(image_bytes.as_slice());

    assert_eq!(image.width(), 807);
    assert_eq!(image.height(), 730);
    assert!(!image.data().is_empty());
}
