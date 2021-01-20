//! Defines the Light data structure

use crate::{AiString, Color3D, Vector3D};
use libc::c_float;

/// Enumerates all supported types of light sources.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]

pub enum LightType {
    /// An undefined light, not a valid value
    // TODO handle this in a rust way?
    Undefined = 0x0,

    /// A directional light source has a well-defined direction.
    ///
    /// A directional light source has a well-defined direction
    /// but is infinitely far away. That's quite a good
    /// approximation for sun light.
    Directional = 0x1,

    /// A point light emits light in all directions.
    ///
    /// A point light source has a well-defined position
    /// in space but no direction - it emits light in all
    /// directions. A normal bulb is a point light.
    Point = 0x2,

    /// A spot light source emits light in a specific angle.
    ///
    /// It has a position and a direction it is pointing to.
    /// A good example for a spot light is a light spot in
    /// sport arenas.
    Spot = 0x3,
}

/// Helper structure to describe a light source.
///
/// Assimp supports multiple sorts of light sources, including
/// directional, point and spot lights. All of them are defined with just
/// a single structure and distinguished by their parameters.
///
/// Note - some file formats (such as 3DS, ASE) export a "target point" -
/// the point a spot light is looking at (it can even be animated). Assimp
/// writes the target point as a subnode of a spotlights's main node,
/// called "<spotName>.Target". However, this is just additional information
/// then, the transformation tracks of the main node make the
/// spot light already point in the right direction.
#[derive(Copy, Clone, PartialEq, Debug)]

pub struct Light {
    /// The name of the light source.
    ///
    /// There must be a node in the scenegraph with the same name.
    /// This node specifies the position of the light in the scene
    /// hierarchy and can be animated.
    pub name: AiString,

    /// The type of the light source.
    ///
    /// `LightType::Undefined` is not a valid value for this member.
    pub light_type: LightType,

    /// Position of the light source in space. Relative to the
    /// transformation of the node corresponding to the light.
    ///
    /// The position is undefined for directional lights.
    pub position: Vector3D,

    /// Direction of the light source in space. Relative to the
    /// transformation of the node corresponding to the light.
    ///
    /// The direction is undefined for point lights. The vector
    /// may be normalized, but it needn't.
    pub direction: Vector3D,

    /// Constant light attenuation factor.
    ///
    /// The intensity of the light source at a given distance 'd' from
    /// the light's position is
    /// ```math
    /// Atten = 1/( att0 + att1 * d + att2 * d*d)
    /// ```
    /// This member corresponds to the att0 variable in the equation.
    /// Naturally undefined for directional lights.
    pub attenuation_constant: c_float,

    /// Linear light attenuation factor.
    ///
    /// The intensity of the light source at a given distance 'd' from
    /// the light's position is
    /// ```
    /// Atten = 1/( att0 + att1 * d + att2 * d*d)
    /// ```
    /// This member corresponds to the att1 variable in the equation.
    /// Naturally undefined for directional lights.
    pub attenuation_linear: c_float,

    /// Quadratic light attenuation factor.
    ///
    /// The intensity of the light source at a given distance 'd' from
    /// the light's position is
    /// ```
    /// Atten = 1/( att0 + att1 * d + att2 * d*d)
    /// ```
    /// This member corresponds to the att2 variable in the equation.
    /// Naturally undefined for directional lights.
    pub attenuation_quadratic: c_float,

    /// Diffuse color of the light source
    ///
    /// The diffuse light color is multiplied with the diffuse
    /// material color to obtain the final color that contributes
    /// to the diffuse shading term.
    pub color_diffuse: Color3D,

    /// Specular color of the light source
    ///
    /// The specular light color is multiplied with the specular
    /// material color to obtain the final color that contributes
    /// to the specular shading term.
    pub color_specular: Color3D,

    /// Ambient color of the light source
    ///
    /// The ambient light color is multiplied with the ambient
    /// material color to obtain the final color that contributes
    /// to the ambient shading term. Most renderers will ignore
    /// this value it, is just a remaining of the fixed-function pipeline
    /// that is still supported by quite many file formats.
    pub color_ambient: Color3D,

    /// Inner angle of a spot light's light cone.
    ///
    /// The spot light has maximum influence on objects inside this
    /// angle. The angle is given in radians. It is 2PI for point
    /// lights and undefined for directional lights.
    pub angle_inner_cone: c_float,

    /// Outer angle of a spot light's light cone.
    ///
    /// The spot light does not affect objects outside this angle.
    /// The angle is given in radians. It is 2PI for point lights and
    /// undefined for directional lights. The outer angle must be
    /// greater than or equal to the inner angle.
    /// It is assumed that the application uses a smooth
    /// interpolation between the inner and the outer cone of the spot light.
    pub angle_outer_cone: c_float,
}

// vim: et tw=78 sw=4:
