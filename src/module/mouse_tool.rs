use ratatui::layout::Rect;
use ratatui::prelude::Color;
use std::cell::Ref;

//아래 데이터는 근사치 이며 이후 프레임워크 업데이트에 따라 교체되거나 삭제될 코드.
static X_SYNC: u16 = 10;
static Y_SYNC: u16 = 22;

pub fn is_rects_hovered(layout: Rect, mouse_pos: (u32, u32)) -> bool {
    let pos_x_min = layout.x * X_SYNC;
    let pos_x_max = layout.x + layout.width * X_SYNC;
    let pos_y_min = layout.y * Y_SYNC;
    let pos_y_max = layout.y + layout.height * Y_SYNC;
    if ((mouse_pos.0 as u16) >= pos_x_min)
        && ((mouse_pos.0 as u16) <= pos_x_max)
        && ((mouse_pos.1 as u16) >= pos_y_min)
        && ((mouse_pos.1 as u16) <= pos_y_max)
    {
        true
    } else {
        false
    }
}
