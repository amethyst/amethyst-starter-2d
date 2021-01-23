use amethyst::{
    assets::{DefaultLoader, Handle, Loader, ProcessingQueue},
    core::transform::Transform,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, SpriteRender, SpriteSheet},
    ui::{Anchor, LineMode, UiImage, UiLabelBuilder, UiTransform},
    window::ScreenDimensions,
};

use log::info;

/// A dummy game state that shows 3 sprites.
pub struct MyState;

impl SimpleState for MyState {
    // Here, we define hooks that will be called throughout the lifecycle of our game state.
    //
    // In this example, `on_start` is used for initializing entities
    // and `handle_state` for managing the state transitions.
    //
    // For more state lifecycle hooks, see:
    // https://book.amethyst.rs/stable/concepts/state.html#life-cycle

    /// The state is initialized with:
    /// - a camera centered in the middle of the screen.
    /// - 3 sprites places around the center.
    fn on_start(&mut self, data: StateData<'_, GameData>) {
        let StateData {
            world, resources, ..
        } = data;

        // Get the screen dimensions so we can initialize the camera and
        // place our sprites correctly later. We'll clone this since we'll
        // pass the world mutably to the following functions.
        //let dimensions = resources.get::<ScreenDimensions>().unwrap();

        // Place the camera
        init_camera(world, resources);

        // Load our sprites and display them
        let sprite_sheet_handle = load_sprite_sheet(&resources);
        init_sprites(world, resources, &sprite_sheet_handle);

        create_ui_example(world, resources);
    }

    /// The following events are handled:
    /// - The game state is quit when either the close button is clicked or when the escape key is pressed.
    /// - Any other keypress is simply logged to the console.
    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            // Listen to any key events
            if let Some(event) = get_key(&event) {
                info!("handling key event: {:?}", event);
            }

            // If you're looking for a more sophisticated event handling solution,
            // including key bindings and gamepad support, please have a look at
            // https://book.amethyst.rs/stable/pong-tutorial/pong-tutorial-03.html#capturing-user-input
        }

        // Keep going
        Trans::None
    }
}

/// Creates a camera entity in the `world`.
///
/// The `dimensions` are used to center the camera in the middle
/// of the screen, as well as make it cover the entire screen.
fn init_camera(world: &mut World, resources: &mut Resources) {
    let dimensions = resources.get::<ScreenDimensions>().unwrap();
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.);

    world.push((
        Camera::standard_2d(dimensions.width(), dimensions.height()),
        transform,
    ));
}

fn load_sprite_sheet(resources: &Resources) -> Handle<SpriteSheet> {
    let loader = resources.get::<DefaultLoader>().unwrap();

    let texture = loader.load("sprites/logo.png");
    let sprites = loader.load("sprites/logo.ron");

    let sprite_sheet_store = resources.get::<ProcessingQueue<SpriteSheet>>().unwrap();
    loader.load_from_data(SpriteSheet { texture, sprites }, (), &sprite_sheet_store)
}

/// Creates an entity in the `world` for each of the provided `sprites`.
/// They are individually placed around the center of the screen.
fn init_sprites(world: &mut World, resources: &mut Resources, sprite_sheet: &Handle<SpriteSheet>) {
    let dimensions = resources.get::<ScreenDimensions>().unwrap();
    for i in 0..3 {
        let x = (i as f32 - 1.) * 100. + dimensions.width() * 0.5;
        let y = (i as f32 - 1.) * 100. + dimensions.height() * 0.5;
        let mut transform = Transform::default();
        transform.set_translation_xyz(x, y, 0.);
        let sprite = SpriteRender::new(sprite_sheet.clone(), i);
        world.push((transform, sprite));
    }
}

/// Creates a simple UI background and a UI text label
/// This is the pure code only way to create UI with amethyst.
pub fn create_ui_example(world: &mut World, resources: &mut Resources) {
    // background
    let image = UiImage::SolidColor([0.6, 0.1, 0.2, 1.0]);
    let transform = UiTransform::new(
        "".to_string(),
        Anchor::TopLeft,
        Anchor::TopLeft,
        30.0,
        -30.,
        0.,
        250.,
        50.,
    );
    world.push((image, transform));

    // font
    let font = {
        resources
            .get::<DefaultLoader>()
            .unwrap()
            .load("fonts/Bangers-Regular.ttf")
    };

    // label
    let (_, label) = UiLabelBuilder::<(), u32>::new("Hello, Amethyst UI!".to_string())
        // general
        .with_size(200., 50.)
        .with_position(145., -65.)
        .with_layer(1.)
        .with_anchor(Anchor::TopLeft)
        .with_line_mode(LineMode::Single)
        // font
        .with_font(font)
        .with_font_size(30.)
        .with_align(Anchor::TopLeft)
        .with_text_color([1., 1., 1., 1.])
        // background
        .build_from_world_and_resources(world, resources);

    world.entry(label.text_entity).unwrap();
}
