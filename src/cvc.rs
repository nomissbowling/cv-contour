//! cvc
//!
//! # Requirements
//!
//! - [ shape-contour ]( https://crates.io/crates/shape-contour )
//! - [ shapelib-rs ]( https://crates.io/crates/shapelib-rs )
//!

use std::error::Error;

pub use shapelib::shape; // {Pt2d, ShpContoursInf};
pub use shape_contour::contours; // {Pt, GrpContoursInf};

// use opencv::{Result, prelude::*, core, videoio, highgui, imgcodecs, imgproc};
use opencv::{Result, prelude::*, core, imgproc};

/// shape Contours2 to cv Contours
pub fn cv_contours(src: &mut Vec<Vec<contours::Pt>>) ->
  Result<core::Vector<core::Vector<core::Point_<i32>>>, Box<dyn Error>> {
  let mut dst = core::Vector::<core::Vector::<core::Point_<i32>>>::new();
  for r_ct in src {
    // from_slice convert Vec<contours::Pt> to core::Vector<core::Point_<i32>>
    let p_ct = &mut r_ct[0] as *mut contours::Pt as *mut core::Point_<i32>;
    let s_ct = unsafe { std::slice::from_raw_parts(p_ct, r_ct.len()) };
    let contour = core::Vector::<core::Point_<i32>>::from_slice(s_ct);
    dst.push(contour);
  }
  Ok(dst)
}

/// &mut Mat or &mut impl core::ToInputOutputArray
pub fn draw_polys(dst: &mut Mat, mode: bool,
  contours: &core::Vector<core::Vector<core::Point_<i32>>>,
  hierarchy: &core::Vector<core::VecN<i32, 4>>,
  ofs: core::Point, col: core::VecN<f64, 4>) -> Result<(), Box<dyn Error>> {
  let contour_idx: i32 = -1; // ( <0: all contours, =>0: index )
  let th: i32 = if mode { -1 } else { 2 }; // thickness ( <0: fill, >0: line )
  let lt: i32 = imgproc::LINE_AA; // LINE_AA LINE_8 LINE_4 FILLED
  let d: i32 = 2; // max_level 2: with hierarchy
  imgproc::draw_contours(dst, contours, contour_idx, col, th, lt,
    hierarchy, d, ofs)?;
  Ok(())
}

/// draw pref
pub fn draw_pref(im: &mut Mat, gci: &mut contours::GrpContoursInf, n: u64,
  mode: i32) -> Result<(), Box<dyn Error>> {
  let w_pref = ((n / 10u64.pow(mode as u32)) % 47) as i32 + 1;
  gci.grp_contours.clear();
  gci.get_grp_contours(20.0, mode * w_pref, 0, true)?; // 20.0 (640, 480)
  gci.grp_scaled_contours.clear();
  gci.whole_scaled()?;

  let wsz = im.size()?;
  let tb = &mut Mat::roi(im, core::Rect::new(0, 0, wsz.width, wsz.height))?;
  core::flip(im, tb, 0)?; // 0: tb 1: lr -1: rot180
  for si in &gci.grp_contours {
    let contours = gci.grp_scaled_contours.get_mut(si).ok_or(
      format!("no key {}", si))?;
/*
  for (&si, contours) in &mut gci.grp_scaled_contours {
//    println!("si: {}", si);
*/
    if mode == 0 {
      let flds = &gci.sci.rec[&si];
      let (pref, _city) = match shape::get_pref_city(flds[0].as_str()) {
      Err(e) => { if false { println!("{} at {}\x07", e, si) }; (0, 0) },
      Ok(r) => r
      };
      if w_pref != pref { continue; }
    }
    let hierarchy = core::Vector::<core::VecN<i32, 4>>::from_slice(&[]);
    let ofs: core::Point = core::Point_::<i32>::new(
      (-gci.offset.x * gci.scale) as i32,
      (-gci.offset.y * gci.scale) as i32);
    let col = core::VecN([96., 192., 32., 255.]); // BGRA or VecN
    draw_polys(tb, true, &cv_contours(contours)?, &hierarchy, ofs, col)?;
  }
  core::flip(im, tb, 0)?; // 0: tb 1: lr -1: rot180
  Ok(())
}
