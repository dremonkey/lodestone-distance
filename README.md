# loadstone-distance

loadstone distance module
Inspired by [turf-distance](https://github.com/Turfjs/turf-distance).


### `loadstone.distance(from, to, [units=kilometers])`

Calculates the distance between two Point|points in degress, radians, miles, or 
kilometers. This uses the [Haversine formula](http://en.wikipedia.org/wiki/Haversine_formula)
to account for global curvature.


### Parameters

| parameter            | type               | description                                               |
| -------------------- | ------------------ | --------------------------------------------------------- |
| `from`               | Feature\.\<Point\> | origin point                                              |
| `to`                 | Feature\.\<Point\> | destination point                                         |
| `[units=kilometers]` | String             | _optional:_ can be degrees, radians, miles, or kilometers |


### Example

```rs
var point1 = {
  "type": "Feature",
  "properties": {},
  "geometry": {
    "type": "Point",
    "coordinates": [-75.343, 39.984]
  }
};
var point2 = {
  "type": "Feature",
  "properties": {},
  "geometry": {
    "type": "Point",
    "coordinates": [-75.534, 39.123]
  }
};
var units = "miles";

var points = {
  "type": "FeatureCollection",
  "features": [point1, point2]
};

//=points

var distance = loadstone.distance(point1, point2, units);

//=distance
```

**Returns** `f64`, distance between the two points

## Installation

```
[dependencies]
loadstone-distance = "0.1.0"
```

## Tests

```sh
$ cargo test
```


