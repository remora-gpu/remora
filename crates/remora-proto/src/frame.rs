use std::fmt;

/// Maximum size of the inline (structured) part of a frame. Bulk data such as
/// memory copies travels in the separate bulk section, chunked by the sender.
pub const MAX_INLINE_LEN: u32 = 16 * 1024 * 1024;

/// Kind of a frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum FrameKind {
    /// Client → server API call.
    Request = 1,
    /// Server → client reply to a `Request`.
    Response = 2,
    /// Server → client unsolicited notification (stream callback, async error).
    Event = 3,
    /// Connection management: handshake, ping, flow control, shutdown.
    Control = 4,
}

impl FrameKind {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::Request),
            2 => Some(Self::Response),
            3 => Some(Self::Event),
            4 => Some(Self::Control),
            _ => None,
        }
    }
}

/// Bit flags carried by every frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct FrameFlags(u8);

impl FrameFlags {
    /// The client does not wait for a `Response`; errors are reported later.
    pub const ASYNC: Self = Self(1 << 0);
    /// The bulk section is compressed (algorithm negotiated in the handshake).
    pub const COMPRESSED: Self = Self(1 << 1);
    /// More frames of the same batch follow; the server may defer execution.
    pub const BATCH_CONTINUES: Self = Self(1 << 2);
    /// This frame is a continuation chunk of a previous bulk transfer.
    pub const BULK_CONTINUATION: Self = Self(1 << 3);

    const KNOWN: u8 = 0b1111;

    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn bits(self) -> u8 {
        self.0
    }

    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

/// Fixed 24-byte header preceding every frame.
///
/// | offset | size | field        |
/// |-------:|-----:|--------------|
/// | 0      | 4    | `inline_len` |
/// | 4      | 1    | `kind`       |
/// | 5      | 1    | `flags`      |
/// | 6      | 2    | `channel`    |
/// | 8      | 4    | `method`     |
/// | 12     | 4    | `bulk_len`   |
/// | 16     | 8    | `request_id` |
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FrameHeader {
    pub inline_len: u32,
    pub kind: FrameKind,
    pub flags: FrameFlags,
    /// Logical channel, e.g. the CUDA stream a call is ordered on.
    pub channel: u16,
    /// API method identifier: `(api_family << 16) | index`, assigned by codegen.
    pub method: u32,
    pub bulk_len: u32,
    /// Monotonic per-connection identifier; responses echo the request's id.
    pub request_id: u64,
}

/// Error returned when a header cannot be decoded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    Truncated { needed: usize, got: usize },
    UnknownKind(u8),
    UnknownFlags(u8),
    InlineTooLarge(u32),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Truncated { needed, got } => {
                write!(f, "frame header truncated: need {needed} bytes, got {got}")
            }
            Self::UnknownKind(kind) => write!(f, "unknown frame kind {kind}"),
            Self::UnknownFlags(flags) => write!(f, "unknown frame flags {flags:#010b}"),
            Self::InlineTooLarge(len) => {
                write!(f, "inline payload of {len} bytes exceeds {MAX_INLINE_LEN}")
            }
        }
    }
}

impl std::error::Error for DecodeError {}

impl FrameHeader {
    pub const LEN: usize = 24;

    pub fn encode(&self) -> [u8; Self::LEN] {
        let mut out = [0u8; Self::LEN];
        out[0..4].copy_from_slice(&self.inline_len.to_le_bytes());
        out[4] = self.kind as u8;
        out[5] = self.flags.bits();
        out[6..8].copy_from_slice(&self.channel.to_le_bytes());
        out[8..12].copy_from_slice(&self.method.to_le_bytes());
        out[12..16].copy_from_slice(&self.bulk_len.to_le_bytes());
        out[16..24].copy_from_slice(&self.request_id.to_le_bytes());
        out
    }

    /// Decodes and validates a header. The server never trusts the peer, so
    /// unknown kinds, unknown flags and oversized payloads are rejected here.
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        let Some(bytes) = bytes.first_chunk::<{ Self::LEN }>() else {
            return Err(DecodeError::Truncated {
                needed: Self::LEN,
                got: bytes.len(),
            });
        };
        let u32_at =
            |i: usize| u32::from_le_bytes([bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]]);

        let inline_len = u32_at(0);
        if inline_len > MAX_INLINE_LEN {
            return Err(DecodeError::InlineTooLarge(inline_len));
        }
        let kind = FrameKind::from_u8(bytes[4]).ok_or(DecodeError::UnknownKind(bytes[4]))?;
        if bytes[5] & !FrameFlags::KNOWN != 0 {
            return Err(DecodeError::UnknownFlags(bytes[5]));
        }
        let mut request_id = [0u8; 8];
        request_id.copy_from_slice(&bytes[16..24]);

        Ok(Self {
            inline_len,
            kind,
            flags: FrameFlags(bytes[5]),
            channel: u16::from_le_bytes([bytes[6], bytes[7]]),
            method: u32_at(8),
            bulk_len: u32_at(12),
            request_id: u64::from_le_bytes(request_id),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> FrameHeader {
        FrameHeader {
            inline_len: 128,
            kind: FrameKind::Request,
            flags: FrameFlags::ASYNC.union(FrameFlags::BATCH_CONTINUES),
            channel: 7,
            method: 0x0001_002a,
            bulk_len: 4096,
            request_id: 0x0102_0304_0506_0708,
        }
    }

    #[test]
    fn roundtrip() {
        let header = sample();
        assert_eq!(FrameHeader::decode(&header.encode()), Ok(header));
    }

    #[test]
    fn layout_is_little_endian() {
        let bytes = sample().encode();
        assert_eq!(&bytes[0..4], &[128, 0, 0, 0]);
        assert_eq!(bytes[4], 1);
        assert_eq!(bytes[5], 0b101);
        assert_eq!(&bytes[16..24], &[8, 7, 6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn rejects_truncated() {
        assert_eq!(
            FrameHeader::decode(&[0; 10]),
            Err(DecodeError::Truncated {
                needed: 24,
                got: 10
            })
        );
    }

    #[test]
    fn rejects_unknown_kind_and_flags() {
        let mut bytes = sample().encode();
        bytes[4] = 99;
        assert_eq!(
            FrameHeader::decode(&bytes),
            Err(DecodeError::UnknownKind(99))
        );

        let mut bytes = sample().encode();
        bytes[5] = 0x80;
        assert_eq!(
            FrameHeader::decode(&bytes),
            Err(DecodeError::UnknownFlags(0x80))
        );
    }

    #[test]
    fn rejects_oversized_inline() {
        let mut header = sample();
        header.inline_len = MAX_INLINE_LEN + 1;
        assert_eq!(
            FrameHeader::decode(&header.encode()),
            Err(DecodeError::InlineTooLarge(MAX_INLINE_LEN + 1))
        );
    }
}
