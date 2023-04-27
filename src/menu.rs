use bevy::{prelude::*, app::AppExit};

use crate::{resource::Fonts, AppState, GameState};

#[derive(Resource)]
struct MenuData(pub Vec<Entity>);

#[derive(Component)]
struct MenuButton(pub ButtonAction);

enum ButtonAction {
    Play,
    Options,
    Exit,
    ToTitle,
}

const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(MenuData(vec![]))
            .add_system(setup_menu.in_schedule(OnEnter(AppState::Menu)))
            .add_system(menu.in_set(OnUpdate(AppState::Menu)))
            .add_system(cleanup.in_schedule(OnExit(AppState::Menu)))

            .add_system(setup_paused.in_schedule(OnEnter(GameState::Paused)))
            .add_system(cleanup.in_schedule(OnExit(GameState::Paused)))

            .add_system(setup_game_over.in_schedule(OnEnter(GameState::GameOver)))
            .add_system(menu.in_set(OnUpdate(GameState::GameOver)))
            .add_system(cleanup.in_schedule(OnExit(GameState::GameOver)));
    }
}

fn setup_menu(mut commands: Commands, fonts: Res<Fonts>, mut menu_data: ResMut<MenuData>) {
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
    menu_data.0.push(root_entity);
}

fn menu(
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MenuButton),
        (Changed<Interaction>, With<Button>, With<MenuButton>),
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
                    ButtonAction::ToTitle => next_state.set(AppState::Menu),
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

fn cleanup(mut commands: Commands, mut menu_data: ResMut<MenuData>) {
    for entity in menu_data.0.iter() {
        if let Some(entity) = commands.get_entity(*entity) {
            entity.despawn_recursive();
        }
    }
    menu_data.0.clear();
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
            MenuButton(action),
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

fn setup_paused(mut commands: Commands, fonts: Res<Fonts>, mut menu_data: ResMut<MenuData>) {
    let root_entity = commands
        .spawn(NodeBundle {
            style: Style {
                // center button
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                padding: UiRect::top(Val::Px(10.0)),
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                gap: Size::height(Val::Px(10.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Paused".to_owned(),
                TextStyle {
                    font: fonts.primary.clone(),
                    font_size: 30.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
            parent.spawn(TextBundle::from_section(
                "Click <Escape> To Resume".to_owned(),
                TextStyle {
                    font: fonts.primary.clone(),
                    font_size: 16.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        }).id();
    menu_data.0.push(root_entity);
}

fn setup_game_over(mut commands: Commands, fonts: Res<Fonts>, mut menu_data: ResMut<MenuData>) {
    let root_entity = commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                gap: Size::height(Val::Px(10.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            button(parent, &fonts, "Return To Title", ButtonAction::ToTitle);
        })
        .id();
    menu_data.0.push(root_entity);
}
