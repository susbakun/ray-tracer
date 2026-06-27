use std::path::{Path, PathBuf};

use anyhow::{Result, anyhow};

use crate::color::Color;

#[derive(Default)]
pub struct RTWImage {
    fdata: Vec<f64>,
    width: usize,
    height: usize,
}

impl RTWImage {
    pub fn new(file_name: impl AsRef<Path>) -> Result<Self> {
        let mut image = Self::default();

        image.load(file_name)?;
        Ok(image)
    }

    pub fn load(&mut self, file_name: impl AsRef<Path>) -> Result<()> {
        let path = PathBuf::from("./images").join(file_name);
        let img = image::open(path)?;
        self.width = img.width() as usize;
        self.height = img.height() as usize;
        self.fdata = img
            .into_rgb32f()
            .to_vec()
            .into_iter()
            .map(f64::from)
            .collect();

        if self.fdata.len() == 0 {
            return Err(anyhow!("the provided image is empty"));
        }

        Ok(())
    }

    pub fn pixel_data(&self, x: usize, y: usize) -> Color {
        let x = x.clamp(0, self.width - 1);
        let y = y.clamp(0, self.height - 1);

        let index = (y * self.width + x) * 3;

        let r = self.fdata[index];
        let g = self.fdata[index + 1];
        let b = self.fdata[index + 2];

        Color::new(r, g, b)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}
