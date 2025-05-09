use std::f32::consts::PI;

use bevy::{color::palettes::css::WHITE, gltf::{GltfMesh, GltfNode}, math::ops::sin_cos, prelude::*};

use bevy_asset_loader::asset_collection::AssetCollection;

#[cfg(feature = "egui")]
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bimap::BiMap;
use rand::{rngs::StdRng, Rng as _, SeedableRng};

fn main() {
    use bevy_asset_loader::loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt};

    let mut app = App::new();

    app
        .add_plugins(DefaultPlugins)
        // .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .insert_resource(AmbientLight {
            // brightness: 750.0,
            brightness: 200.0,
            ..default()
        })
        .insert_resource(CellsParam {
            cell_table: CellTable::new("\
               ┌→→→→→→→→→┐
               ↑ ┌→→→→→┐ ↓
               ↑ ↑0   0↓ ↓
               ↑ └←←←←←┘ ↓
               ↑ ┏←┓ ┌→┐ ↓
               ↑ ↓0↑ ↑0↓ ↓
               ↑ ┗→┛ └←┘ ↓
               ↑ ┏←←←←←┓ ↓
               ↑ ↓0   0↑ ↓
               ↑ ┗→→→→→┛ ↓
               └←←←←←←←←←┘\
                "),
            cell_size: Vec2::new(50.0, 50.0),
            circle_size: 10.0,
            span_sec: 1.0,
        })
        .init_state::<AssetLoadingState>()
        .add_loading_state(
            LoadingState::new(AssetLoadingState::Loading)
                .continue_to_state(AssetLoadingState::Loaded)
                .load_collection::<GltfAssets>()
        )
        .add_systems(Startup, spawn_loading_text)
        .add_systems(OnEnter(AssetLoadingState::Loaded), cleanup_loading_text.before(setup))
        .add_systems(OnEnter(AssetLoadingState::Loaded), setup)
        .add_systems(Update, move_cells)
        // .add_systems(Update, swing_camera)
        ;

    #[cfg(feature = "egui")]
    app
        .add_plugins(EguiPlugin{enable_multipass_for_primary_context: false})
        .add_systems(Update, ui_system);

    app
        .run();
}

#[derive(Component)]
struct LoadingText;

#[derive(Component)]
struct Cell {
    pub pos: Vec2,
    pub move_type: MoveType,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum MoveType {
    Blank,
    Center,
    Left,
    BottomToLeft,
    TopToLeft,
    Right,
    BottomToRight,
    TopToRight,
    Up,
    LeftToTop,
    RightToTop,
    Down,
    LeftToBottom,
    RightToBottom,
}

impl Cell {
    fn new(pos: Vec2, move_type: MoveType) -> Self {
        Cell { pos, move_type }
    }
}

struct CellTable {
    pub table: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
}

impl CellTable {
    fn new(_cell_info: &str) -> Self {
        // first, trimming
        let cell_info = _cell_info.lines().map(|line| line.trim()).collect::<Vec<&str>>().join("\n");
        println!("cell_info:\n{}", cell_info);

        // construct a table
        let mut table = Vec::new();

        let width = cell_info.lines().map(|line| line.chars().count()).max().unwrap();
        let height = cell_info.lines().count();

        for line in cell_info.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c);
            }
            table.push(row);
        }

        CellTable {
            width,
            height,
            table,
        }
    }

    fn get(&self, x: usize, y: usize) -> char {
        if y >= self.height {
            return ' ';
        }
        let row = &self.table[y];
        if x >= row.len() {
            return ' ';
        }
        row[x]
    }
}

#[derive(Resource)]
struct CellsParam {
    pub cell_table: CellTable,
    pub cell_size: Vec2,
    pub circle_size: f32,
    pub span_sec: f32,
}

fn spawn_loading_text(mut commands: Commands) {
    commands
        .spawn( (
            Text::new("loading..."),
            Node {
                position_type: PositionType::Relative,
                top: Val::Percent(50.0),
                left: Val::Percent(50.0),
                ..default()
            },
            LoadingText,
        ));
}

fn cleanup_loading_text(
    mut commands: Commands,
    loading_text: Query<Entity, With<LoadingText>>,
) {
    for entity in loading_text.iter() {
        commands.entity(entity).despawn();
    }
}

#[derive(AssetCollection, Resource)]
pub struct GltfAssets {
//   #[asset(path = "models/stairs.glb")]
//   pub iroha: Handle<Gltf>,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AssetLoadingState {
    #[default]
    Loading,
    Loaded,
}

struct MyTransform(Transform);

impl From<Vec2> for MyTransform {
    fn from(value: Vec2) -> Self {
        MyTransform(Transform::from_xyz(value.x, value.y, 0.0))
    }
}

fn move_type_from_char(c: char) -> MoveType {
    // NOTE
    // - thin keisen: clock wise
    // - thick keisen: counter clock wise
    match c {
        ' ' => MoveType::Blank,
        '0' => MoveType::Center,
        '←' => MoveType::Left,
        '┓' => MoveType::BottomToLeft,
        '┘' => MoveType::TopToLeft,
        '→' => MoveType::Right,
        '┌' => MoveType::BottomToRight,
        '┗' => MoveType::TopToRight,
        '↑' => MoveType::Up,
        '┛' => MoveType::LeftToTop,
        '└' => MoveType::RightToTop,
        '↓' => MoveType::Down,
        '┐' => MoveType::LeftToBottom,
        '┏' => MoveType::RightToBottom,
        _ => panic!("Invalid cell type: {}", c),
    }
}

