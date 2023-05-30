use local_robot_map::Location;
use local_robot_map::{Cell, CellMap, Coords, LocalMap, LocationType, MapState, RealWorldLocation};

pub fn bydistance_frontiers(map: LocalMap<CellMap>) -> LocalMap<CellMap> {
    super::distance::bydistance(map).set_frontiers()
}

/// We shall take the liberty of interpreting the [`MapState::Frontier`] to be
/// the frontier between the [`MapState::Assigned`] region and everything else.
/// It allows us to neatly embed everything without introducing additional
/// types.
trait Frontiers {
    fn set_frontiers(self) -> Self;
}

impl Frontiers for LocalMap<CellMap> {
    fn set_frontiers(mut self) -> Self {
        let width: u32 = self.map().width() as u32;
        let height: u32 = self.map().height() as u32;
        let img = image::ImageBuffer::from_fn(width, height, |x, y| -> image::Luma<u8> {
            let (row, col) = (y as usize, x as usize);
            let cell: LocationType = self.map().cells()[[row, col]];
            match cell {
                MapState::Assigned => image::Luma([255]),
                _ => image::Luma([0]),
            }
        });

        edge_detection::canny(
            img, 1.2,  // sigma
            0.2,  // strong threshold
            0.01, // weak threshold
        )
        .as_image()
        .to_luma8()
        .rows()
        .enumerate()
        .for_each(|(row, elements)| {
            elements.enumerate().for_each(|(col, value)| {
                if value != &[0u8].into() {
                    let location: RealWorldLocation = Cell::from_internal(
                        Coords::new(col as f64, row as f64, 0.0),
                        *self.map().offset(),
                        *self.map().resolution(),
                        &LocationType::Frontier,
                    )
                    .expect("Locations are inside the map")
                    .location()
                    .clone();
                    self.map_mut()
                        .set_location(&location, LocationType::Frontier)
                        .expect("Location is inside the map")
                }
            })
        });

        self
    }
}
