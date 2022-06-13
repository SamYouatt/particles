use bevy::prelude::Color;
use rand::seq::SliceRandom;

pub enum SpriteType<'a> {
    Single(Color),
    Range(&'a [Color]),
}

// pub struct Sprites<'a> {
//     pub foundation: Color,
//     pub stone: Color,
//     pub sand: &'a [Color],
//     pub none: Color,
// }

pub struct Sprites<'a> {
    pub foundation: SpriteType<'a>,
    pub stone: SpriteType<'a>,
    pub sand: SpriteType<'a>,
    pub none: SpriteType<'a>,
}

pub const SPRITES: Sprites = Sprites {
    foundation: SpriteType::Single(Color::Rgba {
        red: 0.15,
        green: 0.15,
        blue: 0.15,
        alpha: 1.0,
    }),
    stone: SpriteType::Single(Color::Rgba {
        red: 0.,
        green: 0.,
        blue: 0.7,
        alpha: 1.0,
    }),
    sand: SpriteType::Range(&[
        Color::Rgba {
            red: 240. / 255.,
            green: 214. / 255.,
            blue: 120. / 255.,
            alpha: 1.0,
        },
        Color::Rgba {
            red: 227. / 255.,
            green: 200. / 255.,
            blue: 102. / 255.,
            alpha: 1.0,
        },
    ]),
    none: SpriteType::Single(Color::Rgba {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
    }),
};

pub fn get_sprite_color(sprite: SpriteType) -> Color {
    match sprite {
        SpriteType::Single(color) => color,
        SpriteType::Range(options) => *options
            .choose_multiple(&mut rand::thread_rng(), 1)
            .next()
            .unwrap(),
    }
}
