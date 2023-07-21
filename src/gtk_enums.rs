
///
/// Application signal enums
/// 
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ApplicationConnections
{
    Activate, //https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/struct.Application.html
    CommandLine,
    HandleLocalOptions,
    NameLost,
    Shutdown,
    Startup,
    ActionGroupNotify,
    ApplicationIdNotify,
    FlagsNotify,
    InactivityTimeoutNotify,
    IsBusyNotify,
    IsRegisteredNotify,
    IsRemoteNotify,
    ResourceBasePathNotify,
    QueryEnd, //https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/prelude/trait.GtkApplicationExt.html
    WindowAdded,
    WindowRemoved,
    ActiveWindowNotify,
    MenubarNotify,
    RegisterSessionNotify,
    ScreensaverActiveNotify,

}

///
/// Box signal enums
/// 
pub enum BoxConnections
{

    BaselinePositionNotify, //https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/prelude/trait.BoxExt.html
    HomogeneousNotify,
    SpacingNotify,
    Destroy,   //https://gtk-rs.org/gtk4-rs/stable/0.4/docs/gtk4/prelude/trait.WidgetExt.html
    DirectionChanged,
    Hide,
    KeynavFailed,
    Map,
    MnemonicActivate,
    MoveFocus,
    QueryTooltip,
    Realize,
    Show,
    StateFlagsChanged,
    Unmap,
    Unrealize,
    CanTargetNotify,
    CssClassesNotify,
    CursorNotify,
    FocusOnClickNotify,
    FocusableNotify,
    HalignNotify,
    HasDefaultNotify,
    HasFocusNotify,
    HasTooltipNotify,
    HeightRequestNotify,
    HexpandNotify,
    HexpandSetNotify,
    LayoutManagerNotify,
    MarginBottomNotify,
    MarginEndNotify,
    MarginStartNotify,
    MarginTopNotify,
    NameNotify,
    OpacityNotify,
    OverflowNotify,
    ParentNotify,
    ReceivesDefaultNotify,
    RootNotify,
    ScaleFactorNotify,
    SensitiveNotify,
    TooltipMarkupNotify,
    TooltipTextNotify,
    ValignNotify,
    VexpandNotify,
    VexpandSetNotify,
    VisibleNotify,
    WidthRequestNotify

}