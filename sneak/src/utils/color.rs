use bevy::prelude::*;

//An helper for creating rbg colors.
/// Relevent of the of the [`Color::srgb`]
pub fn rgb_color(r: u8, g: u8, b: u8) -> Color {
    return Color::srgb(r / 255, g / 255, b / 255);
}
