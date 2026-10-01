//! Remora wire protocol: framing, versioning and handshake.
//!
//! The normative description lives in `docs/en/protocol.md`. This crate is the
//! single source of truth for the binary layout; all integers are little-endian.

mod frame;
mod version;

pub use frame::{DecodeError, FrameFlags, FrameHeader, FrameKind, MAX_INLINE_LEN};
pub use version::{Compatibility, ProtocolVersion, PROTOCOL_VERSION};

/// Magic bytes that open every connection (`ClientHello` and `ServerHello`).
pub const MAGIC: [u8; 4] = *b"RMRA";
