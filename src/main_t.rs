#![allow(clippy::redundant_field_names)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use bevy::{
    prelude::{shape::Quad, *},
    reflect::TypeUuid,
    render::{
        camera::ScalingMode,
        render_resource::{
            encase::{self},
            AsBindGroup, OwnedBindingResource, ShaderRef, ShaderType,
        },
        renderer::RenderQueue,
        Extract, RenderApp, RenderStage,
    },
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, RenderMaterials2d},
    window::PresentMode,
};

pub const CLEAR: Color = Color::rgb(1.0, 1.0, 1.0);
pub const HEIGHT: f32 = 600.0;
pub const RESOLUTION: f32 = 4.0 / 3.0;

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: HEIGHT * RESOLUTION,
            height: HEIGHT,
            title: "Bevy Material Tutorial".to_string(),
            present_mode: PresentMode::Fifo,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(Material2dPlugin::<CoolMaterial>::default())
        .add_startup_system(spawn_camera)
        // .add_system(adjust_colordata_via_kb)
        .add_system(setup_shader);

    // app.sub_app_mut(RenderApp)
    // .add_system_to_stage(RenderStage::Extract, extract_data_to_cool_material)
    // .add_system_to_stage(RenderStage::Prepare, prepare_cool_material);

    app.run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.right = 1.0 * RESOLUTION;
    camera.projection.left = -1.0 * RESOLUTION;

    camera.projection.top = 1.0;
    camera.projection.bottom = -1.0;

    camera.projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}

// This will be serialized into a buffer
#[derive(ShaderType, Clone)]
struct CoolMaterialUniformBuffer {
    color: Color,
    time: f32,
    // position: [Vec4; 10],
    // size: [f32; 2],
}

// This is what we will be interacing with
#[derive(Component, Clone, Copy)]
struct CoolMaterialUniformInput {
    color: Color,
    time: f32,
    // position: [Vec4; 10],
    // size: [f32; 2],
}

impl Default for CoolMaterialUniformInput {
    fn default() -> Self {
        Self {
            color: Color::rgba(0.0, 0.0, 0.0, 0.15),
            time: 0.005,
            // position: [
            //     Vec4::new(0.5, 0.0, 0.25, 0.0),
            //     Vec4::new(-0.5, 0.0, 0.5, 0.0),
            //     Vec4::default(),
            //     Vec4::default(),
            //     Vec4::default(),
            //     Vec4::default(),
            //     Vec4::default(),
            //     Vec4::default(),
            //     Vec4::default(),
            //     Vec4::default(),
            // ],
            // position: [Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0)],
            // size: [0.0, 0.0],
        }
    }
}

// Used inside the wgsl. Only initialized, but further interactions
// are done via the CoolMaterialUniformData struct
#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "a0b0c0d0-e0f0-4a4b-b0c0-d0e0f0a4b4c4"]
pub struct CoolMaterial {
    #[uniform(0)]
    color: Color,
    #[uniform(0)]
    time: f32,
    // #[uniform(0)]
    // position: [Vec4; 10],
}

impl Default for CoolMaterial {
    fn default() -> Self {
        Self {
            color: Color::rgba(0.0, 0.0, 0.0, 0.0),
            time: 0.0,
            // position: [
            //     Vec4::default(),
            //     Vec4::default(),
            //     Vec4::default(),
            //     Vec4::default(),
            //     Vec4::default(),
            //     Vec4::default(),
            //     Vec4::default(),
            //     Vec4::default(),
            //     Vec4::default(),
            //     Vec4::default(),
            // ],
            // position: [Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0)],
            // size: [0.0, 0.0],
        }
    }
}

impl Material2d for CoolMaterial {
    fn fragment_shader() -> ShaderRef {
        "my_material_t.wgsl".into()
    }
}

fn setup_shader(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut my_material_assets: ResMut<Assets<CoolMaterial>>,
) {
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: mesh_assets
                .add(Mesh::from(shape::Quad::from(Quad {
                    size: Vec2::new(0.5, 0.5),
                    ..Default::default()
                })))
                .into(),
            material: my_material_assets.add(CoolMaterial {
                // image: assets.load("awesome.png"),
                ..Default::default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(CoolMaterialUniformInput::default());
}

fn extract_data_to_cool_material(
    mut commands: Commands,
    colordata_query: Extract<Query<(Entity, &CoolMaterialUniformInput, &Handle<CoolMaterial>)>>,
) {
    for (entity, colordata, handle) in colordata_query.iter() {
        commands
            .get_or_spawn(entity)
            .insert(*colordata)
            .insert(handle.clone());
    }
}

fn prepare_cool_material(
    materials: Res<RenderMaterials2d<CoolMaterial>>,
    colordata_query: Query<(&CoolMaterialUniformInput, &Handle<CoolMaterial>)>,
    render_queue: Res<RenderQueue>,
) {
    for (colordata, handle) in &colordata_query {
        if let Some(material) = materials.get(handle) {
            let binding = &material.bindings[0];
            if let OwnedBindingResource::Buffer(cur_buffer) = binding {
                let mut buffer = encase::UniformBuffer::new(Vec::new());
                // get color from cool material
                // buffer
                //     .write(&CoolMaterialUniformData {
                //         color: colordata.color,
                //         time: colordata.time,
                //         position: colordata.position,
                //         // size: colordata.size,
                //     })
                //     .unwrap();

                render_queue.write_buffer(cur_buffer, 0, buffer.as_ref());
            }
        }
    }
}

fn adjust_colordata_via_kb(
    keyboard_input: Res<Input<KeyCode>>,
    mut colordata_query: Query<&mut CoolMaterialUniformInput>,
) {
    for mut colordata in colordata_query.iter_mut() {
        // if keyboard_input.pressed(KeyCode::A) {
        //     colordata.position[0].x -= 0.01;
        // } else if keyboard_input.pressed(KeyCode::D) {
        //     colordata.position[0].x += 0.01;
        // } else if keyboard_input.pressed(KeyCode::S) {
        //     colordata.position[0].y -= 0.01;
        // } else if keyboard_input.pressed(KeyCode::W) {
        //     colordata.position[0].y += 0.01;
        // } else if keyboard_input.pressed(KeyCode::Q) {
        //     colordata.position[0].z -= 0.01;
        // } else if keyboard_input.pressed(KeyCode::E) {
        //     colordata.position[0].z += 0.01;
        // }
        // println!("colordata: {}", colordata.value)
    }
    // println!("");
}
