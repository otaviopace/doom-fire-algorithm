extern "C" {
    fn logit(ptr: *const u8);
    fn getFireIntensity(pixelIndex: u32) -> u32;
    fn getFireColorFromPalette(fireIntensity: u32, prop: *const u8) -> u32;
    fn fillCanvas(colorString: *const u8);
    fn fillRect(column: u32, row: u32, pixelSize: u32);
}

#[no_mangle]
pub unsafe extern fn render_fire(fire_height: u32, fire_width: u32) {
    let pixel_size = 4;
    let column = 200;
    let row = 80;
    // for row in 0..fire_height {
    //     for column in 0..fire_width {
            let pixel_index = column + (fire_width * row);
            let fire_intensity = getFireIntensity(pixel_index);
            let color_string = &format!(
                "{},{},{}",
                getFireColorFromPalette(fire_intensity, "r".as_ptr()),
                getFireColorFromPalette(fire_intensity, "g".as_ptr()),
                getFireColorFromPalette(fire_intensity, "b".as_ptr())
            );

            logit(format!("color_string: {}", color_string).as_ptr());

            fillCanvas(color_string.as_ptr());
            fillRect(column, row, pixel_size);
    //     }
    // }

}
