use std::fmt;

/// Wire protocol version negotiated during the handshake.
///
/// Peers with the same `major` interoperate; the effective feature set is the
/// minimum `minor` of both sides plus the intersection of capability bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProtocolVersion {
    pub major: u16,
    pub minor: u16,
}

/// Protocol version implemented by this build.
pub const PROTOCOL_VERSION: ProtocolVersion = ProtocolVersion { major: 0, minor: 1 };

/// Result of comparing a local and a remote protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Compatibility {
    /// Same major version; communicate using `effective`.
    Compatible { effective: ProtocolVersion },
    /// Different major versions; the connection must be refused.
    Incompatible,
}

impl ProtocolVersion {
    /// Decides whether two peers can talk and which version they should use.
    ///
    /// Before 1.0 (`major == 0`) every minor release is a breaking change, so
    /// the minor versions must match exactly.
    pub fn negotiate(self, remote: Self) -> Compatibility {
        if self.major != remote.major || (self.major == 0 && self.minor != remote.minor) {
            return Compatibility::Incompatible;
        }
        Compatibility::Compatible {
            effective: self.min(remote),
        }
    }
}

impl fmt::Display for ProtocolVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const fn v(major: u16, minor: u16) -> ProtocolVersion {
        ProtocolVersion { major, minor }
    }

    #[test]
    fn stable_majors_pick_lowest_minor() {
        assert_eq!(
            v(1, 4).negotiate(v(1, 2)),
            Compatibility::Compatible { effective: v(1, 2) }
        );
    }

    #[test]
    fn different_majors_are_incompatible() {
        assert_eq!(v(1, 0).negotiate(v(2, 0)), Compatibility::Incompatible);
    }

    #[test]
    fn pre_release_requires_exact_minor() {
        assert_eq!(v(0, 1).negotiate(v(0, 2)), Compatibility::Incompatible);
        assert_eq!(
            v(0, 1).negotiate(v(0, 1)),
            Compatibility::Compatible { effective: v(0, 1) }
        );
    }
}
