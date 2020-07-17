mod images;
mod render_targets;
mod graph;
mod graph_builder;
mod pass;
mod pass_builder;
mod execute_context;
mod executor;
mod schedule;
mod schedule_builder;

pub use images::*;
pub use render_targets::*;
pub use graph::*;
pub use graph_builder::*;
pub use pass::*;
pub use pass_builder::*;
pub use execute_context::*;
pub use executor::*;
pub use schedule::*;
pub use schedule_builder::*;

#[cfg(test)]
mod tests {
    use crate::graphics::{
        *,
        rendergraph::*,
    };

    #[test]
    fn it_works_when_doing_nothing() {
        let mut builder = GraphBuilder::new();

        let back_buffer = builder.import_image("Back Buffer", ImageDescription::new(
            1920,
            1080,
            ImageFormat::R8G8B8A8_SRGB,
            ImageType::Type2D,
            SampleCount::Type_1,
            ImageLayout::Unknown,
        )).unwrap();

        let graph = builder.build(&[back_buffer]);

        let schedule = graph.compile_schedule();

        dbg!(schedule);
        // panic!("{}", "");
    }

    #[test]
    fn it_works() {
        let start = std::time::Instant::now();
        let mut builder = GraphBuilder::new();

        // define all images used to render the frame
        let back_buffer = builder.import_image("Back Buffer", ImageDescription::new(
            1920,
            1080,
            ImageFormat::R8G8B8A8_SRGB,
            ImageType::Type2D,
            SampleCount::Type_1,
            ImageLayout::Unknown,
        )).unwrap();

        let hdr_buffer = builder.create_image("HDR Buffer", ImageDescription::new(
            1920,
            1080,
            ImageFormat::R8G8B8A8_SRGB,
            ImageType::Type2D,
            SampleCount::Type_1,
            ImageLayout::Unknown,
        )).unwrap();

        let normals_buffer = builder.create_image("Normals Buffer", ImageDescription::new(
            1920,
            1080,
            ImageFormat::R8G8B8A8_SRGB,
            ImageType::Type2D,
            SampleCount::Type_1,
            ImageLayout::Unknown,
        )).unwrap();

        let specular_buffer = builder.create_image("Specular Buffer", ImageDescription::new(
            1920,
            1080,
            ImageFormat::R8G8B8A8_SRGB,
            ImageType::Type2D,
            SampleCount::Type_1,
            ImageLayout::Unknown,
        )).unwrap();

        let depth_buffer = builder.create_image("Depth Buffer", ImageDescription::new(
            1920,
            1080,
            ImageFormat::D32_SFLOAT,
            ImageType::Type2D,
            SampleCount::Type_1,
            ImageLayout::Unknown,
        )).unwrap();

        let velocity_buffer = builder.create_image("Velocity Buffer", ImageDescription::new(
            1920,
            1080,
            ImageFormat::R32G32_SFLOAT,
            ImageType::Type2D,
            SampleCount::Type_1,
            ImageLayout::Unknown,
        )).unwrap();

        let shadow_map = builder.create_image("Shadow Map", ImageDescription::new(
            1024,
            1024,
            ImageFormat::D32_SFLOAT,
            ImageType::Type2D,
            SampleCount::Type_1,
            ImageLayout::Unknown,
        )).unwrap();

        let shadow_map = builder.add_pass("Render Shadow Map", |builder| {
            let shadow_map = builder.depth_stencil_attachment(shadow_map);

            shadow_map
        }, |_, _| {
            // rendering code omitted
        });

        // define all render passes needed to render the frame
        let (depth_buffer, velocity_buffer) = builder.add_pass("Z Prepass", |builder| {
            let depth_buffer = builder.depth_stencil_attachment(depth_buffer);
            let velocity_buffer = builder.color_attachment(velocity_buffer);

            (depth_buffer, velocity_buffer)
        }, |_, _| {
            // rendering code omitted
        });

        let (hdr_buffer, normals_buffer, specular_buffer, depth_buffer) = builder.add_pass("Render Scene", |builder| {
            
            builder.sample_image(shadow_map);
            let depth_buffer = builder.depth_stencil_attachment(depth_buffer);
            let hdr_buffer = builder.color_attachment(hdr_buffer);
            let normals_buffer = builder.color_attachment(normals_buffer);
            let specular_buffer = builder.color_attachment(specular_buffer);

            (hdr_buffer, normals_buffer, specular_buffer, depth_buffer)
        }, |_, _| {
            // rendering code omitted
        });

        let hdr_buffer = builder.add_pass("Reflections", |builder| {
            
            builder.sample_image(depth_buffer);
            builder.sample_image(normals_buffer);
            builder.sample_image(specular_buffer);

            let hdr_buffer = builder.color_attachment(hdr_buffer);

            hdr_buffer
        }, |_, _| {
            // rendering code omitted
        });

        let back_buffer = builder.add_pass("Post Process", |builder| {
            
            builder.sample_image(depth_buffer);
            builder.sample_image(normals_buffer);
            builder.sample_image(hdr_buffer);
            builder.sample_image(velocity_buffer);
            let back_buffer = builder.color_attachment(back_buffer);


            back_buffer
        }, |_, _| {
            // rendering code omitted
        });

        // construct the graph
        let graph = builder.build(&[back_buffer]);

        // compile the schedule, this linearizes the graph into a list of instructions for the renderer
        let schedule = graph.compile_schedule();

        let elapsed = start.elapsed();
        println!("{:?}", elapsed);

        // panic!("{}", "");
    }
}
