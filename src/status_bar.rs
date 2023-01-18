use penrose::x11rb::RustConn;
use penrose_ui::{TextStyle, Position, status_bar, StatusBar};

use crate::constants::{FONT, WHITE, BLACK, BAR_HEIGHT_PX, BLUE, GREY};

pub fn build_status_bar() -> StatusBar<RustConn> {
    let style = TextStyle {
        font: FONT.to_string(),
        point_size: 11,
        fg: WHITE.into(),
        bg: Some(BLACK.into()),
        padding: (2.0, 2.0),
    };
    
    let bar = status_bar(
        BAR_HEIGHT_PX,
        &style, 
        BLUE, 
        GREY, 
        Position::Top
    ).unwrap();
    
    bar
}
