use m2_client::ui_py_event_dispatcher::{PyUiEventType, UiPyEventDispatcher};
use pyo3::prelude::*;
use std::sync::{Arc, Mutex};

/// A mock Python class used for testing the dispatcher callbacks.
#[pyclass]
struct MockWindow {
    pub update_called: Arc<Mutex<bool>>,
    pub render_called: Arc<Mutex<bool>>,
    pub key_down_called: Arc<Mutex<Option<u32>>>,
}

#[pymethods]
impl MockWindow {
    fn OnUpdate(&self) {
        if let Ok(mut called) = self.update_called.lock() {
            *called = true;
        }
    }

    fn OnRender(&self) {
        if let Ok(mut called) = self.render_called.lock() {
            *called = true;
        }
    }

    fn OnKeyDown(&self, key: u32) {
        if let Ok(mut called) = self.key_down_called.lock() {
            *called = Some(key);
        }
    }
}

fn init_python() {
    pyo3::prepare_freethreaded_python();
}

#[test]
fn test_dispatcher_initialization() {
    let dispatcher = UiPyEventDispatcher::new();
    assert_eq!(dispatcher.queue_len(), 0);
}

#[test]
fn test_dispatcher_queue_and_clear() {
    init_python();

    Python::with_gil(|py| {
        let dispatcher = UiPyEventDispatcher::new();

        let mock_window = Bound::new(
            py,
            MockWindow {
                update_called: Arc::new(Mutex::new(false)),
                render_called: Arc::new(Mutex::new(false)),
                key_down_called: Arc::new(Mutex::new(None)),
            },
        )
        .unwrap()
        .unbind();

        dispatcher.queue_event(mock_window.clone_ref(py).into_any(), PyUiEventType::Update);
        dispatcher.queue_event(mock_window.clone_ref(py).into_any(), PyUiEventType::Render);

        assert_eq!(dispatcher.queue_len(), 2);

        dispatcher.clear();

        assert_eq!(dispatcher.queue_len(), 0);
    });
}

#[test]
fn test_dispatcher_process_queue() {
    init_python();

    Python::with_gil(|py| {
        let dispatcher = UiPyEventDispatcher::new();

        let update_called = Arc::new(Mutex::new(false));
        let render_called = Arc::new(Mutex::new(false));
        let key_down_called = Arc::new(Mutex::new(None));

        let mock_window = Bound::new(
            py,
            MockWindow {
                update_called: update_called.clone(),
                render_called: render_called.clone(),
                key_down_called: key_down_called.clone(),
            },
        )
        .unwrap()
        .unbind();

        dispatcher.queue_event(mock_window.clone_ref(py).into_any(), PyUiEventType::Update);
        dispatcher.queue_event(mock_window.clone_ref(py).into_any(), PyUiEventType::KeyDown(42));

        assert_eq!(dispatcher.queue_len(), 2);

        dispatcher.process_queue(py);

        assert_eq!(dispatcher.queue_len(), 0);
        assert_eq!(*update_called.lock().unwrap(), true);
        assert_eq!(*render_called.lock().unwrap(), false); // Render was not queued
        assert_eq!(*key_down_called.lock().unwrap(), Some(42));
    });
}

#[test]
fn test_dispatcher_missing_method_does_not_panic() {
    init_python();

    Python::with_gil(|py| {
        let dispatcher = UiPyEventDispatcher::new();

        let update_called = Arc::new(Mutex::new(false));
        let render_called = Arc::new(Mutex::new(false));
        let key_down_called = Arc::new(Mutex::new(None));

        let mock_window = Bound::new(
            py,
            MockWindow {
                update_called: update_called.clone(),
                render_called: render_called.clone(),
                key_down_called: key_down_called.clone(),
            },
        )
        .unwrap()
        .unbind();

        // Queue an event for a method that doesn't exist on MockWindow (e.g., OnClick)
        dispatcher.queue_event(mock_window.clone_ref(py).into_any(), PyUiEventType::Click);

        // Process queue shouldn't panic
        dispatcher.process_queue(py);

        assert_eq!(dispatcher.queue_len(), 0);
    });
}
