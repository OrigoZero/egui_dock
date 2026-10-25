use egui::{Sense, Ui};

use crate::{dock_area::state::State, DockArea, SurfaceIndex, TabViewer};

impl<Tab> DockArea<'_, Tab> {
    pub(super) fn show_root_surface_inside(
        &mut self,
        ui: &mut Ui,
        tab_viewer: &mut impl TabViewer<Tab = Tab>,
        state: &mut State,
    ) {
        let surf_index = SurfaceIndex::main();

        if self.dock_state.main_surface().is_empty() {
            // Editor patch: an empty main surface is the *backdrop* of a
            // floating-window shell. It must NOT register itself as a
            // hover/drop target. Upstream inserts `hover_data =
            // Surface(main)` here whenever the pointer is over the empty
            // centre, which means dragging a tab onto the backdrop reads as
            // "hovering the main surface" — the drag takes the hover branch
            // in `show_inside` and docks into the centre instead of tearing
            // out. Leaving the backdrop hover-less is what makes the
            // no-hover release branch reachable: no leaf and no surface
            // claims the pointer, so `hover_data` is None and "let go over
            // empty space = new window" works. The rect is still allocated
            // so the backdrop consumes the available space.
            let rect = ui.available_rect_before_wrap();
            let _ = ui.allocate_rect(rect, Sense::hover());
            return;
        }

        self.render_nodes(ui, tab_viewer, state, surf_index, None);
    }
}
