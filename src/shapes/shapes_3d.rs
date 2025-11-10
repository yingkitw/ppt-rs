//! 3D Shapes Support
//!
//! This module provides comprehensive 3D shape support including:
//! - 3D shape types (cube, sphere, cylinder, cone, pyramid, etc.)
//! - 3D properties (rotation, perspective, lighting)
//! - 3D material and surface properties
//! - 3D camera and view settings

use crate::dml::color::RGBColor;

/// 3D shape types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape3DType {
    /// Cube shape
    Cube,
    /// Sphere shape
    Sphere,
    /// Cylinder shape
    Cylinder,
    /// Cone shape
    Cone,
    /// Pyramid shape
    Pyramid,
    /// Wedge shape
    Wedge,
    /// Torus shape
    Torus,
    /// Tetrahedron shape
    Tetrahedron,
    /// Octahedron shape
    Octahedron,
    /// Icosahedron shape
    Icosahedron,
    /// Dodecahedron shape
    Dodecahedron,
}

impl Shape3DType {
    /// Get the preset name
    pub fn preset_name(&self) -> &str {
        match self {
            Shape3DType::Cube => "cube",
            Shape3DType::Sphere => "sphere",
            Shape3DType::Cylinder => "cylinder",
            Shape3DType::Cone => "cone",
            Shape3DType::Pyramid => "pyramid",
            Shape3DType::Wedge => "wedge",
            Shape3DType::Torus => "torus",
            Shape3DType::Tetrahedron => "tetrahedron",
            Shape3DType::Octahedron => "octahedron",
            Shape3DType::Icosahedron => "icosahedron",
            Shape3DType::Dodecahedron => "dodecahedron",
        }
    }

    /// Get the shape name
    pub fn name(&self) -> &str {
        match self {
            Shape3DType::Cube => "Cube",
            Shape3DType::Sphere => "Sphere",
            Shape3DType::Cylinder => "Cylinder",
            Shape3DType::Cone => "Cone",
            Shape3DType::Pyramid => "Pyramid",
            Shape3DType::Wedge => "Wedge",
            Shape3DType::Torus => "Torus",
            Shape3DType::Tetrahedron => "Tetrahedron",
            Shape3DType::Octahedron => "Octahedron",
            Shape3DType::Icosahedron => "Icosahedron",
            Shape3DType::Dodecahedron => "Dodecahedron",
        }
    }
}

/// 3D material types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Material3D {
    /// Matte material
    Matte,
    /// Plastic material
    Plastic,
    /// Metal material
    Metal,
    /// Wireframe material
    Wireframe,
}

impl Material3D {
    /// Get the material name
    pub fn name(&self) -> &str {
        match self {
            Material3D::Matte => "Matte",
            Material3D::Plastic => "Plastic",
            Material3D::Metal => "Metal",
            Material3D::Wireframe => "Wireframe",
        }
    }
}

/// 3D lighting types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lighting3D {
    /// Flat lighting
    Flat,
    /// Gouraud lighting
    Gouraud,
    /// Phong lighting
    Phong,
}

impl Lighting3D {
    /// Get the lighting name
    pub fn name(&self) -> &str {
        match self {
            Lighting3D::Flat => "Flat",
            Lighting3D::Gouraud => "Gouraud",
            Lighting3D::Phong => "Phong",
        }
    }
}

/// 3D rotation configuration
#[derive(Debug, Clone)]
pub struct Rotation3D {
    /// X-axis rotation in degrees (0-360)
    x_rotation: f32,
    /// Y-axis rotation in degrees (0-360)
    y_rotation: f32,
    /// Z-axis rotation in degrees (0-360)
    z_rotation: f32,
}

impl Default for Rotation3D {
    fn default() -> Self {
        Self::new()
    }
}

impl Rotation3D {
    /// Create a new 3D rotation
    pub fn new() -> Self {
        Self {
            x_rotation: 0.0,
            y_rotation: 0.0,
            z_rotation: 0.0,
        }
    }

    /// Set X-axis rotation
    pub fn set_x_rotation(mut self, degrees: f32) -> Self {
        self.x_rotation = degrees % 360.0;
        self
    }

    /// Get X-axis rotation
    pub fn x_rotation(&self) -> f32 {
        self.x_rotation
    }

