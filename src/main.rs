use amethyst::{
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        rendy::hal::command::ClearColor,
        types::DefaultBackend,
        RenderingBundle,
    },
    //ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

mod state;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("assets");
    let display_config = app_root.join("config/display_config.ron");
    let key_bindings_path = app_root.join("config/input.ron");

    let mut game_data = DispatcherBuilder::default();
    game_data
        .add_bundle(TransformBundle::default())
        .add_bundle(InputBundle::new().with_bindings_from_file(&key_bindings_path)?)
        //.with_bundle(UiBundle::new())
        .add_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?.with_clear(ClearColor {
                        float32: [0.34, 0.36, 0.52, 1.0],
                    }),
                )
                .with_plugin(RenderFlat2D::default()),
            //.with_plugin(RenderUi::default())
        );

    let game = Application::new(resources, state::MyState, game_data)?;
    game.run();

    Ok(())
}
