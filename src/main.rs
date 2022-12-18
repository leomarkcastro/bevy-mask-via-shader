#![allow(clippy::redundant_field_names)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use bevy::{
    prelude::{shape::Quad, *},
    reflect::TypeUuid,
    render::{
        camera::ScalingMode,
        render_resource::{AsBindGroup, ShaderRef},
        renderer::RenderQueue,
    },
    render::{render_resource::*, Extract, RenderApp, RenderStage},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, RenderMaterials2d},
    window::PresentMode,
};

pub const CLEAR: Color = Color::rgb(1.0, 1.0, 1.0);
pub const HEIGHT: f32 = 600.0;
pub const RESOLUTION: f32 = 4.0 / 3.0;

const MAX_LIGHTS: usize = 64;

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CoolMaterial {
    #[uniform(0)]
    color: Color,
    #[uniform(0)]
    position: [Vec4; MAX_LIGHTS],
}

// create n amount of vec4s using macro
macro_rules! vec4s {
    ($n:expr) => {
        [Vec4::default(); $n]
    };
}

impl Default for CoolMaterial {
    fn default() -> Self {
        Self {
            color: Color::rgba(0.0, 0.0, 0.0, 0.0),
            position: vec4s!(MAX_LIGHTS),
        }
    }
}

impl Material2d for CoolMaterial {
    fn fragment_shader() -> ShaderRef {
        "my_material_t_2.wgsl".into()
    }
}

#[derive(Clone, ShaderType)]
struct CoolMaterialUniformBuffer {
    color: Color,
    position: [Vec4; MAX_LIGHTS],
}

#[derive(Component, Clone, Copy)]
struct CoolMaterialUniformInput {
    color: Color,
    position: [Vec4; MAX_LIGHTS],
}

impl Default for CoolMaterialUniformInput {
    fn default() -> Self {
        Self {
            color: Color::rgba(0.0, 0.0, 0.0, 0.0),
            position: vec4s!(MAX_LIGHTS),
        }
    }
}

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
        // .add_plugin(ExtractResourcePlugin::<ExtractedTime>::default())
        .add_startup_system(setup)
        .add_system(adjust_colordata_via_kb);
    // Add all render world systems/resources
    app.sub_app_mut(RenderApp)
        .add_system_to_stage(RenderStage::Extract, extract_health)
        .add_system_to_stage(RenderStage::Prepare, prepare_my_material);

    app.run();
}

fn setup(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut my_material_assets: ResMut<Assets<CoolMaterial>>,
) {
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: mesh_assets
                .add(Mesh::from(shape::Quad::from(Quad {
                    size: Vec2::new(1., 1.),
                    ..Default::default()
                })))
                .into(),
            material: my_material_assets.add(CoolMaterial {
                ..Default::default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(CoolMaterialUniformInput {
            color: Color::rgba(0.0, 0.0, 0.0, 0.75),
            ..Default::default()
        });
}

fn extract_health(
    mut commands: Commands,
    materialinput_query: Extract<Query<(Entity, &CoolMaterialUniformInput, &Handle<CoolMaterial>)>>,
) {
    for (entity, material_input, handle) in materialinput_query.iter() {
        commands
            .get_or_spawn(entity)
            .insert(*material_input)
            .insert(handle.clone());
    }
}

fn prepare_my_material(
    materials: Res<RenderMaterials2d<CoolMaterial>>,
    health_query: Query<(&CoolMaterialUniformInput, &Handle<CoolMaterial>)>,
    render_queue: Res<RenderQueue>,
) {
    for (material_input, handle) in health_query.iter() {
        if let Some(material) = materials.get(handle) {
            let binding = &material.bindings[0];
            if let OwnedBindingResource::Buffer(cur_buffer) = binding {
                let mut buffer = encase::UniformBuffer::new(Vec::new());
                buffer
                    .write(&CoolMaterialUniformBuffer {
                        color: material_input.color,
                        position: material_input.position,
                    })
                    .unwrap();
                render_queue.write_buffer(cur_buffer, 0, buffer.as_ref());
            }
        }
    }
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

const TIME_SKIP: f32 = 1. / 60.;
const SPEED: f32 = 100.0;

fn adjust_colordata_via_kb(
    keyboard_input: Res<Input<KeyCode>>,
    mut colordata_query: Query<(&mut CoolMaterialUniformInput, &mut Transform)>,
) {
    for (mut colordata, mut transform) in colordata_query.iter_mut() {
        if keyboard_input.pressed(KeyCode::A) {
            colordata.position[0].x -= 0.01 * TIME_SKIP * SPEED;
        } else if keyboard_input.pressed(KeyCode::D) {
            colordata.position[0].x += 0.01 * TIME_SKIP * SPEED;
        } else if keyboard_input.pressed(KeyCode::S) {
            colordata.position[0].y -= 0.01 * TIME_SKIP * SPEED;
        } else if keyboard_input.pressed(KeyCode::W) {
            colordata.position[0].y += 0.01 * TIME_SKIP * SPEED;
        } else if keyboard_input.pressed(KeyCode::Q) {
            colordata.position[0].z -= 0.01 * TIME_SKIP * SPEED;
            colordata.position[0].z = colordata.position[0].z.max(0.0);
        } else if keyboard_input.pressed(KeyCode::E) {
            colordata.position[0].z += 0.01 * TIME_SKIP * SPEED;
        }

        if keyboard_input.pressed(KeyCode::Numpad4) {
            colordata.position[1].x -= 0.01 * TIME_SKIP * SPEED;
        } else if keyboard_input.pressed(KeyCode::Numpad6) {
            colordata.position[1].x += 0.01 * TIME_SKIP * SPEED;
        } else if keyboard_input.pressed(KeyCode::Numpad2) {
            colordata.position[1].y -= 0.01 * TIME_SKIP * SPEED;
        } else if keyboard_input.pressed(KeyCode::Numpad8) {
            colordata.position[1].y += 0.01 * TIME_SKIP * SPEED;
        } else if keyboard_input.pressed(KeyCode::Numpad7) {
            colordata.position[1].z -= 0.01 * TIME_SKIP * SPEED;
            colordata.position[1].z = colordata.position[1].z.max(0.0);
        } else if keyboard_input.pressed(KeyCode::Numpad9) {
            colordata.position[1].z += 0.01 * TIME_SKIP * SPEED;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += 0.01 * TIME_SKIP * SPEED;
        } else if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= 0.01 * TIME_SKIP * SPEED;
        } else if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= 0.01 * TIME_SKIP * SPEED;
        } else if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += 0.01 * TIME_SKIP * SPEED;
        }

        if keyboard_input.pressed(KeyCode::Z) {
            let cur_color_a = colordata.color.a();
            colordata.color.set_a(cur_color_a - 0.01);
        } else if keyboard_input.pressed(KeyCode::X) {
            let cur_color_a = colordata.color.a();
            colordata.color.set_a(cur_color_a + 0.01);
        }
        // println!("colordata: {}", colordata.value)
    }
    // println!("");
}
