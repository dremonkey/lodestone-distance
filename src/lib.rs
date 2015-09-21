/// The main crate for lodestone-distance
/// 
/// ## Overview
/// 
/// Calculates the distance between two points on the surface of the earth using 
/// the [Haversine formula](http://en.wikipedia.org/wiki/Haversine_formula).

// Third party packages
extern crate lodestone_core;
extern crate lodestone_point;

use lodestone_point::FeaturePoint;
use lodestone_core::{utils, wgs84};

pub extern fn distance(
    from_point: &FeaturePoint,
    to_point: &FeaturePoint,
    units: &str) -> f64 {

  let coord1 = from_point.coordinates();
  let coord2 = to_point.coordinates();

  let lat1 = coord1[1].to_radians();
  let lat2 = coord2[1].to_radians();
  let dlat = (coord2[1] - coord1[1]).to_radians();
  let dlng = (coord2[0] - coord1[0]).to_radians();

  let a = (dlat/2.0).sin() * (dlat/2.0).sin() +
          lat1.cos() * lat2.cos() * (dlng/2.0).sin() * (dlng/2.0).sin();
  let c = 2.0 * a.sqrt().atan2((1.0-a).sqrt());

  match units {
    "degrees" => c.to_degrees(),
    "kilometers" | "km" => c * wgs84::RADIUS / 1000.0,
    "meters" | "m" => c * wgs84::RADIUS,
    "miles" | "mi" => {
      let radius_mi = utils::km_to_mi(wgs84::RADIUS / 1000.0);
      c * radius_mi
    },
    "radians" => c,
    _ => panic!("Unknown unit of measurement: {}", units)
  }
}

#[cfg(test)]
mod tests {
  use lodestone_point::FeaturePoint;
  use super::distance;

  #[test]
  #[should_panic(expected = "Unknown unit of measurement")]
  fn test_wrong_units() {
    let sf = vec![-122.4167,37.7833];
    let ny = vec![-74.0059,40.7127];

    let sf_point = FeaturePoint::new(sf);
    let ny_point = FeaturePoint::new(ny);

    distance(&sf_point, &ny_point, "leagues");
  }

  #[test]
  fn test_simple() {
    let pt1 = FeaturePoint::new(vec![0.0, 0.0]);
    let pt2 = FeaturePoint::new(vec![1.0, 0.0]);
    let d_km = distance(&pt1, &pt2, "km");

    assert_eq!(d_km, 111.31949079327356);
  }
  
  #[test]
  fn test_sf_to_ny() {
    let sf = vec![-122.4167,37.7833];
    let ny = vec![-74.0059,40.7127];

    let sf_point = FeaturePoint::new(sf);
    let ny_point = FeaturePoint::new(ny);
    
    let d_km = distance(&sf_point, &ny_point, "kilometers");
    let d_mi = distance(&sf_point, &ny_point, "miles");

    assert_eq!(d_km, 4133.177968880825);
    assert_eq!(d_mi, 2568.236927701447);
  }

  #[test]
  fn test_sf_to_la() {
    let sf = vec![-122.4167,37.7833];
    let la = vec![-118.2500,34.0500];

    let sf_point = FeaturePoint::new(sf);
    let la_point = FeaturePoint::new(la);
    let d_km = distance(&sf_point, &la_point, "kilometers");
    let d_mi = distance(&sf_point, &la_point, "miles");

    assert_eq!(d_km, 560.0666437217378);
    assert_eq!(d_mi, 348.00917047601996);
  }
}