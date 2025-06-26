// Holds information about the partpiece

use std::fmt::Display;

#[derive(Clone)]
pub enum Shape {
    CornerStart,
    Edge,
    CornerEnd
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::CornerStart => "corner start",
            Self::Edge => "edge",
            Self::CornerEnd => "corner end"
        })
    }
}

pub enum UDColor {
    Black,
    White
}

impl Display for UDColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Black => "black",
            Self::White => "white"
        })
    }
}

pub enum SideColor {
    Red,
    Blue,
    Orange,
    Green
}

impl Display for SideColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Red => "red",
            Self::Blue => "blue",
            Self::Orange => "orange",
            Self::Green => "green"
        })
    }
}

pub struct PartPiece {
    pub shape: Shape,
    pub udcolor: UDColor,
    pub sidecolor: SideColor
}

impl Display for PartPiece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}", self.shape, self.udcolor, self.sidecolor)
    }
}

impl PartPiece {

    pub fn get_adj(&self, left: bool) -> Option<Self> {
        let swap = (match self.shape {
            Shape::CornerStart => true,
            Shape::Edge => return None,
            Shape::CornerEnd => false
        } == match self.udcolor {
            UDColor::Black => true,
            UDColor::White => false
        }) == left;
        let adj_sidecolor = match self.sidecolor {
            SideColor::Red => if swap {SideColor::Blue} else {SideColor::Green},
            SideColor::Blue => if swap {SideColor::Orange} else {SideColor::Red},
            SideColor::Orange => if swap {SideColor::Green} else {SideColor::Blue},
            SideColor::Green => if swap {SideColor::Red} else {SideColor::Orange},
        };
        Some(PartPiece {
            shape: match self.shape {
                Shape::CornerStart => Shape::CornerEnd,
                Shape::Edge => return None,
                Shape::CornerEnd => Shape::CornerStart,
            },
            udcolor: match self.udcolor {
                UDColor::Black => UDColor::Black,
                UDColor::White => UDColor::White,
            },
            sidecolor: adj_sidecolor
        })
    }

    pub fn get_id(&self, left: bool) -> Option<u8> {
        match self.shape {
            Shape::CornerStart => {
                match self.udcolor {
                    UDColor::Black => {
                        match self.sidecolor {
                            SideColor::Red => if left {
                                Some(0)
                            } else {
                                Some(6)
                            },
                            SideColor::Blue => if left {
                                Some(2)
                            } else {
                                Some(0)
                            },
                            SideColor::Orange => if left {
                                Some(4)
                            } else {
                                Some(2)
                            },
                            SideColor::Green => if left {
                                Some(6)
                            } else {
                                Some(4)
                            }
                        }
                    },
                    UDColor::White => {
                        match self.sidecolor {
                            SideColor::Red => if left {
                                Some(10)
                            } else {
                                Some(12)
                            },
                            SideColor::Blue => if left {
                                Some(12)
                            } else {
                                Some(14)
                            },
                            SideColor::Orange => if left {
                                Some(14)
                            } else {
                                Some(8)
                            },
                            SideColor::Green => if left {
                                Some(8)
                            } else {
                                Some(10)
                            }
                        }
                    }
                }
            },
            Shape::Edge => {
                match self.udcolor {
                    UDColor::Black => {
                        match self.sidecolor {
                            SideColor::Red => Some(7),
                            SideColor::Blue => Some(1),
                            SideColor::Orange => Some(3),
                            SideColor::Green => Some(5),
                        }
                    },
                    UDColor::White => {
                        match self.sidecolor {
                            SideColor::Red => Some(11),
                            SideColor::Blue => Some(13),
                            SideColor::Orange => Some(15),
                            SideColor::Green => Some(9),
                        }
                    }
                }
            },
            Shape::CornerEnd => None
        }
    }
}