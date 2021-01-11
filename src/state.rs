use amethyst::{
    assets::{DefaultLoader, Handle, Loader, LoaderBundle, ProcessingQueue},
    core::transform::Transform,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, Texture},
    // ui::{
    //     Anchor, FontHandle, LineMode, Stretch, TtfFormat, UiButtonBuilder, UiImage, UiText,
    //     UiTransform,
    // },
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
        let dimensions = resources.get::<ScreenDimensions>().unwrap();

        // Place the camera
        init_camera(world, &dimensions);

        // Load our sprites and display them
        let sprite_sheet_handle = load_sprite_sheet(&resources);
        init_sprites(world, &dimensions, &sprite_sheet_handle);

        //create_ui_example(world);
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
fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
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
fn init_sprites(
    world: &mut World,
    dimensions: &ScreenDimensions,
    sprite_sheet: &Handle<SpriteSheet>,
) {
    for i in 0..3 {
        let x = (i as f32 - 1.) * 100. + dimensions.width() * 0.5;
        let y = (i as f32 - 1.) * 100. + dimensions.height() * 0.5;
        let mut transform = Transform::default();
        transform.set_translation_xyz(x, y, 0.);
        let sprite = SpriteRender::new(sprite_sheet.clone(), i);
        world.push((transform, sprite));
    }
}

// /// Creates a simple UI background and a UI text label
// /// This is the pure code only way to create UI with amethyst.
// pub fn create_ui_example(world: &mut World) {
//     // this creates the simple gray background UI element.
//     let ui_background = world
//         .create_entity()
//         .with(UiImage::SolidColor([0.6, 0.1, 0.2, 1.0]))
//         .with(UiTransform::new(
//             "".to_string(),
//             Anchor::TopLeft,
//             Anchor::TopLeft,
//             30.0,
//             -30.,
//             0.,
//             250.,
//             50.,
//         ))
//         .build();

//     // This simply loads a font from the asset folder and puts it in the world as a resource,
//     // we also get a ref to the font that we then can pass to the text label we crate later.
//     let font: FontHandle = world.read_resource::<Loader>().load(
//         "fonts/Bangers-Regular.ttf",
//         TtfFormat,
//         (),
//         &world.read_resource(),
//     );

//     // This creates the actual label and places it on the screen.
//     // Take note of the z position given, this ensures the label gets rendered above the background UI element.
//     world
//         .create_entity()
//         .with(UiTransform::new(
//             "".to_string(),
//             Anchor::TopLeft,
//             Anchor::TopLeft,
//             40.0,
//             -40.,
//             1.,
//             200.,
//             50.,
//         ))
//         .with(UiText::new(
//             font,
//             "Hello, Amethyst UI!".to_string(),
//             [1., 1., 1., 1.],
//             30.,
//             LineMode::Single,
//             Anchor::TopLeft,
//         ))
//         .build();
// }
