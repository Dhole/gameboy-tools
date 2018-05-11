extern crate image;

use image::GenericImage;

pub fn img8x8_to_gbtile(img: image::SubImage<image::GrayImage>) -> Vec<u8> {
    assert!(img.dimensions() == (8, 8));
    let mut gbtile = Vec::new();
    for y in 0..8 {
        let mut lsb = 0 as u8;
        let mut msb = 0 as u8;
        for x in 0..8 {
            let p = img.get_pixel(x, y).data[0];
            let (low, high) = if p < 64 {
                (1, 1)
            } else if p < 128 {
                (0, 1)
            } else if p < 192 {
                (1, 0)
            } else {
                (0, 0)
            };
            lsb = lsb | (low << (7 - x));
            msb = msb | (high << (7 - x));
        }
        gbtile.push(lsb);
        gbtile.push(msb);
    }
    return gbtile;
}
