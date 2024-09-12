use iced::Point;
use xcap::image::RgbaImage;

#[derive(Debug)]
pub struct CropWindow {
    pub cursor_position: Point,
    pub image: RgbaImage,
    pub selection_area: SelectionArea,
}

#[derive(Debug, Clone)]
pub enum CropEvent {
    SetInitialPoint,
    UpdateCurrentPosition(Point),
    SetFinalPoint,
}

#[derive(Debug, Clone)]
pub struct SelectionArea {
    pub initial_pos: Option<Point>,
    pub final_pos: Option<Point>,
}