    /// Set Y-axis rotation
    pub fn set_y_rotation(mut self, degrees: f32) -> Self {
        self.y_rotation = degrees % 360.0;
        self
    }

    /// Get Y-axis rotation
    pub fn y_rotation(&self) -> f32 {
        self.y_rotation
    }

    /// Set Z-axis rotation
    pub fn set_z_rotation(mut self, degrees: f32) -> Self {
        self.z_rotation = degrees % 360.0;
        self
    }

    /// Get Z-axis rotation
    pub fn z_rotation(&self) -> f32 {
        self.z_rotation
    }

    /// Generate XML for 3D rotation
    pub fn to_xml(&self) -> String {
        format!(
            "<a:rot x=\"{}\" y=\"{}\" z=\"{}\"/>",
            (self.x_rotation * 60000.0) as i32,
            (self.y_rotation * 60000.0) as i32,
            (self.z_rotation * 60000.0) as i32
        )
    }
}

/// 3D camera configuration
#[derive(Debug, Clone)]
pub struct Camera3D {
    /// Camera preset (orthographic, perspective, etc.)
    preset: String,
    /// Field of view in degrees
    fov: f32,
    /// Zoom level
    zoom: f32,
}

impl Default for Camera3D {
    fn default() -> Self {
        Self::new()
    }
}

impl Camera3D {
    /// Create a new 3D camera
    pub fn new() -> Self {
        Self {
            preset: "Perspective".to_string(),
            fov: 60.0,
            zoom: 1.0,
        }
    }

    /// Set camera preset
    pub fn set_preset(mut self, preset: impl Into<String>) -> Self {
        self.preset = preset.into();
        self
    }

    /// Get camera preset
    pub fn preset(&self) -> &str {
        &self.preset
    }

    /// Set field of view
    pub fn set_fov(mut self, fov: f32) -> Self {
        self.fov = fov.max(1.0).min(179.0);
        self
    }

    /// Get field of view
    pub fn fov(&self) -> f32 {
        self.fov
    }

    /// Set zoom level
    pub fn set_zoom(mut self, zoom: f32) -> Self {
        self.zoom = zoom.max(0.1);
        self
    }

    /// Get zoom level
    pub fn zoom(&self) -> f32 {
        self.zoom
    }

    /// Generate XML for 3D camera
    pub fn to_xml(&self) -> String {
        format!(
            "<a:camera preset=\"{}\" fov=\"{}\" zoom=\"{}\"/>",
            self.preset,
            (self.fov * 60000.0) as i32,
            (self.zoom * 100000.0) as i32
        )
    }
}

/// 3D light configuration
#[derive(Debug, Clone)]
pub struct Light3D {
    /// Light type (key, fill, back)
    light_type: String,
    /// Light color
    color: RGBColor,
    /// Light intensity (0.0-1.0)
    intensity: f32,
    /// Light direction (x, y, z)
    direction: (f32, f32, f32),
}

impl Light3D {
    /// Create a new 3D light
    pub fn new(light_type: impl Into<String>) -> Self {
        Self {
            light_type: light_type.into(),
            color: RGBColor::new(255, 255, 255),
            intensity: 1.0,
            direction: (0.0, 0.0, 1.0),
        }
    }

    /// Get light type
    pub fn light_type(&self) -> &str {
        &self.light_type
    }

    /// Set light color
    pub fn set_color(mut self, color: RGBColor) -> Self {
        self.color = color;
        self
    }

    /// Get light color
    pub fn color(&self) -> &RGBColor {
        &self.color
    }

    /// Set light intensity
    pub fn set_intensity(mut self, intensity: f32) -> Self {
        self.intensity = intensity.max(0.0).min(1.0);
        self
    }

    /// Get light intensity
    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    /// Set light direction
    pub fn set_direction(mut self, x: f32, y: f32, z: f32) -> Self {
        self.direction = (x, y, z);
        self
    }

    /// Get light direction
    pub fn direction(&self) -> (f32, f32, f32) {
        self.direction
    }

    /// Generate XML for 3D light
    pub fn to_xml(&self) -> String {
        format!(
            "<a:light type=\"{}\" color=\"{}\" intensity=\"{}\"/>",
            self.light_type,
            self.color.to_hex(),
            self.intensity
        )
    }
}