fn create_cell(cell_type: char, pos: Vec2) -> Cell {
    let move_type = move_type_from_char(cell_type);
    Cell::new(pos, move_type)
}

fn setup(
    mut commands: Commands,
    // mut asset_server: ResMut<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    // gltf_res: Res<GltfAssets>,
    // assets_gltf: Res<Assets<Gltf>>,
    // assets_gltfmeshes: Res<Assets<GltfMesh>>,
    // assets_gltfnodes: Res<Assets<GltfNode>>,
    mut cells_param: ResMut<CellsParam>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Create a camera
    commands.spawn((
        Camera2d::default(),
    ));

        // commands.spawn((
        //     Mesh3d(mesh_handle.clone()),
        //     Transform::from_xyz(x, y, z).with_rotation(rotation),
        //     MeshMaterial3d( materials.add(
        //         StandardMaterial {
        //             base_color: Color::srgb(0.8, 0.7, 0.6),
        //             ..default()
        //         }
        //     ))
        // ));
        
    let mesh = meshes.add(Circle::new (
        cells_param.circle_size
    ));

    let pos = Vec2::new(0.0, 0.0);
    let rot = Quat::from_rotation_z(0.0);

    let w = cells_param.cell_table.width;
    let h = cells_param.cell_table.height;

    let base_x = -((w as f32) * cells_param.cell_size.x) / 2.0;
    let base_y = -((h as f32) * cells_param.cell_size.y) / 2.0;

    for _iy in 0..h {
        // flip y
        let iy = h - _iy - 1;
        for ix in 0..w {
            let c = cells_param.cell_table.get(ix, iy);
            println!("{}, {} = {:?}", ix, _iy, move_type_from_char(c));
            let x = ix as f32 * cells_param.cell_size.x + base_x;
            let y = _iy as f32 * cells_param.cell_size.y + base_y;
            let pos = Vec2::new(x as f32, y as f32);
            let rot = Quat::from_rotation_z(0.0);
            commands.spawn((
                Mesh2d(mesh.clone()),
                MyTransform::from(pos).0.with_rotation(rot),
                MeshMaterial2d(materials.add(Color::from(WHITE))),
                create_cell(c, pos),
            ));
        }
    }
}

fn map (
    input: f32,
    in_min: f32,
    in_max: f32,
    out_min: f32,
    out_max: f32,
) -> f32 {
    (input - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

fn move_cells(
    time: Res<Time>,
    mut cells_param: ResMut<CellsParam>,
    mut cells: Query<(&mut Transform, &mut Cell)>,
) {
    let ss = cells_param.span_sec;
    let w = cells_param.cell_size.x;
    let h = cells_param.cell_size.y;
    let w_hf = w / 2.0;
    let h_hf = h / 2.0;
    let rate: f32 = ((time.elapsed_secs_f64() % (ss as f64)) / (ss as f64)) as f32;

    // move circle from right to left
    for (mut transform, mut cell) in cells.iter_mut() {
        let x = cell.pos.x;
        let y = cell.pos.y;
        let move_type = cell.move_type;
        match move_type {
            MoveType::Blank => {
                // WORKAROUND
                transform.translation.x = -999999.0;
                transform.translation.y = -999999.0;
            }
            MoveType::Center => {
                // do nothing
                // transform.translation.x = x;
                // transform.translation.y = y;
            }
            MoveType::Left => {
                transform.translation.x = map(rate, 0.0, 1.0, x + w_hf, x - w_hf);
            }
            MoveType::BottomToLeft => {
                transform.translation.x = map(rate, 0.0, 1.0, x, x - w_hf);
                transform.translation.y = map(rate, 0.0, 1.0, y - w_hf, y);
            }
            MoveType::TopToLeft => {
                transform.translation.x = map(rate, 0.0, 1.0, x, x - w_hf);
                transform.translation.y = map(rate, 0.0, 1.0, y + h_hf, y);
            }
            MoveType::Right => {
                transform.translation.x = map(rate, 0.0, 1.0, x - w_hf, x + w_hf);
            }
            MoveType::BottomToRight => {
                transform.translation.x = map(rate, 0.0, 1.0, x, x + w_hf);
                transform.translation.y = map(rate, 0.0, 1.0, y - w_hf, y);
            }
            MoveType::TopToRight => {
                transform.translation.x = map(rate, 0.0, 1.0, x, x + w_hf);
                transform.translation.y = map(rate, 0.0, 1.0, y + h_hf, y);
            }
            MoveType::Up => {
                transform.translation.y = map(rate, 0.0, 1.0, y - h_hf, y + h_hf);
            }
            MoveType::LeftToTop => {
                transform.translation.x = map(rate, 0.0, 1.0, x - w_hf, x);
                transform.translation.y = map(rate, 0.0, 1.0, y, y + h_hf);
            }
            MoveType::RightToTop => {
                transform.translation.x = map(rate, 0.0, 1.0, x + w_hf, x);
                transform.translation.y = map(rate, 0.0, 1.0, y, y + h_hf);
            }
            MoveType::Down => {
                transform.translation.y = map(rate, 0.0, 1.0, y + h_hf, y - h_hf);
            }
            MoveType::LeftToBottom => {
                transform.translation.x = map(rate, 0.0, 1.0, x - w_hf, x);
                transform.translation.y = map(rate, 0.0, 1.0, y, y - h_hf);
            }
            MoveType::RightToBottom => {
                transform.translation.x = map(rate, 0.0, 1.0, x + w_hf, x);
                transform.translation.y = map(rate, 0.0, 1.0, y, y - h_hf);
            }
        }
    }

}

#[cfg(feature = "egui")]
fn ui_system(mut contexts: EguiContexts) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}