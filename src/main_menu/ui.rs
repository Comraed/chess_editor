use std::fmt::Display;
use crate::{ui_defaults::{MAINBACK, MAINBTN, MAINBTNHOV, MAINBTNSEL, MAINBTNTXT}, RootNode, WindowState};
use bevy::{
    ecs::{
        query::With, 
        system::*}, 
    prelude::*, 
    window::{
        PrimaryWindow, 
        Window}
};

#[derive(Component, Debug, PartialEq, Eq, Hash)]
pub enum MainBtnType {
    DesignPieceBtn,
    DesignLayoutBtn,
    DesignInteractionBtn,
    PlayBtn,
}

impl Display for MainBtnType{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match &self {
            MainBtnType::DesignPieceBtn => "Design Pieces".to_string(),
            MainBtnType::DesignLayoutBtn => "Design Layout".to_string(),
            MainBtnType::DesignInteractionBtn => "Design Interaction".to_string(),
            MainBtnType::PlayBtn => "Play".to_string(),
        };
        write!(f, "{}", str)
    }
}

pub fn setup_main_menu(
    mut commands: Commands,
    mut q_window: Query<&mut Window, With<PrimaryWindow>>,
){
    let mut window = q_window.single_mut();   
    window.resolution.set(640.0, 640.0);
    window.resize_constraints = WindowResizeConstraints{min_width: 400.0, min_height: 350.0, ..default()};

    commands.spawn((NodeBundle{
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(20.0),
            ..default()
        },
        background_color: MAINBACK.into(),
        ..default()
    }, RootNode))
    .with_children(|parent|{
        add_main_btn(parent, MainBtnType::DesignPieceBtn);
        add_main_btn(parent, MainBtnType::DesignLayoutBtn);
        add_main_btn(parent, MainBtnType::DesignInteractionBtn);
        add_main_btn(parent, MainBtnType::PlayBtn);
    });
}

pub fn handle_main_menu_ui(
    mut q_interaction: Query<(&Interaction, &mut BackgroundColor, &MainBtnType),(Changed<Interaction>, With<MainBtnType>)>,
    mut state: ResMut<NextState<WindowState>>,
){
    for (interaction, mut back_color, btn_type) in q_interaction.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                back_color.0 = MAINBTNSEL;
                match btn_type {
                    MainBtnType::DesignPieceBtn => {state.set(WindowState::DesignPieces)},
                    MainBtnType::DesignLayoutBtn => {state.set(WindowState::DesignLayout)},
                    MainBtnType::DesignInteractionBtn => {state.set(WindowState::DesignInteraction)},
                    MainBtnType::PlayBtn => {state.set(WindowState::PlayGame)},
                }
            },
            Interaction::Hovered => {
                back_color.0 = MAINBTNHOV
            }
            Interaction::None => {
                back_color.0 = MAINBTN
            }
        }
    }
}

fn add_main_btn(parent :&mut ChildBuilder<'_>, btn_type: MainBtnType){
   
    let txt:&str = &btn_type.to_string();

    parent.spawn((
        ButtonBundle{
            button: Button,
            style: Style{
                width: Val::Px(300.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: MAINBTN.into(),
            ..default()
        }, 
        btn_type
    ))
    .with_children(|button|{
        button.spawn(TextBundle::from_section(
            txt,
            TextStyle { 
                font_size: 16., 
                color: MAINBTNTXT.clone(), 
                ..default()
            }
        ));
    });
}

pub fn despawn_main_menu(
    mut commands: Commands,
    q_despawn: Query<Entity, With<RootNode>>,
){
    q_despawn.iter().for_each(|e| commands.entity(e).despawn_recursive())
}

fn update_select_pieces(){
    
}