/// 3D shape configuration
#[derive(Debug, Clone)]
pub struct Shape3D {
    /// Shape type
    shape_type: Shape3DType,
    /// 3D rotation
    rotation: Rotation3D,
    /// 3D camera
    camera: Camera3D,
    /// 3D material
    material: Material3D,
    /// 3D lighting
    lighting: Lighting3D,
    /// Ambient color
    ambient_color: RGBColor,
    /// Specular color
    specular_color: RGBColor,
}

impl Shape3D {
    /// Create a new 3D shape
    pub fn new(shape_type: Shape3DType) -> Self {
        Self {
            shape_type,
            rotation: Rotation3D::new(),
            camera: Camera3D::new(),
            material: Material3D::Plastic,
            lighting: Lighting3D::Phong,
            ambient_color: RGBColor::new(128, 128, 128),
            specular_color: RGBColor::new(255, 255, 255),
        }
    }

    /// Get shape type
    pub fn shape_type(&self) -> Shape3DType {
        self.shape_type
    }

    /// Get rotation
    pub fn rotation(&self) -> &Rotation3D {
        &self.rotation
    }

    /// Set rotation
    pub fn set_rotation(mut self, rotation: Rotation3D) -> Self {
        self.rotation = rotation;
        self
    }

    /// Get camera
    pub fn camera(&self) -> &Camera3D {
        &self.camera
    }

    /// Set camera
    pub fn set_camera(mut self, camera: Camera3D) -> Self {
        self.camera = camera;
        self
    }

    /// Set material
    pub fn set_material(mut self, material: Material3D) -> Self {
        self.material = material;
        self
    }

    /// Get material
    pub fn material(&self) -> Material3D {
        self.material
    }

    /// Set lighting
    pub fn set_lighting(mut self, lighting: Lighting3D) -> Self {
        self.lighting = lighting;
        self
    }

    /// Get lighting
    pub fn lighting(&self) -> Lighting3D {
        self.lighting
    }

    /// Set ambient color
    pub fn set_ambient_color(mut self, color: RGBColor) -> Self {
        self.ambient_color = color;
        self
    }

    /// Get ambient color
    pub fn ambient_color(&self) -> &RGBColor {
        &self.ambient_color
    }

    /// Set specular color
    pub fn set_specular_color(mut self, color: RGBColor) -> Self {
        self.specular_color = color;
        self
    }

    /// Get specular color
    pub fn specular_color(&self) -> &RGBColor {
        &self.specular_color
    }

    /// Generate XML for 3D shape
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<p:sp3d>");
        xml.push_str(&format!(
            "<p:type preset=\"{}\"/>",
            self.shape_type.preset_name()
        ));
        xml.push_str(&self.rotation.to_xml());
        xml.push_str(&self.camera.to_xml());
        xml.push_str(&format!(
            "<p:material type=\"{}\"/>",
            self.material.name()
        ));
        xml.push_str(&format!(
            "<p:lighting type=\"{}\"/>",
            self.lighting.name()
        ));
        xml.push_str("</p:sp3d>");
        xml
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_3d_type_cube() {
        let shape_type = Shape3DType::Cube;
        assert_eq!(shape_type.preset_name(), "cube");
        assert_eq!(shape_type.name(), "Cube");
    }

    #[test]
    fn test_shape_3d_type_sphere() {
        let shape_type = Shape3DType::Sphere;
        assert_eq!(shape_type.preset_name(), "sphere");
        assert_eq!(shape_type.name(), "Sphere");
    }

    #[test]
    fn test_material_3d_matte() {
        let material = Material3D::Matte;
        assert_eq!(material.name(), "Matte");
    }

    #[test]
    fn test_material_3d_metal() {
        let material = Material3D::Metal;
        assert_eq!(material.name(), "Metal");
    }

    #[test]
    fn test_lighting_3d_phong() {
        let lighting = Lighting3D::Phong;
        assert_eq!(lighting.name(), "Phong");
    }

    #[test]
    fn test_rotation_3d_new() {
        let rotation = Rotation3D::new();
        assert_eq!(rotation.x_rotation(), 0.0);
        assert_eq!(rotation.y_rotation(), 0.0);
        assert_eq!(rotation.z_rotation(), 0.0);
    }

