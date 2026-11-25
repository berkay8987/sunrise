//! The `screen_file_dropped` hook: declarations the mod API generator reads (see `hooks/mod.rs`).

pub trait Hooks {
    /// A file or folder from the operating system was dropped onto the game window while a
    /// screen is open (vanilla's `Screen.onFilesDrop`, which `MouseHandler.onDrop` calls): a mod
    /// screen imports what was dropped on it (scripts, schematics, presets, media). `screen` is
    /// the screen open then, the same receiving screens as `screen_key_pressed_pre`, and `path`
    /// the dropped file's absolute path as the operating system gave it. Set `canceled` and the
    /// screen's own handling of that file does not run: the resource pack screen and the
    /// world creation screen copy a dropped pack in.
    ///
    /// It fires once per file, as the window system reports each dropped file on its own,
    /// where vanilla hands a screen one list. With no screen open it does not fire. A path that
    /// is not valid Unicode does not fire it, and the screen handles that file as before. The
    /// listener runs on the client's main thread: read a large file elsewhere.
    #[hook(
        target = "sunrise_client::game::native_hooks::screen_file_dropped::Game<'_>::native_screen_file_dropped",
        receiver = "read",
        dispatch = "flat",
        side = "client",
        contract = "cancel",
        out = "canceled",
        cancellable = true
    )]
    fn screen_file_dropped(screen: &ScreenView<'_>, path: &str, canceled: &mut bool);
}
