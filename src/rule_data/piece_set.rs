use bevy::{ecs::{component::Component, system::Resource}, render::color::Color, utils::HashMap};
use crate::design_pieces::piece_coords::PieceCoord;

#[derive(Resource, Default, Debug, PartialEq, Eq)]
pub struct PieceMoveSet{
    pub hm_move_data: HashMap<PieceType, Vec<PieceMoveData>>,
} 

#[derive(Debug, PartialEq, Eq)]
pub struct PieceMoveData{
    pub move_data: HashMap<PieceCoord, TileMoveData>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum PieceType {
    Pawn, Knight, Bishop, Rook, Queen, King, Elephant, Tiger, Ship
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TileMoveData{
    pub inf: bool,
    pub can_take: MoveType,
}

#[derive(Component, Debug, PartialEq, Default, Eq, Hash)]
pub enum MoveType{
    All, 
    None,
    #[default]
    Enemies, 
}

impl MoveType{
    pub fn get_color(&self, hov:bool) -> Color{
        match &self {
            Self::All => if !hov {
                Color::rgb(1., 0.4, 0.)} 
            else {
                Color::rgb(0.9, 0.3, 0.)
            },
            Self::None => if !hov {
                Color::rgb(1., 0.9, 0.25)} 
            else {
                Color::rgb(0.9, 0.8, 0.)
            },
            Self::Enemies => if !hov {
                Color::rgb(0.443, 0.902, 0.)
            } else {
                Color::rgb(0.196, 0.831, 0.)
            },
        }
    }
}