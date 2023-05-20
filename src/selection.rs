use bevy::prelude::*;

use crate::{resource::*, GameState, component::{TurretBundle, TurretClass}};

#[derive(Resource)]
struct SelectionData(pub Vec<Entity>);

#[derive(Component)]
struct SelectionButton(pub u8);

const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SelectionData(vec![]))

            .add_system(setup_selection.in_schedule(OnEnter(GameState::Selection)))
            .add_system(menu.in_set(OnUpdate(GameState::Selection)))
            .add_system(cleanup.in_schedule(OnExit(GameState::Selection)));
    }
}

fn setup_selection(mut commands: Commands, fonts: Res<Fonts>, mut menu_data: ResMut<SelectionData>) {

    // Roll for options


    let root_entity = commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                gap: Size::width(Val::Px(10.0)),
                ..Default::default()
            },
            ..default()
        })
        .with_children(|parent| {
            button(parent, &fonts, "Option A", 0);
            button(parent, &fonts, "Option B", 1);
            button(parent, &fonts, "Option C", 2);
        })
        .id();
    menu_data.0.push(root_entity);
}

fn menu(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &SelectionButton),
        (Changed<Interaction>, With<Button>, With<SelectionButton>),
    >,
) {
    for (interaction, mut color, button) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                match button.0 {
                    0 => (),
                    1 => (),
                    2 => (),
                    _ => (),
                }
                next_state.set(GameState::Running);
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

fn cleanup(mut commands: Commands, mut menu_data: ResMut<SelectionData>) {
    for entity in menu_data.0.iter() {
        if let Some(entity) = commands.get_entity(*entity) {
            entity.despawn_recursive();
        }
    }
    menu_data.0.clear();
}

fn button(parent: &mut ChildBuilder, fonts: &Res<Fonts>, text: &str, index: u8) {
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
            SelectionButton(index),
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


