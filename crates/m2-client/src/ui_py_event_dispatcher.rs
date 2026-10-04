use pyo3::prelude::*;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

/// Represents the different types of UI events that can be dispatched to Python.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PyUiEventType {
    Update,
    Render,
    Press,
    Click,
    MouseOver,
    MouseOut,
    KeyDown(u32),
}

/// A dispatcher that queues UI events and dispatches them to Python UI window handlers.
/// Uses a thread-safe queue to avoid deadlocks, e.g., when called from the WGPU render thread.
#[derive(Clone)]
pub struct UiPyEventDispatcher {
    queue: Arc<Mutex<VecDeque<(Py<PyAny>, PyUiEventType)>>>,
}

impl UiPyEventDispatcher {
    /// Creates a new, empty event dispatcher.
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    /// Queues an event to be dispatched to a Python handler.
    pub fn queue_event(&self, handler: Py<PyAny>, event_type: PyUiEventType) {
        if let Ok(mut queue) = self.queue.lock() {
            queue.push_back((handler, event_type));
        }
    }

    /// Retrieves the current length of the queue.
    pub fn queue_len(&self) -> usize {
        if let Ok(queue) = self.queue.lock() {
            queue.len()
        } else {
            0
        }
    }

    /// Clears the event queue without processing.
    pub fn clear(&self) {
        if let Ok(mut queue) = self.queue.lock() {
            queue.clear();
        }
    }

    /// Processes the queued events, dispatching them to the respective Python handlers.
    /// This method drains the queue to ensure all queued events are processed.
    pub fn process_queue(&self, py: Python<'_>) {
        let events = {
            if let Ok(mut queue) = self.queue.lock() {
                // Drain the queue to a local vector to minimize lock contention and avoid deadlocks
                // during Python execution if Python callbacks try to queue new events.
                queue.drain(..).collect::<Vec<_>>()
            } else {
                return;
            }
        };

        for (handler, event_type) in events {
            let method_name = match event_type {
                PyUiEventType::Update => "OnUpdate",
                PyUiEventType::Render => "OnRender",
                PyUiEventType::Press => "OnPress",
                PyUiEventType::Click => "OnClick",
                PyUiEventType::MouseOver => "OnMouseOver",
                PyUiEventType::MouseOut => "OnMouseOut",
                PyUiEventType::KeyDown(_) => "OnKeyDown",
            };

            // Call the appropriate method, safely ignoring errors if the method doesn't exist
            // or if an exception is raised within the python code.
            let result = match event_type {
                PyUiEventType::KeyDown(key) => {
                    handler.bind(py).call_method1(method_name, (key,))
                }
                _ => {
                    handler.bind(py).call_method0(method_name)
                }
            };

            if let Err(e) = result {
                // Ignore standard AttributeError (method not found), it's common for handlers
                // to not implement all callbacks.
                if !e.is_instance_of::<pyo3::exceptions::PyAttributeError>(py) {
                    // For other errors, we could log them, but to prevent crashing
                    // we do nothing and ignore it.
                }
            }
        }
    }
}

impl Default for UiPyEventDispatcher {
    fn default() -> Self {
        Self::new()
    }
}
