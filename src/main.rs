use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;
use std::collections::{HashSet, HashMap};

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Ingredient {
    EyeOfNewt,
    DragonScale,
    Moondew,
    GoblinEarwax,
    PhoenixFeather,
}

impl Ingredient {
    fn all() -> [Ingredient; 5] {
        [
            Ingredient::EyeOfNewt,
            Ingredient::DragonScale,
            Ingredient::Moondew,
            Ingredient::GoblinEarwax,
            Ingredient::PhoenixFeather,
        ]
    }

    fn name(&self) -> &'static str {
        match self {
            Ingredient::EyeOfNewt => "Eye of Newt",
            Ingredient::DragonScale => "Dragon Scale",
            Ingredient::Moondew => "Moondew",
            Ingredient::GoblinEarwax => "Goblin Earwax",
            Ingredient::PhoenixFeather => "Phoenix Feather",
        }
    }
}

#[derive(Resource, Default)]
struct SelectedIngredients(HashSet<Ingredient>);

#[derive(Resource, Default)]
struct PotionResult(String);

#[derive(Resource, Default)]
struct PotionName(String);

#[derive(Resource, Default)]
struct NamingState {
    active: bool,
}

#[derive(Component)]
struct IngredientButton(Ingredient);

#[derive(Component)]
struct MixButton;

#[derive(Component)]
struct SelectedText;

#[derive(Component)]
struct ResultText;

#[derive(Component)]
struct NamePromptText;

#[derive(Component)]
struct PotionNameText;

#[derive(Resource, Default)]
struct SavedPotionNames(HashMap<Vec<&'static str>, String>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<SelectedIngredients>()
        .init_resource::<PotionResult>()
        .init_resource::<PotionName>()
        .init_resource::<NamingState>()
        .init_resource::<SavedPotionNames>()
        .add_systems(Startup, setup_ui)
        .add_systems(Update, button_interaction_system)
        .add_systems(Update, name_input_system)
        .add_systems(Update, update_text_system)
        .run();
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                padding: UiRect::all(Val::Px(20.0)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Potion Lab",
                TextStyle {
                    font: font.clone(),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ));

            parent
                .spawn(NodeBundle {
                    style: Style {
                        margin: UiRect::top(Val::Px(15.0)),
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|row| {
                    for ing in Ingredient::all() {
                        row.spawn(ButtonBundle {
                            style: Style {
                                margin: UiRect::all(Val::Px(6.0)),
                                padding: UiRect::all(Val::Px(8.0)),
                                ..default()
                            },
                            background_color: BackgroundColor(Color::rgb(0.7, 0.7, 0.7)),
                            ..default()
                        })
                        .insert(IngredientButton(ing))
                        .with_children(|b| {
                            b.spawn(TextBundle::from_section(
                                ing.name(),
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 16.0,
                                    color: Color::BLACK,
                                },
                            ));
                        });
                    }
                });

            parent
                .spawn(ButtonBundle {
                    style: Style {
                        margin: UiRect::top(Val::Px(20.0)),
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::rgb(0.0, 0.5, 0.0)),
                    ..default()
                })
                .insert(MixButton)
                .with_children(|b| {
                    b.spawn(TextBundle::from_section(
                        "Mix!",
                        TextStyle {
                            font: font.clone(),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    ));
                });

            parent
                .spawn(TextBundle::from_section(
                    "Selected: (none)",
                    TextStyle {
                        font: font.clone(),
                        font_size: 18.0,
                        color: Color::WHITE,
                    },
                ))
                .insert(SelectedText);

            parent
                .spawn(TextBundle::from_section(
                    "Result: ",
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.0,
                        color: Color::rgb(1.0, 0.84, 0.0),
                    },
                ))
                .insert(ResultText);

            parent
                .spawn(TextBundle::from_section(
                    "",
                    TextStyle {
                        font: font.clone(),
                        font_size: 18.0,
                        color: Color::WHITE,
                    },
                ))
                .insert(NamePromptText);

            parent
                .spawn(TextBundle::from_section(
                    "",
                    TextStyle {
                        font,
                        font_size: 18.0,
                        color: Color::rgb(0.8, 0.8, 0.2),
                    },
                ))
                .insert(PotionNameText);
        });
}

