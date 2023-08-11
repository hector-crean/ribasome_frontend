#[derive(Clone, Copy, Debug)]
pub struct Icon {
    /// Human readable unique id
    pub id: &'static str,

    pub png_bytes: &'static [u8],
}

impl Icon {
    pub const fn new(id: &'static str, png_bytes: &'static [u8]) -> Self {
        Self { id, png_bytes }
    }
}

pub const RERUN_MENU: Icon = Icon::new(
    "rerun_menu",
    include_bytes!("../assets/icons/rerun_menu.png"),
);

pub const RERUN_IO_TEXT: Icon =
    Icon::new("rerun_io", include_bytes!("../assets/icons/rerun_io.png"));

pub const PLAY: Icon = Icon::new("play", include_bytes!("../assets/icons/play.png"));
pub const FOLLOW: Icon = Icon::new("follow", include_bytes!("../assets/icons/follow.png"));
pub const PAUSE: Icon = Icon::new("pause", include_bytes!("../assets/icons/pause.png"));
pub const ARROW_LEFT: Icon = Icon::new(
    "arrow_left",
    include_bytes!("../assets/icons/arrow_left.png"),
);
pub const ARROW_RIGHT: Icon = Icon::new(
    "arrow_right",
    include_bytes!("../assets/icons/arrow_right.png"),
);
pub const LOOP: Icon = Icon::new("loop", include_bytes!("../assets/icons/loop.png"));

pub const RIGHT_PANEL_TOGGLE: Icon = Icon::new(
    "right_panel_toggle",
    include_bytes!("../assets/icons/right_panel_toggle.png"),
);
pub const BOTTOM_PANEL_TOGGLE: Icon = Icon::new(
    "bottom_panel_toggle",
    include_bytes!("../assets/icons/bottom_panel_toggle.png"),
);
pub const LEFT_PANEL_TOGGLE: Icon = Icon::new(
    "left_panel_toggle",
    include_bytes!("../assets/icons/left_panel_toggle.png"),
);

pub const MINIMIZE: Icon = Icon::new("minimize", include_bytes!("../assets/icons/minimize.png"));
pub const MAXIMIZE: Icon = Icon::new("maximize", include_bytes!("../assets/icons/maximize.png"));

pub const VISIBLE: Icon = Icon::new("visible", include_bytes!("../assets/icons/visible.png"));
pub const INVISIBLE: Icon = Icon::new("invisible", include_bytes!("../assets/icons/invisible.png"));

pub const ADD: Icon = Icon::new("add", include_bytes!("../assets/icons/add.png"));
pub const REMOVE: Icon = Icon::new("remove", include_bytes!("../assets/icons/remove.png"));

pub const RESET: Icon = Icon::new("reset", include_bytes!("../assets/icons/reset.png"));

pub const CLOSE: Icon = Icon::new("close", include_bytes!("../assets/icons/close.png"));

pub const SPACE_VIEW_TEXT: Icon = Icon::new(
    "spaceview_text",
    include_bytes!("../assets/icons/spaceview_text.png"),
);
// TODO(jleibs): Differentiate icon?
pub const SPACE_VIEW_TEXTBOX: Icon = Icon::new(
    "spaceview_text",
    include_bytes!("../assets/icons/spaceview_text.png"),
);
pub const SPACE_VIEW_2D: Icon = Icon::new(
    "spaceview_2d",
    include_bytes!("../assets/icons/spaceview_2d.png"),
);
pub const SPACE_VIEW_3D: Icon = Icon::new(
    "spaceview_3d",
    include_bytes!("../assets/icons/spaceview_3d.png"),
);
pub const SPACE_VIEW_CHART: Icon = Icon::new(
    "spaceview_chart",
    include_bytes!("../assets/icons/spaceview_chart.png"),
);
pub const SPACE_VIEW_SCATTERPLOT: Icon = Icon::new(
    "spaceview_scatterplot",
    include_bytes!("../assets/icons/spaceview_scatterplot.png"),
);
pub const SPACE_VIEW_RAW: Icon = Icon::new(
    "spaceview_raw",
    include_bytes!("../assets/icons/spaceview_raw.png"),
);
pub const SPACE_VIEW_TENSOR: Icon = Icon::new(
    "spaceview_tensor",
    include_bytes!("../assets/icons/spaceview_tensor.png"),
);
pub const SPACE_VIEW_HISTOGRAM: Icon = Icon::new(
    "spaceview_histogram",
    include_bytes!("../assets/icons/spaceview_histogram.png"),
);
pub const SPACE_VIEW_UNKNOWN: Icon = Icon::new(
    "spaceview_unknown",
    include_bytes!("../assets/icons/spaceview_unknown.png"),
);

pub const CONTAINER: Icon = Icon::new("container", include_bytes!("../assets/icons/container.png"));