    #[test]
    fn test_rotation_3d_set_x() {
        let rotation = Rotation3D::new().set_x_rotation(45.0);
        assert_eq!(rotation.x_rotation(), 45.0);
    }

    #[test]
    fn test_rotation_3d_set_y() {
        let rotation = Rotation3D::new().set_y_rotation(90.0);
        assert_eq!(rotation.y_rotation(), 90.0);
    }

    #[test]
    fn test_rotation_3d_set_z() {
        let rotation = Rotation3D::new().set_z_rotation(180.0);
        assert_eq!(rotation.z_rotation(), 180.0);
    }

    #[test]
    fn test_rotation_3d_to_xml() {
        let rotation = Rotation3D::new().set_x_rotation(45.0);
        let xml = rotation.to_xml();
        assert!(xml.contains("<a:rot"));
        assert!(xml.contains("/>"));
    }

    #[test]
    fn test_camera_3d_new() {
        let camera = Camera3D::new();
        assert_eq!(camera.preset(), "Perspective");
        assert_eq!(camera.fov(), 60.0);
        assert_eq!(camera.zoom(), 1.0);
    }

    #[test]
    fn test_camera_3d_set_fov() {
        let camera = Camera3D::new().set_fov(45.0);
        assert_eq!(camera.fov(), 45.0);
    }

    #[test]
    fn test_camera_3d_set_zoom() {
        let camera = Camera3D::new().set_zoom(2.0);
        assert_eq!(camera.zoom(), 2.0);
    }

    #[test]
    fn test_camera_3d_to_xml() {
        let camera = Camera3D::new();
        let xml = camera.to_xml();
        assert!(xml.contains("<a:camera"));
        assert!(xml.contains("/>"));
    }

    #[test]
    fn test_light_3d_new() {
        let light = Light3D::new("Key");
        assert_eq!(light.light_type(), "Key");
        assert_eq!(light.intensity(), 1.0);
    }

    #[test]
    fn test_light_3d_set_color() {
        let light = Light3D::new("Fill")
            .set_color(RGBColor::new(255, 0, 0));
        assert_eq!(light.color(), &RGBColor::new(255, 0, 0));
    }

    #[test]
    fn test_light_3d_set_intensity() {
        let light = Light3D::new("Back").set_intensity(0.5);
        assert_eq!(light.intensity(), 0.5);
    }

    #[test]
    fn test_light_3d_to_xml() {
        let light = Light3D::new("Key");
        let xml = light.to_xml();
        assert!(xml.contains("<a:light"));
        assert!(xml.contains("/>"));
    }

    #[test]
    fn test_shape_3d_new() {
        let shape = Shape3D::new(Shape3DType::Cube);
        assert_eq!(shape.shape_type(), Shape3DType::Cube);
        assert_eq!(shape.material(), Material3D::Plastic);
        assert_eq!(shape.lighting(), Lighting3D::Phong);
    }

    #[test]
    fn test_shape_3d_set_material() {
        let shape = Shape3D::new(Shape3DType::Sphere)
            .set_material(Material3D::Metal);
        assert_eq!(shape.material(), Material3D::Metal);
    }

    #[test]
    fn test_shape_3d_set_lighting() {
        let shape = Shape3D::new(Shape3DType::Cylinder)
            .set_lighting(Lighting3D::Flat);
        assert_eq!(shape.lighting(), Lighting3D::Flat);
    }

    #[test]
    fn test_shape_3d_set_colors() {
        let shape = Shape3D::new(Shape3DType::Cone)
            .set_ambient_color(RGBColor::new(100, 100, 100))
            .set_specular_color(RGBColor::new(200, 200, 200));
        assert_eq!(shape.ambient_color(), &RGBColor::new(100, 100, 100));
        assert_eq!(shape.specular_color(), &RGBColor::new(200, 200, 200));
    }

    #[test]
    fn test_shape_3d_to_xml() {
        let shape = Shape3D::new(Shape3DType::Pyramid);
        let xml = shape.to_xml();
        assert!(xml.contains("<p:sp3d>"));
        assert!(xml.contains("</p:sp3d>"));
        assert!(xml.contains("pyramid"));
    }
}
