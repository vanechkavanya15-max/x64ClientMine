use crate::ui_window_mgr::WindowManager;
use crate::wgpu_camera_orbit_controller::CameraOrbitController;
use crate::py_event_dispatcher;

pub fn dispatch_mouse_wheel(
    ui_mgr: &mut WindowManager,
    camera: &mut CameraOrbitController,
    wparam: usize,
) -> bool {
    let delta = ((wparam >> 16) & 0xFFFF) as i16;
    let mut handled = false;

    // Check if the UI intercepts the scroll event
    if let Some(hovered) = ui_mgr.point_window {
        if let Some(win) = ui_mgr.windows.get(&hovered) {
            if win.py_handler != 0 {
                // Return true if the python callback handles it, otherwise we zoom the camera.
                // call_python_object_method1_i32 usually returns bool for handled status in these bindings?
                // Wait, it just returns true if successful. The requirement implies UI always captures if hovered, or we assume calling the callback is enough.
                // "obsluge WM_MOUSEWHEEL do regulacji dystansu kamery oraz scrollowania list okien UI"
                // If it's hovered and has py_handler, we call OnRunMouseWheel
                py_event_dispatcher::call_python_object_method1_i32(win.py_handler, "OnRunMouseWheel", delta as i32);
                handled = true;
            }
        }
    }

    if !handled {
        // Typical zoom ratios.
        // Positive delta means wheel scrolled forward (away from user), usually means zoom in.
        // Zooming in means decreasing distance.
        // Zooming out means increasing distance.
        if delta > 0 {
            camera.zoom(0.9); // zoom in (decrease distance by 10%)
        } else if delta < 0 {
            camera.zoom(1.11111); // zoom out (increase distance by ~11%)
        }
    }

    true
}
