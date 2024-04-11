#[allow(dead_code)]
#[allow(unused_imports)]

use bevy::render::color::Color;

const PRIMARY1:Color = Color::rgb(0.949, 0.957, 1.);
const PRIMARY2:Color = Color::rgb(0.808, 0.843, 1.);
const PRIMARY3:Color = Color::rgb(0.667, 0.745, 1.);
const PRIMARY4:Color = Color::rgb(0.522, 0.663, 1.);
const PRIMARY5:Color = Color::rgb(0.38, 0.596, 1.);
const PRIMARY6:Color = Color::rgb(0.208, 0.443, 0.788);
const PRIMARY7:Color = Color::rgb(0.118, 0.329, 0.576);
const PRIMARY8:Color = Color::rgb(0.067, 0.224, 0.361);
const PRIMARY9:Color = Color::rgb(0.027, 0.098, 0.149);

const ACCENT4:Color = Color::rgb(0.949, 0.494, 0.376);
const ACCENT5:Color = Color::rgb(0.878, 0.29, 0.184);
const ACCENT6:Color = Color::rgb(0.698, 0.125, 0.067);

const NEUTRAL1:Color = Color::rgb(0.98, 0.98, 0.988);
const NEUTRAL2:Color = Color::rgb(0.906, 0.914, 0.933);
const NEUTRAL3:Color = Color::rgb(0.835, 0.847, 0.875);
const NEUTRAL4:Color = Color::rgb(0.765, 0.78, 0.816);
const NEUTRAL5:Color = Color::rgb(0.698, 0.722, 0.761);
const NEUTRAL6:Color = Color::rgb(0.557, 0.576, 0.608);
const NEUTRAL7:Color = Color::rgb(0.416, 0.435, 0.455);
const NEUTRAL8:Color = Color::rgb(0.275, 0.29, 0.302);
const NEUTRAL9:Color = Color::rgb(0.133, 0.145, 0.149);

pub const MAINBTN:Color = PRIMARY5;
pub const MAINBTNHOV:Color = PRIMARY6;
pub const MAINBTNSEL:Color = PRIMARY7;
pub const MAINBTNTXT:Color = PRIMARY1;

pub const SECBTN:Color = PRIMARY1;
pub const SECBTNHOV:Color = PRIMARY2;
pub const SECBTNSEL:Color = PRIMARY3;
pub const SECBTNTXT:Color = NEUTRAL8;

pub const GRIDBORDER:Color = PRIMARY1;
pub const GRIDOUTER:Color = PRIMARY6;

pub const MAINBACK:Color = PRIMARY1;
pub const UIBACK:Color = PRIMARY2;

pub const TILEDARK:Color = PRIMARY4;
pub const TILELIGHT:Color = PRIMARY2;

pub const TILEDARKALT:Color = PRIMARY5;
pub const TILELIGHTALT:Color = PRIMARY3;

pub const MOVEANY:Color = Color::RED;
pub const MOVEENEMY:Color = Color::YELLOW;
pub const MOVEONLY:Color = Color::LIME_GREEN;

pub const MOVEANYHOV:Color = Color::RED;
pub const MOVEENEMYHOV:Color = Color::YELLOW;
pub const MOVEONLYHOV:Color = Color::LIME_GREEN;