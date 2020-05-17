use std::fmt;
use serde::{self, Deserialize};
use serde::de::{self, Deserializer, Visitor};

#[derive(Debug, Copy, Clone)]
pub enum Coord {
    OffZero (f64),
    OffFarEdge (f64)
}

#[derive(Debug, Deserialize)]
struct OffFarEdgeCoord {
    #[serde(rename = "off far edge")]
    off_far_edge: f64
}

struct CoordVisitor;

impl<'de> Visitor<'de> for CoordVisitor {
    type Value = Coord;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "A floating point value or a map {{from far edge: <floating point value>}}")
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where E: de::Error {
        Ok(Coord::OffZero(v))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where A: de::MapAccess<'de> {
        let ofe: OffFarEdgeCoord = Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))?;
        Ok(Coord::OffFarEdge(ofe.off_far_edge))
    }
}

impl<'de> Deserialize<'de> for Coord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        deserializer.deserialize_any(CoordVisitor { })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_number_as_zero_based_coord() {
        let c: Coord = serde_yaml::from_str("4.0")
            .expect("A simple number shoul be successfully parsed as a coordinate.");
        if let Coord::OffZero(x) = c {
            assert_eq!(x, 4.0);
        } else {
            panic!("A simple number shoul be parsed as a zero-based coordinate, but was not.")
        }
    }

    #[test]
    fn parse_map_as_far_edge_based_coord() {
        let c: Coord = serde_yaml::from_str("{\"off far edge\": 3.0}")
            .expect("A {\"off far edge\": number} map should be successfully parsed as a coordinate.");
        if let Coord::OffFarEdge(x) = c {
            assert_eq!(x, 3.0);
        } else {
            panic!("A {\"off far edge\": number} should be parsed as a far-edge-based coordinate, but was not.")
        }
    }
}