use bevy::prelude::*;
use bevy::render::camera::RenderTarget::Window;
use bevy::render::camera::ScalingMode::WindowSize;

pub struct ClickEvent();
pub struct ChangeEvent();

pub struct ButtonColorsResource {
    white: Handle<Image>,
    red: Handle<Image>,
    green: Handle<Image>,
}

impl ButtonColorsResource {
    pub fn red(&self) -> Handle<Image> {
        self.red.clone()
    }

    pub fn green(&self) -> Handle<Image> {
        self.green.clone()
    }

    pub fn white(&self) -> Handle<Image> {
        self.white.clone()
    }
}

#[derive(Component)]
pub struct ButtonTimer {
    timer: Timer,
}

#[derive(Component)]
pub enum ButtonStatus {
    White,
    Red,
    Green,
}

#[derive(Bundle)]
pub struct ButtonBundle {
    status: ButtonStatus,
    timer: ButtonTimer,

    #[bundle]
    sprite: SpriteBundle,
}

impl ButtonStatus {
    fn next_status(&mut self) {
        match self {
            ButtonStatus::White => *self = ButtonStatus::Red,
            ButtonStatus::Red => *self = ButtonStatus::Green,
            ButtonStatus::Green => *self = ButtonStatus::White,
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, time: Res<Time>) {
    commands.spawn_bundle(Camera2dBundle::default());

    let white = asset_server.load("white_signal.png");
    let red = asset_server.load("red_signal.png");
    let green = asset_server.load("green_signal.png");

    commands.spawn_bundle(ButtonBundle {
        status: ButtonStatus::White,
        timer: ButtonTimer {
            timer: Timer::from_seconds(1.0, true),
        },
        sprite: SpriteBundle {
            texture: white.clone(),
            ..Default::default()
        },
    });

    commands.insert_resource(ButtonColorsResource { white, red, green });
}

fn timer(
    mut events: EventWriter<ChangeEvent>,
    time: Res<Time>,
    mut query: Query<&mut ButtonTimer>,
) {
    let mut queried = query.single_mut();

    queried.timer.tick(time.delta());

    if queried.timer.finished() {
        events.send(ChangeEvent());
    }
}

fn changer(
    mut events: EventReader<ChangeEvent>,
    resource: Res<ButtonColorsResource>,
    mut query: Query<(&mut ButtonStatus, &mut Handle<Image>)>,
) {
    for _ in events.iter() {
        for (mut button, mut handler) in query.iter_mut() {
            match *button {
                ButtonStatus::White => {
                    *handler = resource.red();
                    button.next_status();
                }
                ButtonStatus::Red => {
                    *handler = resource.green();
                    button.next_status();
                }
                ButtonStatus::Green => {
                    *handler = resource.white();
                    button.next_status();
                }
            }
        }
    }
}

fn click(
    mut commands: Commands,
    mouse: Res<Input<MouseButton>>,
    resource: Res<ButtonColorsResource>,
    mut event: EventWriter<ClickEvent>,
) {
    //fn click(mut commands: Commands, mouse: Res<Input<MouseButton>>, window: Res<Windows>, resource: Res<ButtonResources>, mut query: Query<(&mut Button, &mut Handle<Image>)>) {
    //if mouse.just_pressed(MouseButton::Left) {
    //       event.send(ClickEvent());
    // match *button {
    //     Button::White => {
    //         *handler = resource.Red.clone();
    //         *button = Button::Red;
    //     },
    //     Button::Red => {
    //         *handler = resource.Green.clone();
    //         *button = Button::Green;
    //     },
    //     Button::Green => {
    //         *handler = resource.White.clone();
    //         *button = Button::White;
    //     }
    // }
    //}
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Reaction".to_string(),
            present_mode: bevy::window::PresentMode::AutoVsync,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_event::<ChangeEvent>()
        //.add_event::<ClickEvent>()
        .add_startup_system(setup)
        .add_system(timer)
        .add_system(changer)
        //.add_system(click)
        .run();
}
