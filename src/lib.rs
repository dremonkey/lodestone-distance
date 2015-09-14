/// The main create for lodestone-distance
/// 
/// ## Overview
/// 
/// Calculates the distance between two points on the surface of the earth using 
/// the [Haversine formula](http://en.wikipedia.org/wiki/Haversine_formula).

// Standard lib packages
use std::f64::consts::{PI};

// Third party packages
extern crate lodestone_point;

use lodestone_point::FeaturePoint;

// Radius of the Earth
// http://www.wolframalpha.com/input/?i=average+radius+of+Earth
const RADIUS_KM: f64 = 6367.4447;
const RADIUS_MI: f64 = 3956.5467;
const RADIUS_DEGREES: f64 = 57.2957795;

pub extern fn distance(
    from_point: &FeaturePoint,
    to_point: &FeaturePoint,
    units: &str) -> f64 {

  let coord1 = from_point.coordinates();
  let coord2 = to_point.coordinates();

  let lat1 = to_rad(coord1[1]);
  let lat2 = to_rad(coord2[1]);
  let dlat = to_rad(coord2[1] - coord1[1]);
  let dlng = to_rad(coord2[0] - coord1[0]);

  let a = (dlat/2.0).sin() * (dlat/2.0).sin() +
          lat1.cos() * lat2.cos() * (dlng/2.0).sin() * (dlng/2.0).sin();
  let c = 2.0 * a.sqrt().atan2((1.0-a).sqrt());

  match units {
    "degrees" => RADIUS_DEGREES * c,
    "kilometers" => RADIUS_KM * c,
    "miles" => RADIUS_MI * c,
    "radians" => c,
    _ => RADIUS_KM * c,
  }
}

fn to_rad(degree: f64) -> f64 {
  return degree * PI / 180.0;
}

#[cfg(test)]
mod tests {
  use lodestone_point::FeaturePoint;
  use super::distance;
  
  #[test]
  fn test_sf_to_ny() {
    let sf = vec![-122.4167,37.7833];
    let ny = vec![-74.0059,40.7127];

    let sf_point = FeaturePoint::new(sf);
    let ny_point = FeaturePoint::new(ny);
    
    let d_km = distance(&sf_point, &ny_point, "kilometers");
    let d_mi = distance(&sf_point, &ny_point, "miles");

    assert_eq!(d_km, 4126.249115079681);
    assert_eq!(d_mi, 2563.9323290308953);
  }

  #[test]
  fn test_sf_to_la() {
    let sf = vec![-122.4167,37.7833];
    let la = vec![-118.2500,34.0500];

    let sf_point = FeaturePoint::new(sf);
    let la_point = FeaturePoint::new(la);
    let d_km = distance(&sf_point, &la_point, "kilometers");
    let d_mi = distance(&sf_point, &la_point, "miles");

    assert_eq!(d_km, 559.1277487787996);
    assert_eq!(d_mi, 347.4258754550611);
  }
}