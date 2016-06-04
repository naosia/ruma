//! Types for the *m.room.avatar* event.

use core::EventType;

/// A redaction of an event.
pub struct RedactionEvent {
    pub content: RedactionEventContent,
    pub event_id: String,
    pub event_type: EventType,
    /// The ID of the event that was redacted.
    pub redacts: String,
    pub room_id: String,
    pub user_id: String,
}

/// The payload of a `RedactionEvent`.
pub struct RedactionEventContent {
    /// The reason for the redaction, if any.
    pub reason: Option<String>,
}
