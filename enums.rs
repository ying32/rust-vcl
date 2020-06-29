#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#[repr(C)]
pub enum TPosition {
    poDesigned,        // use bounds from the designer (read from stream)
    poDefault,         // LCL decision (normally window manager decides)
    poDefaultPosOnly,  // designed size and LCL position
    poDefaultSizeOnly, // designed position and LCL size
    poScreenCenter,    // center form on screen (depends on DefaultMonitor)
    poDesktopCenter,   // center form on desktop (total of all screens)
    poMainFormCenter,  // center form on main form (depends on DefaultMonitor)
    poOwnerFormCenter, // center form on owner form (depends on DefaultMonitor)
    poWorkAreaCenter,  // center form on working area (depends on DefaultMonitor)
}