fn button_interaction_system(
    mut selected: ResMut<SelectedIngredients>,
    mut potion_result: ResMut<PotionResult>,
    mut potion_name: ResMut<PotionName>,
    mut naming_state: ResMut<NamingState>,
    saved_names: Res<SavedPotionNames>,
    mut query: Query<
        (&Interaction, &mut BackgroundColor, Option<&IngredientButton>, Option<&MixButton>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background, ing_opt, mix_opt) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                *background = BackgroundColor(Color::rgb(0.33, 0.33, 0.33));
                if let Some(ingredient_button) = ing_opt {
                    if selected.0.contains(&ingredient_button.0) {
                        selected.0.remove(&ingredient_button.0);
                    } else {
                        selected.0.insert(ingredient_button.0);
                    }
                } else if mix_opt.is_some() {
                    let result = identify_potion(&selected.0);
                    potion_result.0 = result.to_string();

                    // Check if this mixture has a saved name
                    let key = get_ingredient_key(&selected.0);
                    if result == "Unknown Mixture" {
                        if let Some(saved_name) = saved_names.0.get(&key) {
                            potion_name.0 = saved_name.clone();
                            naming_state.active = false;
                        } else {
                            potion_name.0.clear();
                            naming_state.active = true;
                        }
                    } else {
                        potion_name.0.clear();
                        naming_state.active = false;
                    }
                }
            }
            Interaction::Hovered => {
                *background = BackgroundColor(Color::rgb(0.5, 1.0, 0.5));
            }
            Interaction::None => {
                *background = BackgroundColor(Color::rgb(0.7, 0.7, 0.7));
            }
        }
    }
}

fn name_input_system(
    mut key_events: EventReader<KeyboardInput>,
    mut potion_name: ResMut<PotionName>,
    mut naming_state: ResMut<NamingState>,
    selected: Res<SelectedIngredients>,
    mut saved_names: ResMut<SavedPotionNames>,
) {
    if !naming_state.active {
        return;
    }

    for event in key_events.read() {
        if event.state.is_pressed() {
            match event.key_code {
                KeyCode::Backspace => {
                    potion_name.0.pop();
                }
                KeyCode::Enter => {
                    // Save the potion name before deactivating
                    if !potion_name.0.is_empty() {
                        let key = get_ingredient_key(&selected.0);
                        saved_names.0.insert(key, potion_name.0.clone());
                    }
                    naming_state.active = false;
                }
                _ => {
                    if let bevy::input::keyboard::Key::Character(c) = &event.logical_key {
                        if potion_name.0.len() < 24 {
                            potion_name.0.push_str(c);
                        }
                    }
                }
            }
        }
    }
}

fn get_ingredient_key(selected: &HashSet<Ingredient>) -> Vec<&'static str> {
    let mut names: Vec<_> = selected.iter().map(|i| i.name()).collect();
    names.sort();
    names
}

fn identify_potion(set: &HashSet<Ingredient>) -> &'static str {
    use Ingredient::*;
    if set.contains(&EyeOfNewt) && set.contains(&Moondew) && set.len() == 2 {
        "Healing Draught"
    } else if set.contains(&DragonScale) && set.contains(&PhoenixFeather) && set.len() == 2 {
        "Fire Resistance"
    } else if set.contains(&GoblinEarwax)
        && set.contains(&EyeOfNewt)
        && set.contains(&DragonScale)
    {
        "Stinking Curse"
    } else if set.len() >= 4 {
        "Chaos Elixir"
    } else if set.is_empty() {
        "(none)"
    } else {
        "Unknown Mixture"
    }
}

fn update_text_system(
    selected: Res<SelectedIngredients>,
    potion_result: Res<PotionResult>,
    potion_name: Res<PotionName>,
    naming_state: Res<NamingState>,
    mut texts: ParamSet<(
        Query<&mut Text, With<SelectedText>>,
        Query<&mut Text, With<ResultText>>,
        Query<&mut Text, With<NamePromptText>>,
        Query<&mut Text, With<PotionNameText>>,
    )>,
) {
    if let Ok(mut selected_text) = texts.p0().get_single_mut() {
        if selected.0.is_empty() {
            selected_text.sections[0].value = "Selected: (none)".to_string();
        } else {
            let mut names: Vec<_> = selected.0.iter().map(|i| i.name()).collect();
            names.sort();
            selected_text.sections[0].value = format!("Selected: {}", names.join(", "));
        }
    }

    if let Ok(mut result_text) = texts.p1().get_single_mut() {
        result_text.sections[0].value = format!("Result: {}", potion_result.0);
    }

    if let Ok(mut prompt_text) = texts.p2().get_single_mut() {
        if naming_state.active {
            prompt_text.sections[0].value =
                "Name this unknown mixture and press Enter:".to_string();
        } else {
            prompt_text.sections[0].value = "".to_string();
        }
    }

    if let Ok(mut name_text) = texts.p3().get_single_mut() {
        if !potion_name.0.is_empty() {
            name_text.sections[0].value = format!("Potion name: {}", potion_name.0);
        } else {
            name_text.sections[0].value = "".to_string();
        }
    }
}
