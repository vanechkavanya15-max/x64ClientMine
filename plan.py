# Reviewing the user requirements:
# "Zaimplementuj nowy, w 100% precyzyjny modul:"
# - Plik docelowy: crates/m2-client/src/ui_modal_dialog_queue_v2.rs
# - Plik testow: crates/m2-client/tests/test_ui_modal_dialog_queue_v2.rs
# "Zaimplementuj zarzadzanie oknami modalnymi: wyswietlanie pytan potwierdzenia (Tak/Nie przy niszczeniu itemu, wychodzeniu z gry), popupy informacyjne, blokada klikniec pod oknem modalnym i powrot do poprzedniego stanu po zamknieciu."
# 
# Wait, let's look at C++ CWindowManager and CWindow (or similar).
# CWindowManager has:
# void LockWindow(CWindow * pWin);
# void UnlockWindow();
# CWindow * m_pLockWindow;
# TWindowContainer m_LockWindowList;
#
# But the prompt specifically asks for `ui_modal_dialog_queue_v2.rs`.
# Modals in typical GUI systems have a stack (or list/queue) of modal windows.
# The `PythonWindowManager` handles it using `m_pLockWindow` and `m_LockWindowList`.
# So `ui_modal_dialog_queue_v2` should probably be a Rust struct managing a stack/queue of locked windows (modals).
# We need to implement a modal window queue manager.
# Requirements:
# - lock_window(window_id)
# - unlock_window()
# - get_lock_window() -> Option<window_id>
# - A queue (or stack) of locked windows.
# - Return to previous state after closing (unlocking).
# Wait, in C++, it's done via:
# 
# void CWindowManager::LockWindow(CWindow * pWin) {
#    m_LockWindowList.remove(pWin);
#    if (m_pLockWindow) {
#       if (m_pLockWindow == pWin) return;
#       m_LockWindowList.push_back(m_pLockWindow);
#    }
#    m_pLockWindow = pWin;
# }
# 
# void CWindowManager::UnlockWindow() {
#    if (m_pLockWindow) {
#       if (m_LockWindowList.empty()) {
#          m_pLockWindow = NULL;
#       } else {
#          m_pLockWindow = m_LockWindowList.back();
#          m_LockWindowList.pop_back();
#       }
#    }
# }
# 
# This exactly matches "powrot do poprzedniego stanu po zamknieciu" (return to previous state after closing).
# "blokada klikniec pod oknem modalnym" -> The UI routing logic (which we don't fully implement here, but we provide the state for it) will check if there's a locked window. If there is, it routes inputs ONLY to the locked window.
# The prompt says: "wyswietlanie pytan potwierdzenia (Tak/Nie przy niszczeniu itemu, wychodzeniu z gry), popupy informacyjne". This implies the queue might store enums or objects representing these specific dialogs, OR it just stores Window IDs (like u32 or a generic type).
# Wait, the prompt says "Zaimplementuj zarzadzanie oknami modalnymi: wyswietlanie pytan potwierdzenia..., popupy informacyjne, blokada klikniec pod oknem...".
# This sounds like we need to define the ModalDialogQueue containing some Dialog representations.
