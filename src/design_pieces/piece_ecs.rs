use bevy::prelude::{Color, Component, Resource};
use crate::rule_data::piece_set::{MoveType, PieceMoveSet, PieceType};

#[derive(Component, Debug, PartialEq, Eq)]
pub struct PieceGrid;

#[derive(Component, Debug, PartialEq, Eq)]
pub struct RightPanel;

#[derive(Resource, Debug, PartialEq, Eq)]
pub struct UiGridData{
    pub lst_moves: PieceMoveSet,
    pub piece_type: Option<PieceType>,
    pub edit_limit: u8,
}


#[derive(Resource, Debug, Default, PartialEq, Eq)]
pub struct EditType{
    pub move_type: MoveType,
    pub inf: bool,
}

#[derive(Component, Debug, Default, PartialEq, Eq)]
pub struct InfiniteButton{pub selected: bool}

impl Default for UiGridData {
    fn default() -> Self {
        Self {
            lst_moves: PieceMoveSet::default(),
            piece_type: None,
            edit_limit: 3u8
        }
    }
}

#[derive(Component ,Debug, PartialEq, Eq, Hash)]
pub struct GridSquare {
    pub is_editable: bool,
    pub is_white: bool,
    pub status: Option<MoveType>,
}

impl GridSquare{
    fn get_color(&self, default: Color) -> Color{
        let mut col = if let Some(status) = &self.status{
            match status {
                MoveType::All => Color::RED,
                MoveType::Enemies => Color::ORANGE,
                MoveType::None => Color::LIME_GREEN
            }
        }
        else {default};

        //darken
        if !self.is_editable {
            col.set_l(0.7);
        }

        col
    }
}

#[derive(Component ,Debug, PartialEq, Eq, Hash)]
pub struct MidSquare{
    pub piece: Option<PieceType>
}

#[derive(Component ,Debug, PartialEq, Eq, Hash)]
pub struct SaveButton;