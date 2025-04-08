// The cameras see the cube and supply the lights

// The cameras supply a function to take pictures of the Square-1
// The cameras supply functions to get shape and color information about specified spots
// The cameras supply a function to control the lights

enum Show {
    Off,
    Startup,
    Normal,
    Fast,
    Error
}

enum Shape {
    CornerStart,
    Edge,
    CornerEnd
}

enum UDColor {
    Black,
    White
}

enum SideColor {
    Red,
    Blue,
    Orange,
    Green
}

pub(crate) struct Cameras {

}

impl Cameras {
    pub(crate) fn new() -> Self {
        Self {  }
    }

    pub(crate) fn show(show: Show) {

    }

    pub(crate) fn blink(show: Show) {

    }

    pub(crate) fn capture() {

    }

    pub(crate) fn get_shape(id: u8) -> Option<Shape> {
        None
    }

    pub(crate) fn get_udcolor(id: u8) -> Option<UDColor> {
        None
    }

    pub(crate) fn get_sidecolor(id: u8) -> Option<SideColor> {
        None
    }
}