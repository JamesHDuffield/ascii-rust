use bevy::prelude::*;

use crate::{AppState, resource::Fonts};

#[derive(Resource)]
struct MainMenuData {
    button_entity: Entity,
}

const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
         .add_system(setup_menu.in_schedule(OnEnter(AppState::Menu)))
         .add_system(menu.in_set(OnUpdate(AppState::Menu)))
         .add_system(cleanup_menu.in_schedule(OnExit(AppState::Menu)));
    }
}

fn setup_menu(mut commands: Commands, fonts: Res<Fonts>) {
  let button_entity = commands
      .spawn(NodeBundle {
          style: Style {
              // center button
              size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
              justify_content: JustifyContent::Center,
              align_items: AlignItems::Center,
              ..default()
          },
          ..default()
      })
      .with_children(|parent| {
          parent
              .spawn(ButtonBundle {
                  style: Style {
                      size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                      // horizontally center child text
                      justify_content: JustifyContent::Center,
                      // vertically center child text
                      align_items: AlignItems::Center,
                      ..default()
                  },
                  background_color: NORMAL_BUTTON.into(),
                  ..default()
              })
              .with_children(|parent| {
                  parent.spawn(TextBundle::from_section(
                      "Play",
                      TextStyle {
                          font: fonts.primary.clone(),
                          font_size: 40.0,
                          color: Color::rgb(0.9, 0.9, 0.9),
                      },
                  ));
              });
      })
      .id();
  commands.insert_resource(MainMenuData { button_entity });
}

fn menu(
  mut next_state: ResMut<NextState<AppState>>,
  mut interaction_query: Query<
      (&Interaction, &mut BackgroundColor),
      (Changed<Interaction>, With<Button>),
  >,
) {
  for (interaction, mut color) in &mut interaction_query {
      match *interaction {
          Interaction::Clicked => {
              next_state.set(AppState::InGame);
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
  commands.entity(menu_data.button_entity).despawn_recursive();
}
