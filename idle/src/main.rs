// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

use druid::{AppLauncher, WindowDesc, Widget, PlatformError};
use druid::widget::{Button, Label, Flex, Padding, Align};
use engine::MageCity;

mod engine;

fn build_ui() -> impl Widget<()> {
    Padding::new(
        10.0,
        Flex::row()
            .with_flex_child(
                Flex::column()
                    .with_flex_child(Label::new("top left"), 1.0)
                    .with_flex_child(Align::centered(Label::new("bottom left")), 1.0),
                1.0)
            .with_flex_child(
                Flex::column()
                    .with_flex_child(Button::new("Add 1 money")
                    .on_click(|_ctx,data: &mut MageCity, _: _env| {
                        data.add_value_money(1);
                    })
                    , 1.0)
                    .with_flex_child(Align::centered(Label::new("bottom left")), 1.0),
                1.0))
}

fn main() -> Result<(), PlatformError> {
    let mage_city = engine::MageCity{
        nbr_of_plp: 0.0,
        money: 0.0,
        mana: 0.0
    };
    let window = WindowDesc::new(build_ui)
    .window_size((1000.0, 500.0))
    .resizable(false)
    .title("Idle Rust Game");
    AppLauncher::with_window(window).launch(())?;
    Ok(())
}