#![allow(clippy::redundant_field_names)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        camera::ScalingMode,
        extract_resource::{ExtractResource, ExtractResourcePlugin},
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

pub const CLEAR: Color = Color::rgb(0.3, 0.3, 0.3);
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
        .add_system(adjust_colordata_via_kb)
        .add_plugin(ExtractResourcePlugin::<ExtractedTime>::default())
        .add_system(setup_shader);

    app.sub_app_mut(RenderApp)
        .add_system_to_stage(RenderStage::Extract, extract_data_to_cool_material)
        .add_system_to_stage(RenderStage::Prepare, prepare_cool_material);

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

// This is what we will be interacing with
#[derive(Component, ShaderType, Clone, Copy)]
struct CoolMaterialUniformData {
    color: Color,
    time: f32,
}

#[derive(Clone, Default)]
struct Fire {
    position: Vec2,
    size: f32,
}

#[derive(Clone, Default)]
struct ActiveFires {
    active_fires: Vec<Fire>,
}

impl encase::ShaderType for ActiveFires {
    type ExtraMetadata = ();

    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = {
        let size =
            encase::private::SizeValue::from(<Vec2 as encase::private::ShaderSize>::SHADER_SIZE);
        let alignment = encase::private::AlignmentValue::from_next_power_of_two_size(size);

        encase::private::Metadata {
            alignment,
            has_uniform_min_alignment: false,
            min_size: size,
            extra: (),
        }
    };

    const UNIFORM_COMPAT_ASSERT: fn() = || {};
}

impl encase::private::WriteInto for ActiveFires {
    fn write_into<B: encase::private::BufferMut>(&self, writer: &mut encase::private::Writer<B>) {
        let linear = [0];
        for el in &linear {
            encase::private::WriteInto::write_into(el, writer);
        }
    }
}

impl encase::private::ReadFrom for ActiveFires {
    fn read_from<B: encase::private::BufferRef>(
        &mut self,
        reader: &mut encase::private::Reader<B>,
    ) {
        let mut buffer = [0.0f32; 4];
        for el in &mut buffer {
            encase::private::ReadFrom::read_from(el, reader);
        }

        *self = ActiveFires {
            active_fires: vec![Fire {
                position: Vec2::new(buffer[0], buffer[1]),
                size: buffer[2],
            }],
        }
    }
}
impl encase::private::CreateFrom for ActiveFires {
    fn create_from<B>(reader: &mut encase::private::Reader<B>) -> Self
    where
        B: encase::private::BufferRef,
    {
        // These are intentionally not inlined in the constructor to make this
        // resilient to internal Color refactors / implicit type changes.
        let red: f32 = encase::private::CreateFrom::create_from(reader);
        let green: f32 = encase::private::CreateFrom::create_from(reader);
        let blue: f32 = encase::private::CreateFrom::create_from(reader);
        let alpha: f32 = encase::private::CreateFrom::create_from(reader);
        ActiveFires {
            active_fires: vec![Fire {}],
        }
    }
}

impl encase::ShaderSize for ActiveFires {}

// Used inside the wgsl. Only initialized, but further interactions
// are done via the CoolMaterialUniformData struct
#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CoolMaterial {
    #[uniform(0)]
    color: Color,
    #[uniform(0)]
    time: f32,
    #[uniform(0)]
    fires: ActiveFires,
    #[texture(1)]
    #[sampler(2)]
    image: Handle<Image>,
}

impl Default for CoolMaterial {
    fn default() -> Self {
        Self {
            color: Color::rgb(0.0, 0.0, 0.0),
            time: 0.0,
            image: Default::default(),
            fires: ActiveFires::default(),
        }
    }
}

impl Material2d for CoolMaterial {
    fn fragment_shader() -> ShaderRef {
        "my_material_2.wgsl".into()
    }
}

fn setup_shader(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut my_material_assets: ResMut<Assets<CoolMaterial>>,
    assets: Res<AssetServer>,
) {
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: mesh_assets.add(Mesh::from(shape::Quad::default())).into(),
            material: my_material_assets.add(CoolMaterial {
                image: assets.load("awesome.png"),
                ..Default::default()
            }),
            transform: Transform::from_xyz(-0.6, 0.0, 0.0),
            ..default()
        })
        .insert(CoolMaterialUniformData {
            color: Color::rgb(0.0, 1.0, 0.0),
            time: 0.0,
        });
    // commands
    //     .spawn_bundle(MaterialMesh2dBundle {
    //         mesh: mesh_assets.add(Mesh::from(shape::Quad::default())).into(),
    //         material: my_material_assets.add(CoolMaterial {
    //             image: assets.load("awesome.png"),
    //             ..Default::default()
    //         }),
    //         transform: Transform::from_xyz(0.6, 0.0, 0.0),
    //         ..default()
    //     })
    //     .insert(CoolMaterialUniformData {
    //         color: Color::rgb(1.0, 0.0, 0.0),
    //         time: 0.0,
    //     });
}

struct ExtractedTime {
    seconds_since_startup: f32,
}

impl ExtractResource for ExtractedTime {
    type Source = Time;

    fn extract_resource(time: &Self::Source) -> Self {
        ExtractedTime {
            seconds_since_startup: time.seconds_since_startup() as f32,
        }
    }
}

fn extract_data_to_cool_material(
    mut commands: Commands,
    colordata_query: Extract<Query<(Entity, &CoolMaterialUniformData, &Handle<CoolMaterial>)>>,
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
    colordata_query: Query<(&CoolMaterialUniformData, &Handle<CoolMaterial>)>,
    render_queue: Res<RenderQueue>,
    time: Res<ExtractedTime>,
) {
    for (colordata, handle) in &colordata_query {
        if let Some(material) = materials.get(handle) {
            let binding = &material.bindings[2];
            if let OwnedBindingResource::Buffer(cur_buffer) = binding {
                let mut buffer = encase::UniformBuffer::new(Vec::new());
                // get color from cool material
                buffer
                    .write(&CoolMaterialUniformData {
                        color: colordata.color,
                        time: colordata.time,
                    })
                    .unwrap();

                render_queue.write_buffer(cur_buffer, 0, buffer.as_ref());
            }
        }
    }
}

fn adjust_colordata_via_kb(
    keyboard_input: Res<Input<KeyCode>>,
    mut colordata_query: Query<&mut CoolMaterialUniformData>,
) {
    for mut colordata in colordata_query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Up) {
            colordata.time += 0.0001;
        } else if keyboard_input.pressed(KeyCode::Down) {
            colordata.time -= 0.0001;
        }
        // println!("colordata: {}", colordata.value)
    }
    // println!("");
}
