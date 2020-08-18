use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub const GAME_HEIGHT:f32 = 100.0;
pub const GAME_WIDTH:f32 = 100.0;
pub const PADDLE_HEIGHT:f32 = 16.0;
pub const PADDLE_WIDTH:f32 = 4.0;


fn init_camera(world: &mut World){
    let mut transform = Transform::default();
    transform.set_translation_xyz(GAME_WIDTH * 0.5, GAME_HEIGHT * 0.5, 1.0 );
    world
    .create_entity()
    .with(Camera::standard_2d(GAME_WIDTH, GAME_HEIGHT))
    .with(transform)
    .build();
}

fn init_paddles(world: &mut World, spritesheet_handle: Handle<SpriteSheet>){
    let (mut left_transform, mut right_transform) = (Transform::default(), Transform::default());
    let middle_y = GAME_HEIGHT / 2.0;
    left_transform.set_translation_xyz(PADDLE_WIDTH * 0.5, middle_y, 0.0);
    right_transform.set_translation_xyz(GAME_WIDTH - (PADDLE_WIDTH*0.5), middle_y, 0.0);
    let sprite_number = 0;
    let sprite_sheet = spritesheet_handle;
    let sprite_render = SpriteRender{sprite_sheet, sprite_number};

    world.create_entity()
        .with(sprite_render.clone())
        .with(Paddle::new(Side::Left))
        .with(left_transform)
        .build();
    world.create_entity()
        .with(sprite_render)
        .with(Paddle::new(Side::Right))
        .with(right_transform)
        .build();
}

fn load_sprites(world: &mut World) -> Handle<SpriteSheet>{
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "textures/pong_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "textures/pong_spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

pub struct GameState;

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    fn new(side: Side) -> Paddle {
        Paddle {
            side,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

impl SimpleState for GameState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        let world = _data.world;
        
        let sprite_sheet_handle = load_sprites(world);
        world.register::<Paddle>();
        
        init_camera(world);
        init_paddles(world, sprite_sheet_handle);
    }
}