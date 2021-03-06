pub use druid::theme::*;
use druid::{Color, Env, FontDescriptor, FontFamily, Key};

pub const MOD_COLOR: Key<Color> = Key::new("druid-rustdoc.mod-color");
pub const STRUCT_COLOR: Key<Color> = Key::new("druid-rustdoc.struct-color");
pub const ENUM_COLOR: Key<Color> = Key::new("druid-rustdoc.enum-color");
pub const FN_COLOR: Key<Color> = Key::new("druid-rustdoc.function-color");
pub const KEYWORD_COLOR: Key<Color> = Key::new("druid-rustdoc.keyword-color");
pub const TRAIT_COLOR: Key<Color> = Key::new("druid-rustdoc.trait-color");
pub const CONST_COLOR: Key<Color> = Key::new("druid-rustdoc.const-color");
pub const TYPE_COLOR: Key<Color> = Key::new("druid-rustdoc.type-color");
pub const PRIMITIVE_COLOR: Key<Color> = Key::new("druid-rustdoc.primitive-color");
pub const CODE_COLOR: Key<Color> = Key::new("druid-rustdoc.code-color");
pub const ICONS_FONT: Key<FontDescriptor> = Key::new("druid-rustdoc.icons-font");
pub const CODE_FONT: Key<FontDescriptor> = Key::new("druid-rustdoc.code-font");

pub fn configure_env(env: &mut Env) {
    env.set(BACKGROUND_DARK, Color::Rgba32(0x0f1419ff));
    env.set(WINDOW_BACKGROUND_COLOR, Color::Rgba32(0x0f1419ff));
    env.set(MOD_COLOR, Color::Rgba32(0xacccf9ff));
    env.set(STRUCT_COLOR, Color::Rgba32(0xffa0a5ff));
    env.set(ENUM_COLOR, Color::Rgba32(0x99e0c9ff));
    env.set(FN_COLOR, Color::Rgba32(0xfdd687ff));
    env.set(CONST_COLOR, Color::Rgba32(0x6380a0ff));
    env.set(KEYWORD_COLOR, Color::Rgba32(0xff7733ff));
    env.set(TYPE_COLOR, Color::Rgba32(0xcfbcf5ff));
    env.set(TRAIT_COLOR, Color::Rgba32(0x39AFD7ff));
    env.set(CODE_COLOR, Color::Rgba32(0xffb454ff));
    env.set(PRIMITIVE_COLOR, Color::Rgba32(0x32889bff));
    env.set(
        ICONS_FONT,
        FontDescriptor::new(FontFamily::new_unchecked("Segoe MDL2 Assets")),
    );
    env.set(WIDGET_PADDING_VERTICAL, 10.);
    env.set(
        CODE_FONT,
        FontDescriptor::new(FontFamily::new_unchecked("Source Code Pro")).with_size(16.0),
    );
    env.set(
        UI_FONT,
        FontDescriptor::new(FontFamily::SANS_SERIF).with_size(16.0),
    )
}
