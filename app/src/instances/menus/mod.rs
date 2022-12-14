use super::*;
use gridbugs::chargrid::border::BorderStyle;

mod main_menu;
mod options;
mod paused;
mod prologue;
mod prompt;
mod upgrade;

pub use main_menu::*;
pub use options::*;
pub use paused::*;
pub use prologue::*;
pub use prompt::*;
pub use upgrade::*;

fn _menu_style<T: 'static>(menu: AppCF<T>) -> AppCF<T> {
    menu.border(BorderStyle::default()).fill(color::MENU_BACKGROUND).centre().overlay_tint(
        render_state(|state: &State, ctx, fb| state.render(color::CURSOR, ctx, fb)),
        gridbugs::chargrid::core::TintDim(63),
        10,
    )
}

pub fn menu_style<T: 'static>(menu: AppCF<T>) -> AppCF<T> {
    menu.border(BorderStyle::default()).fill(Rgba32::new_grey(0)).centre().overlay_tint(
        render_state(|state: &State, ctx, fb| state.render(color::CURSOR, ctx, fb)),
        gridbugs::chargrid::core::TintDim(63),
        10,
    )
}

pub fn popup_style<T: 'static>(menu: AppCF<T>) -> AppCF<T> {
    menu.border(BorderStyle::default()).fill(Rgba32::new_grey(0)).centre().add_y(0).overlay_tint(
        render_state(|state: &State, ctx, fb| state.render(color::CURSOR, ctx, fb)),
        gridbugs::chargrid::core::TintDim(255),
        10,
    )
}
