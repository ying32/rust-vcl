#[repr(C)]
pub enum TPosition {
    PoDesigned,        // use bounds from the designer (read from stream)
    PoDefault,         // LCL decision (normally window manager decides)
    PoDefaultPosOnly,  // designed size and LCL position
    PoDefaultSizeOnly, // designed position and LCL size
    PoScreenCenter,    // center form on screen (depends on DefaultMonitor)
    PoDesktopCenter,   // center form on desktop (total of all screens)
    PoMainFormCenter,  // center form on main form (depends on DefaultMonitor)
    PoOwnerFormCenter, // center form on owner form (depends on DefaultMonitor)
    PoWorkAreaCenter,  // center form on working area (depends on DefaultMonitor)
}
