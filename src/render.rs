use amethyst::{
    ecs::prelude::{ReadExpect, Resources, SystemData},
    renderer::{
        pass::DrawFlat2DTransparentDesc,
        rendy::{
            factory::Factory,
            graph::{
                render::{RenderGroupDesc, SubpassBuilder},
                GraphBuilder,
            },
            hal::format::Format,
        },
        types::DefaultBackend,
        GraphCreator, Kind,
    },
    window::{ScreenDimensions, Window},
};

// Window background color
static CLEAR_COLOR: [f32; 4] = [0.34, 0.36, 0.52, 1.0];

#[derive(Default)]
pub struct RenderGraph {
    dimensions: Option<ScreenDimensions>,
    dirty: bool,
}

impl GraphCreator<DefaultBackend> for RenderGraph {
    fn rebuild(&mut self, res: &Resources) -> bool {
        use std::ops::Deref;

        // Only rebuild when dimensions have changed
        let new_dimensions = res.try_fetch::<ScreenDimensions>();
        let new_dimensions = new_dimensions.as_ref().map(|d| d.deref());

        if self.dimensions.as_ref() != new_dimensions {
            self.dirty = true;
            self.dimensions = new_dimensions.map(|d| d.clone());
            return false;
        }

        self.dirty
    }

    fn builder(
        &mut self,
        factory: &mut Factory<DefaultBackend>,
        res: &Resources,
    ) -> GraphBuilder<DefaultBackend, Resources> {
        use amethyst::renderer::rendy::{
            graph::present::PresentNode,
            hal::command::{ClearDepthStencil, ClearValue},
        };

        // Since we're freshly building the graph, it will never
        // be dirty after this function is called.
        self.dirty = false;

        let window = <ReadExpect<'_, Window>>::fetch(res);

        let surface = factory.create_surface(&window);
        let surface_format = factory.get_surface_format(&surface);

        let dimensions = self.dimensions.as_ref().unwrap();
        let window_kind = Kind::D2(dimensions.width() as u32, dimensions.height() as u32, 1, 1);

        let clear_color = ClearValue::Color(CLEAR_COLOR.into());
        let clear_depth = ClearValue::DepthStencil(ClearDepthStencil(1.0, 0));

        // Build the RenderGraph
        let mut builder = GraphBuilder::new();
        let color = builder.create_image(window_kind, 1, surface_format, Some(clear_color));
        let depth = builder.create_image(window_kind, 1, Format::D32Sfloat, Some(clear_depth));

        // Add additional draw groups here for things like UI
        let pass = builder.add_node(
            SubpassBuilder::new()
                // Draw sprites with transparency
                .with_group(DrawFlat2DTransparentDesc::new().builder())
                .with_color(color)
                .with_depth_stencil(depth)
                .into_pass(),
        );

        // Render the result to the surface
        let present = PresentNode::builder(factory, surface, color).with_dependency(pass);
        builder.add_node(present);

        builder
    }
}
