/*
  build.rs for shapelib-rs shape-contour cv-contour
*/

fn main() {
  println!("cargo:rustc-link-search=./shapelib/lib");
  println!("cargo:rustc-link-lib=shapelib_i");
}
