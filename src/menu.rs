use bevy::{prelude::*, app::AppExit};

use crate::{resource::Fonts, AppState};

#[derive(Resource)]
struct MainMenuData {
    root_entity: Entity,
}

#[derive(Component)]
struct MainMenuButton(pub ButtonAction);

enum ButtonAction {
    Play,
    Options,
    Exit,
}

const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_menu.in_schedule(OnEnter(AppState::Menu)))
            .add_system(menu.in_set(OnUpdate(AppState::Menu)))
            .add_system(cleanup_menu.in_schedule(OnExit(AppState::Menu)));
    }
}

fn setup_menu(mut commands: Commands, fonts: Res<Fonts>) {
    let root_entity = commands
        .spawn(NodeBundle {
            style: Style {
                // center button
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                gap: Size::height(Val::Px(10.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            button(parent, &fonts, "Play", ButtonAction::Play);
            button(parent, &fonts, "Options", ButtonAction::Options);
            button(parent, &fonts, "Exit", ButtonAction::Exit);
        })
        .id();
    commands.insert_resource(MainMenuData { root_entity });
}

fn menu(
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MainMenuButton),
        (Changed<Interaction>, With<Button>, With<MainMenuButton>),
    >,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, mut color, button) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                match button.0 {
                    ButtonAction::Play => next_state.set(AppState::InGame),
                    ButtonAction::Options => (),
                    ButtonAction::Exit => exit.send(AppExit),
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu_data: Res<MainMenuData>) {
    commands.entity(menu_data.root_entity).despawn_recursive();
}

fn button(parent: &mut ChildBuilder, fonts: &Res<Fonts>, text: &str, action: ButtonAction) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    min_size: Size::width(Val::Px(200.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: NORMAL_BUTTON.into(),
                ..default()
            },
            MainMenuButton(action),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font: fonts.primary.clone(),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
}
