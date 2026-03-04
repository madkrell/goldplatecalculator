use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Shape {
    Rectangle {
        length: f64,
        width: f64,
        height: f64,
    },
    Cylinder {
        radius: f64,
        height: f64,
    },
    Sphere {
        radius: f64,
    },
}

impl Shape {
    pub fn surface_area(&self) -> f64 {
        match self {
            Shape::Rectangle {
                length,
                width,
                height,
            } => 2.0 * (length * width + length * height + width * height),
            Shape::Cylinder { radius, height } => {
                2.0 * PI * radius * radius + 2.0 * PI * radius * height
            }
            Shape::Sphere { radius } => 4.0 * PI * radius * radius,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Shape::Rectangle { .. } => "Rectangle",
            Shape::Cylinder { .. } => "Cylinder",
            Shape::Sphere { .. } => "Sphere",
        }
    }
}

#[derive(Clone, Debug)]
pub struct ObjectEntry {
    pub id: usize,
    pub name: String,
    pub shape: Shape,
}

/// A saved object preset that can be re-used across sessions.
/// Stored by name and shape so users can quickly re-add common items.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SavedObject {
    pub name: String,
    pub shape: Shape,
}

impl ObjectEntry {
    pub fn surface_area(&self) -> f64 {
        self.shape.surface_area()
    }
}

pub fn target_amperage(total_surface_area_cm2: f64) -> f64 {
    0.008 * total_surface_area_cm2
}
