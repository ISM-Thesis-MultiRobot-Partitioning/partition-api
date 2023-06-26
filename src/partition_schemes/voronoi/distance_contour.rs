//! Same as [`super::distance`], except that the border region of the
//! [`LocationType::Assigned`] area is marked using [`edge_detection`].

use imageproc::{
    contours::{find_contours, BorderType, Contour},
    point::Point,
};
use local_robot_map::{
    Cell, CellMap, Coords, LocalMap, Location, LocationType, RealWorldLocation,
};

use crate::ps::Factors;

pub fn bydistance_contours(map: LocalMap<CellMap, Factors>, factors: Option<Factors>) -> LocalMap<CellMap, Factors> {
    super::distance::bydistance(map, factors).set_frontiers()
}

/// We shall take the liberty of interpreting the [`LocationType::Frontier`] to be
/// the frontier between the [`LocationType::Assigned`] region and everything else.
/// It allows us to neatly embed everything without introducing additional
/// types.
trait Frontiers {
    fn set_frontiers(self) -> Self;
}

impl Frontiers for LocalMap<CellMap, Factors> {
    fn set_frontiers(mut self) -> Self {
        let width: u32 = self.map().width() as u32;
        let height: u32 = self.map().height() as u32;
        let img: image::GrayImage =
            image::ImageBuffer::from_fn(width, height, |x, y| -> image::Luma<u8> {
                let (row, col) = (y as usize, x as usize);
                let cell: LocationType = self.map().cells()[[row, col]];
                match cell {
                    LocationType::Assigned => image::Luma([255]),
                    _ => image::Luma([0]),
                }
            });

        let contours: Vec<Vec<RealWorldLocation>> = find_contours(&img)
            .iter()
            .filter(|c: &&Contour<usize>| c.border_type == BorderType::Outer)
            .map(|contour| {
                contour
                    .points
                    .iter()
                    .map(|point: &Point<usize>| {
                        Cell::from_internal(
                            Coords::new(point.x as f64, point.y as f64, 0.0),
                            *self.map().offset(),
                            *self.map().resolution(),
                            &LocationType::Frontier,
                        )
                        .expect("Locations are in the map")
                        .location()
                        .clone()
                    })
                    .collect()
            })
            .collect();

        for locations in contours {
            for location in locations {
                self.map_mut()
                    .set_location(&location, LocationType::Frontier)
                    .expect("Locations are in the map");
            }
        }

        self
    }
}
