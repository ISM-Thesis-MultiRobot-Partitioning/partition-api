use local_robot_map::{AxisResolution, CellMap, Coords, RealWorldLocation};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct CoordXYZ {
    x: f64,
    y: f64,
    z: f64,
}

impl CoordXYZ {
    pub(super) fn into_real_world(self) -> RealWorldLocation {
        RealWorldLocation::from_xyz(self.x, self.y, self.z)
    }
    pub(super) fn into_axis_resolution(self) -> AxisResolution {
        AxisResolution::new(self.x, self.y, self.z)
    }
}

impl From<&Coords> for CoordXYZ {
    fn from(value: &Coords) -> Self {
        CoordXYZ {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl From<&RealWorldLocation> for CoordXYZ {
    fn from(value: &RealWorldLocation) -> Self {
        CoordXYZ {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl From<&AxisResolution> for CoordXYZ {
    fn from(value: &AxisResolution) -> Self {
        CoordXYZ {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct InputData {
    pub(crate) vertices: Vec<CoordXYZ>,
    pub(crate) resolution: CoordXYZ,
    pub(crate) me: CoordXYZ,
    pub(crate) others: Vec<CoordXYZ>,
}

#[derive(Serialize)]
pub struct OutputData {
    cells: Vec<(CoordXYZ, &'static str)>,
    offset: CoordXYZ,
    resolution: CoordXYZ,
}

impl OutputData {
    pub(super) fn from_cellmap(map: &CellMap) -> Self {
        Self {
            cells: map
                .cells()
                .indexed_iter()
                .map(|((row, col), e)| {
                    (
                        CoordXYZ {
                            x: col as f64,
                            y: row as f64,
                            z: 0.0,
                        },
                        e.into(),
                    )
                })
                .collect(),
            offset: map.offset().into(),
            resolution: map.resolution().into(),
        }
    }

    pub(super) fn new(
        cells: Vec<(CoordXYZ, &'static str)>,
        offset: CoordXYZ,
        resolution: CoordXYZ,
    ) -> Self {
        Self {
            cells,
            offset,
            resolution,
        }
    }
}
