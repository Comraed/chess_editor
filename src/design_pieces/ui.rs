use bevy::{
    ecs::{
        component::Component, entity::Entity, event::EventReader, query::{Changed, With}, system::{Commands, Query, Res, ResMut, Resource}
    }, hierarchy::{BuildChildren, DespawnRecursiveExt}, prelude::default, render::color::Color, text::TextStyle, ui::{node_bundles::{ButtonBundle, NodeBundle, TextBundle}, *}, window::{PrimaryWindow, Window, WindowResizeConstraints, WindowResized, WindowResolution}
};

use crate::{rule_data::piece_set::{MoveType, PieceMoveSet, PieceType}, ui_const::{GRIDOUTER, MAINBACK, MAINBTN, MAINBTNHOV, MAINBTNSEL, MAINBTNTXT, SECBTN, SECBTNHOV, SECBTNSEL, SECBTNTXT, TILEDARK, TILEDARKALT, TILELIGHT, TILELIGHTALT, UIBACK}, RootNode};
use crate::design_pieces::piece_coords::PieceCoord;
use crate::design_pieces::piece_ecs::{GridSquare, InfiniteButton, MidSquare, PieceGrid, RightPanel, SaveButton, UiGridData};

pub fn setup_design_pieces(
    mut commands: Commands,
    mut q_window: Query<&mut Window, With<PrimaryWindow>>,
){
    let mut window = q_window.single_mut();   
    window.resolution = WindowResolution::default();
    window.resize_constraints = WindowResizeConstraints{min_width: 960.0, min_height: 660.0, ..default()};

    commands.spawn((NodeBundle{
        style: Style {
            display: Display::Flex,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Stretch,
            align_items: AlignItems::Stretch,
            row_gap: Val::Px(20.0),
            ..default()
        },
        ..default()
    }, RootNode))
    .with_children(|root| {
        
        root.spawn((NodeBundle{
            style: Style{
                display: Display::Grid,
                height: Val::Percent(100.),
                width: Val::Percent(100.),
                column_gap: Val::Px(2.),
                row_gap: Val::Px(2.),
                border: UiRect::right(Val::Px(5.)),
                ..default()
            },
            border_color: GRIDOUTER.into(),
            background_color: MAINBACK.into(),
            ..default()
        }, PieceGrid));

        root.spawn((NodeBundle{
            style: Style{
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Stretch,
                flex_grow: 0.,
                flex_basis: Val::Px(300.),
                ..default()
            },
            background_color: UIBACK.into(),
            ..default()
        }, RightPanel)).with_children(|right_panel|{

            //menu
            right_panel.spawn(NodeBundle{
                style: Style{
                    display: Display::Flex,
                    flex_grow: 0.,
                    flex_basis: Val::Px(50.),
                    ..default()
                },
                background_color: SECBTN.into(),
                ..default()
            });

            //colors
            right_panel.spawn(NodeBundle{
                style: Style{
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceAround,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(10.),
                    flex_grow: 0.,
                    flex_basis: Val::Px(50.),
                    ..default()
                },
                background_color: UIBACK.into(),
                ..default()
            }).with_children(|select_options|{

                let swatch_style = Style{
                    height: Val::Px(40.),
                    width: Val::Px(40.),
                    flex_grow: 0.,
                    ..default()
                };
                select_options.spawn((NodeBundle{
                    style: swatch_style.clone(),
                    background_color: MoveType::None.get_color(false).into(),
                    ..default()
                }, MoveType::None));

                select_options.spawn((NodeBundle{
                    style: swatch_style.clone(),
                    background_color: MoveType::Enemies.get_color(false).into(),
                    ..default()
                }, MoveType::Enemies));

                select_options.spawn((NodeBundle{
                    style: swatch_style,
                    background_color: MoveType::All.get_color(false).into(),
                    ..default()
                }, MoveType::All));

                select_options.spawn((ButtonBundle{
                    style: Style{
                        height: Val::Px(40.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_grow: 1.,
                        ..default()
                    },
                    background_color: SECBTN.into(),
                    ..default()
                }, InfiniteButton{selected:false})).with_children(|button|{
                        button.spawn(TextBundle::from_section(
                        "Infinite",
                        TextStyle { 
                            font_size: 20., 
                            color: SECBTNTXT.clone(), 
                            ..default()
                        }));
                    });
            });

            //main panel
            right_panel.spawn(NodeBundle{
                style: Style{
                    display: Display::Flex,
                    flex_grow: 1.,
                    ..default()
                },
                background_color: SECBTNSEL.into(),
                ..default()
            });

            //save btn
            right_panel.spawn((ButtonBundle{
                style: Style{
                    display: Display::Flex,
                    flex_grow: 0.,
                    flex_basis: Val::Px(50.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: MAINBTN.into(),
                ..default()
            }, SaveButton)).with_children(|button|{
                button.spawn(TextBundle::from_section(
                    "Save",
                    TextStyle { 
                        font_size: 30., 
                        color: MAINBTNTXT.clone(), 
                        ..default()
                    }
                ));
            });
        });
    });
}

pub fn resize_grid(
    mut e_window_resized: EventReader<WindowResized>,
    mut q_grid_container: Query<(Entity, &Node, &mut Style), With<PieceGrid>>,
    mut commands: Commands,
    grid_data: Res<UiGridData>,
){
    if e_window_resized.read().last().is_some(){
        let (ent, node, mut style) = match q_grid_container.get_single_mut() {
            Ok(style) => {style},
            Err(_) => {return;},
        };

        let grid_range = grid_data.edit_limit + 3;
        let full_dia = grid_range * 2 + 1;

        let min_size = node.size().min_element();

        let get_nearest_odd = |num:f32| -> u16{
            let u_num = num.floor() as u16;
            (u_num) - (u_num % 2) + 1
        };
        
        //set columns
        let width = node.size().x;

        let calc_cols = (width/min_size) * full_dia as f32;
        let mut num_cols = get_nearest_odd(calc_cols); //floors to nearest odd num
        let rem_cols = (calc_cols % 1.) / 2. ;

        let mut lst_cols: Vec<RepeatedGridTrack> = vec![RepeatedGridTrack::fr(num_cols.clone() ,1.)];

        if rem_cols > 0.01{
            lst_cols.insert(0, RepeatedGridTrack::fr(1, rem_cols));
            lst_cols.push(RepeatedGridTrack::fr(1, rem_cols));
            num_cols += 2;
        }

        style.grid_template_columns = lst_cols;
  
        //set rows
        let height = node.size().y;

        let calc_rows = (height/min_size) * full_dia as f32;
        let mut num_rows = get_nearest_odd(calc_rows);
        let rem_rows = (calc_rows % 1.) / 2. ;

        let mut lst_rows: Vec<RepeatedGridTrack> = vec![RepeatedGridTrack::fr(num_rows.clone() ,1.)];

        if rem_rows > 0.01{
            lst_rows.insert(0, RepeatedGridTrack::fr(1, rem_rows));
            lst_rows.push(RepeatedGridTrack::fr(1, rem_rows));
            num_rows += 2;
        }

        style.grid_template_rows = lst_rows;

        if let Some(mut ent_commands) = commands.get_entity(ent) {
            ent_commands.despawn_descendants();

            let edit_range = grid_data.edit_limit as i8;

            let split = |num: u16| -> (i8, i8) {
                let cnt = num.div_ceil(2) as i8;
                (cnt - num as i8, cnt)
            };
            let (row_start, row_end) = split(num_rows);
            let (col_start, col_end) = split(num_cols);

            println!("rows: {}, cols: {}", num_rows, num_cols);
            println!("rows: {} -> {}, cols: {} -> {}", row_start, row_end, col_start, col_end);

            ent_commands.with_children(|grid|{
                for row in row_start..row_end{
                    for col in col_start..col_end{
                        let mut node_bun = NodeBundle::default();

                        if row == 0 && col == 0{
                            node_bun.background_color = Color::WHITE.into();
                            grid.spawn((node_bun, MidSquare{piece: None}));
                        }
                        else{
                            let editable = row.abs() <= edit_range && col.abs() <= edit_range;
                            let is_dark = (row + col) % 2 != 0;

                            node_bun.background_color = if editable{
                                (if is_dark {TILEDARK} else {TILELIGHT}).into()
                            }
                            else{
                                (if is_dark {TILEDARKALT} else {TILELIGHTALT}).into()
                            };

                            grid.spawn((node_bun, GridSquare{
                                is_editable: editable,
                                is_white: !is_dark,
                                status: None,
                            }, PieceCoord::new(col, -row)));
                        }
                    }
                }
            });
        }
    }
}

pub fn despawn_design_pieces(
    mut commands: Commands,
    q_despawn: Query<Entity, With<RootNode>>,
){
    q_despawn.iter().for_each(|e| commands.entity(e).despawn_recursive())
}

pub fn handle_save_design_pieces(
    mut q_interaction: Query<(&Interaction, &mut BackgroundColor, &SaveButton),(Changed<Interaction>, With<SaveButton>)>,
){
    for (interaction, mut back_color, btn_type) in q_interaction.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                back_color.0 = MAINBTNSEL
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

