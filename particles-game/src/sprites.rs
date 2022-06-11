use bevy::prelude::Color;

pub struct Sprites<'a> {
    pub foundation: Color,
    pub stone: Color,
    pub sand: &'a [Color],
    pub none: Color,
}

pub const SPRITES: Sprites = Sprites {
    foundation: Color::Rgba {
        red: 0.15,
        green: 0.15,
        blue: 0.15,
        alpha: 1.0,
    },
    stone: Color::Rgba {
        red: 0.,
        green: 0.,
        blue: 0.7,
        alpha: 1.0,
    },
    sand: &[
        Color::Rgba {
            red: 0.5,
            green: 0.,
            blue: 0.,
            alpha: 1.0,
        },
        Color::Rgba {
            red: 0.,
            green: 0.5,
            blue: 0.,
            alpha: 1.0,
        },
    ],
    none: Color::Rgba {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
    },
};
