mod main_menu;
mod ui_defaults;
mod design_peices;
mod resources;
mod chess_comp;

use bevy::{prelude::*, ui::UiSystem};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use chess_comp::coordinates::PieceCoord;
use design_peices::ui::{despawn_design_pieces, handle_save_design_pieces, resize_grid, setup_design_pieces, EditType, UiGridData};
use main_menu::ui::{despawn_main_menu, handle_main_menu_ui, setup_main_menu};
use resources::piece_set::PieceMoveSet;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum WindowState {
    #[default]
    MainMenu,
    DesignPieces,
    DesignLayout,
    DesignInteraction,
    PlayGame,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum WindowSet {
    MainMenuSet,
    DesignPiecesSet,
    DesignLayoutSet,
    DesignInteractionSet,
    PlayGameSet,
}

#[derive(Component, Debug, PartialEq, Eq, Hash)]
struct MainWindow;

#[derive(Component, Debug, PartialEq, Eq, Hash)]
pub struct RootNode;

fn main(){
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_plugins(WorldInspectorPlugin::new());
    app.init_state::<WindowState>();
    app.register_type::<PieceCoord>();

    app.init_resource::<UiGridData>();
    app.init_resource::<PieceMoveSet>();
    app.init_resource::<EditType>();
    app.add_systems(Startup, setup_cam,);

    //Configure SETS for STATES
    app.configure_sets(Update, (
        WindowSet::MainMenuSet.run_if(in_state(WindowState::MainMenu)),
        WindowSet::DesignPiecesSet.run_if(in_state(WindowState::DesignPieces)),
        WindowSet::DesignLayoutSet.run_if(in_state(WindowState::DesignLayout)),
        WindowSet::DesignInteractionSet.run_if(in_state(WindowState::DesignInteraction)),
        WindowSet::PlayGameSet.run_if(in_state(WindowState::PlayGame)),
    ));

    // STATE Setup & Removal
    app.add_systems(OnEnter(WindowState::MainMenu), (
        setup_main_menu,
    ));
    app.add_systems(OnExit(WindowState::MainMenu), (
        despawn_main_menu,
    ));
    
    
    app.add_systems(OnEnter(WindowState::DesignPieces), (
        setup_design_pieces,
    ));
    app.add_systems(OnExit(WindowState::DesignPieces), (
        despawn_design_pieces,
    ));

    app.add_systems(Update, (
        //Main Menu
        (handle_main_menu_ui).in_set(WindowSet::MainMenuSet),
        (handle_save_design_pieces).in_set(WindowSet::DesignPiecesSet)
    ));

    app.add_systems(PostUpdate, (
        (resize_grid)
            .after(UiSystem::Layout)
            .in_set(WindowSet::DesignPiecesSet),
    ));


    app.run()

}   

fn setup_cam(mut commands: Commands,){
    commands.spawn(Camera2dBundle::default());
}