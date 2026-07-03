use crate::vector::Vec3;

// assuming 3 x 3 matrices
pub struct Matrix {
    columns: Vec<Vec3>,
}

impl Matrix {
    pub fn new(columns: Vec<Vec3>) -> Self {
        Self { columns }
    }

    pub fn determinant(&self) -> f64 {
        let t1 = self.columns[0].x()
            * ((self.columns[1].y() * self.columns[2].z())
                - (self.columns[2].y() * self.columns[1].z()));

        let t2 = self.columns[1].x()
            * ((self.columns[0].y() * self.columns[2].z())
                - (self.columns[2].y() * self.columns[0].z()));

        let t3 = self.columns[2].x()
            * ((self.columns[0].y() * self.columns[1].z())
                - (self.columns[1].y() * self.columns[0].z()));

        t1 - t2 + t3
    }
}
