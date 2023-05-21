use bevy::prelude::*;

use crate::{resource::*, GameState, upgrade::UpgradeEvent, colour};


#[derive(Resource)]
struct SelectionData(pub Vec<Entity>);

#[derive(Component)]
struct SelectionButton(UpgradeEvent);

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
    let options: Vec<UpgradeEvent> = (0..3).map(|_| rand::random()).collect();

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
            for option in options {
                button(parent, &fonts, option);
            }
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
    mut upgrade_event: EventWriter<UpgradeEvent>,
) {
    for (interaction, mut color, button) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                upgrade_event.send(button.0);
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

fn button(parent: &mut ChildBuilder, fonts: &Res<Fonts>, upgrade: UpgradeEvent) {
    let type_text = match upgrade {
        UpgradeEvent::Weapon(_) => format!("Weapon"),
        UpgradeEvent::Passive(_) => format!("Passive"),
    };
    let type_color = match upgrade {
        UpgradeEvent::Weapon(_) => colour::RED,
        UpgradeEvent::Passive(_) => colour::SHIELD,
    };
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    min_size: Size::width(Val::Px(200.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: NORMAL_BUTTON.into(),
                ..default()
            },
            SelectionButton(upgrade),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                type_text,
                TextStyle {
                    font: fonts.primary.clone(),
                    font_size: 14.0,
                    color: type_color,
                },
            ));
            parent.spawn(TextBundle::from_section(
                format!("{}", upgrade),
                TextStyle {
                    font: fonts.primary.clone(),
                    font_size: 24.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
}


