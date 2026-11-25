use super::*;

impl Game<'_> {
    /// `Screen.onFilesDrop` for one file dropped on the window: the open screen's mods hear it
    /// first (`screen_file_dropped`). Whether a listener took it, so the screen's own handling
    /// is skipped. False with no receiving screen, and for a path that is not valid Unicode.
    pub(in crate::game) fn screen_file_dropped(&self, path: &std::path::Path) -> bool {
        let Some(path) = path.to_str() else {
            return false;
        };
        let Some(receiver) = self.observed_input_screen() else {
            return false;
        };
        let mut canceled = false;
        self.native_screen_file_dropped(&receiver.view(), path, &mut canceled);
        canceled
    }
    /// A file was dropped on the open screen.
    pub(in crate::game) fn native_screen_file_dropped(
        &self,
        screen: &ScreenView<'_>,
        path: &str,
        canceled: &mut bool,
    ) {
        self.fire_screen_file_dropped(screen, path, canceled);
    }
}
