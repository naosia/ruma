//! Modules for events in the *m.call* namespace.
//!
//! This module also contains types shared by events in its child namespaces.

pub mod answer;
pub mod candidates;
pub mod hangup;
pub mod invite;

/// A VoIP session description.
pub struct SessionDescription {
    /// The type of session description.
    pub session_type: SessionDescriptionType,
    /// The SDP text of the session description.
    pub sdp: String,
}

/// The type of VoIP session description.
pub enum SessionDescriptionType {
    /// An answer.
    Answer,
    /// An offer.
    Offer,
}
