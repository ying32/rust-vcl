/*
   类型实现
*/

#![allow(non_snake_case)]

use crate::types::*;

impl TGUID {
    pub fn Empty() -> Self {
        TGUID {
            d1: 0,
            d2: 0,
            d3: 0,
            d4: [0, 0, 0, 0, 0, 0, 0, 0],
        }
    }

    pub fn From(guid: &TGUID) -> Self {
        TGUID {
            d1: guid.d1,
            d2: guid.d2,
            d3: guid.d3,
            d4: guid.d4,
        }
    }
}

impl TResItem {
    pub fn Empty() -> Self {
        TResItem { name: 0, value: 0 }
    }
}

impl TPoint {
    pub fn Empty() -> Self {
        TPoint { x: 0, y: 0 }
    }

    pub fn From(p: &TPoint) -> Self {
        TPoint { x: p.x, y: p.y }
    }
}

impl TRect {
    pub fn Empty() -> Self {
        TRect {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        }
    }

    pub fn From(r: &TRect) -> Self {
        TRect {
            left: r.left,
            top: r.top,
            right: r.right,
            bottom: r.bottom,
        }
    }
}

impl TSize {
    pub fn Empty() -> Self {
        TSize { cx: 0, cy: 0 }
    }

    pub fn From(s: &TSize) -> Self {
        TSize { cx: s.cx, cy: s.cy }
    }
}

impl TGridCoord {
    pub fn Empty() -> Self {
        TGridCoord { x: 0, y: 0 }
    }

    pub fn From(p: &TGridCoord) -> Self {
        TGridCoord { x: p.x, y: p.y }
    }
}

// impl TGridRect {
//     pub fn Empty() -> Self {
//         TGridRect {
//             left: 0,
//             top: 0,
//             right: 0,
//             bottom: 0,
//         }
//     }
//
//     pub fn From(r: &TGridRect) -> Self {
//         TGridRect {
//             left: r.left,
//             top: r.top,
//             right: r.right,
//             bottom: r.bottom,
//         }
//     }
// }
