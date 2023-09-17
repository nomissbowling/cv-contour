#![doc(html_root_url = "https://docs.rs/cv-contour/0.2.0")]
//! Rust crate cv-contour supports ESRI J shapefile
//!
//! # Requirements
//!
//! - [ shape-contour ]( https://crates.io/crates/shape-contour )
//! - [ shapelib-rs ]( https://crates.io/crates/shapelib-rs )
//! - [ OSGeo ]( https://OSGeo.org/ )
//! - [ OSGeo shapelib (C) ]( https://github.com/OSGeo/shapelib )
//! - [ shapelib ]( http://shapelib.maptools.org/ )
//! - [ ESRI J shapefile ]( https://www.esrij.com/products/japan-shp/ )
//!
//! link shapelib_i.lib
//!

pub mod cvc;

#[cfg(test)]
mod tests {
  use super::cvc::{self, shape, contours};
  use std::path::PathBuf;

  /// with [-- --nocapture] or with [-- --show-output]
  #[test]
  fn check_cv_contour() {
    let rp = "../shapelib-rs";
    let s_path: String = if cfg!(docsrs) {
      std::env::var("OUT_DIR").unwrap()
    }else{
      rp.to_string()
    }; // to keep lifetime
    let o_path: &str = s_path.as_str();
    if o_path != rp { return; }
    let bp = PathBuf::from(o_path).join("shp").join("ESRIJ_com_japan_ver84");
    println!("{}", bp.join("japan_ver84.shp").to_str().unwrap());
    println!("{}", bp.join("japan_ver84.dbf").to_str().unwrap());
    let s = bp.join("japan_ver84"); // to keep lifetime
    let u = s.to_str().unwrap(); // to keep lifetime
    let shp = shape::ShapeF::new(u, "cp932").unwrap();
    shp.disp_record_inf().unwrap();
    let sci = shp.get_shp_contours(false).unwrap();
    drop(shp);
    let mut gci = contours::GrpContoursInf::new(sci).unwrap();
    gci.grp_contours.clear();
    gci.get_grp_contours(50.0, 26, 343, false).unwrap(); // 50.0 (1600, 1200)
    assert_eq!(gci.grp_contours.len(), 1);
    gci.grp_contours.clear();
    gci.get_grp_contours(50.0, 0, 0, false).unwrap(); // 1-47, 0
    assert_eq!(gci.grp_contours.len(), 1907);
    gci.grp_scaled_contours.clear();
    gci.whole_scaled().unwrap();
    assert_eq!(gci.grp_scaled_contours.len(), 1907);
    let contours = gci.grp_scaled_contours.get_mut(&0).unwrap();
    assert_eq!(cvc::cv_contours(contours).unwrap().len(), 1);
  }
}
