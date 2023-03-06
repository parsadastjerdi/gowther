mod ppm;

use crate::ppm::write_ppm_file;

fn main() {
    write_ppm_file("output.ppm".to_string());
}
