use std::fs::File;
use std::io::Write;

/**
    Writes a PPM file locally

    1. pixels are written out in rows with pixels from left to right
    2. the rows are written out from top to bottom
    3. each of the rgb components range from 0.0 to 1.0
    4. red goes from fully off (black)to fully on (bright red) from left to right.
        green goes from black at the bottom to fully on at the top. red + green make yellow
        so we should expect the upper right corner to be yellow
 */
pub fn write_ppm_file(file_name: String) {
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    // Setup file
    let mut file = File::create(file_name).expect("Unable to create file");
    file.write_all("P3\n".as_bytes()).expect("TODO: panic message");
    file.write_all(format!("{} {}\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes()).expect("TODO: panic message");
    file.write_all("255\n".as_bytes()).expect("TODO: panic message");

    // Render
    const SOME_CONST: f32 = 255.999;
    let b: i32 = (SOME_CONST * 0.25) as i32;
    for g in (0..IMAGE_HEIGHT).rev() {
        println!("Scanlines remaining: {}", g);
        for r in 0..IMAGE_WIDTH {
            file.write_all(format!("{} {} {}\n", r, g, b).as_bytes()).expect("TODO: panic message");
        }
    }

    println!("Done writing to PPM file.");
}