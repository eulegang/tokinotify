/// A mask specifying event type interest
#[derive(Clone, Copy)]
pub struct Mask(pub(crate) u32);

impl Mask {
    // basic event masks

    /// File Accessed
    pub const ACCESS: Mask = Mask(0x00000001);

    /// File modified
    pub const MODIFY: Mask = Mask(0x00000002);

    /// Metadata changed
    pub const ATTRIB: Mask = Mask(0x00000004);

    /// Writable file was closed
    pub const CLOSE_WRITE: Mask = Mask(0x00000008);

    /// Unwritable file closed
    pub const CLOSE_NOWRITE: Mask = Mask(0x00000010);

    /// File was opened
    pub const OPEN: Mask = Mask(0x00000020);

    /// File was moved from X
    pub const MOVED_FROM: Mask = Mask(0x00000040);

    /// File was moved to Y
    pub const MOVED_TO: Mask = Mask(0x00000080);

    /// Subfile was created
    pub const CREATE: Mask = Mask(0x00000100);

    /// Subfile was deleted
    pub const DELETE: Mask = Mask(0x00000200);

    /// Self was deleted
    pub const DELETE_SELF: Mask = Mask(0x00000400);

    /// Self was moved
    pub const MOVE_SELF: Mask = Mask(0x00000800);

    // status masks

    /// Backing fs was unmounted
    pub const UNMOUNT: Mask = Mask(0x00002000);

    /// Event queued overflowed
    pub const Q_OVERFLOW: Mask = Mask(0x00004000);

    /// File was ignored
    pub const IGNORED: Mask = Mask(0x00008000);

    // helper merged flags

    /// Close
    pub const CLOSE: Mask = Mask(Self::CLOSE_WRITE.0 | Self::CLOSE_NOWRITE.0);

    /// Moves
    pub const MOVE: Mask = Mask(Self::MOVED_TO.0 | Self::MOVED_FROM.0);

    // special flaqs

    /// Only watch the path if it is a directory
    pub const ONLYDIR: Mask = Mask(0x01000000);

    /// Don't follow a sym link
    pub const DONT_FOLLOW: Mask = Mask(0x02000000);

    /// Exclude events on unlinked objects
    pub const EXCL_UNLINK: Mask = Mask(0x04000000);

    /// Only create watches
    pub const MASK_CREATE: Mask = Mask(0x10000000);

    /// Add to the mask of an already existing watch
    pub const MASK_ADD: Mask = Mask(0x20000000);

    /// Event occurred against dir
    pub const ISDIR: Mask = Mask(0x40000000);

    /// Only send event once
    pub const ONESHOT: Mask = Mask(0x80000000);

    /// test if a mask constains a submask
    pub fn contains(self, other: Mask) -> bool {
        (self & other) == other
    }
}

impl PartialEq for Mask {
    fn eq(&self, other: &Self) -> bool {
        const ALL: u32 = 0xF700EFFF;

        (self.0 & ALL) == (other.0 & ALL)
    }
}

impl std::ops::BitAnd<Mask> for Mask {
    type Output = Mask;

    fn bitand(self, rhs: Mask) -> Self::Output {
        Mask(self.0 & rhs.0)
    }
}

impl std::ops::BitAndAssign for Mask {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl std::ops::BitOr<Mask> for Mask {
    type Output = Mask;

    fn bitor(self, rhs: Mask) -> Self::Output {
        Mask(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for Mask {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

const CHECK: &[(Mask, &str)] = &[
    (Mask::ACCESS, "ACCESS"),
    (Mask::MODIFY, "MODIFY"),
    (Mask::ATTRIB, "ATTRIB"),
    (Mask::CLOSE_WRITE, "CLOSE_WRITE"),
    (Mask::CLOSE_NOWRITE, "CLOSE_NOWRITE"),
    (Mask::OPEN, "OPEN"),
    (Mask::MOVED_FROM, "MOVED_FROM"),
    (Mask::MOVED_TO, "MOVED_TO"),
    (Mask::CREATE, "CREATE"),
    (Mask::DELETE, "DELETE"),
    (Mask::DELETE_SELF, "DELETE_SELF"),
    (Mask::MOVE_SELF, "MOVE_SELF"),
    (Mask::UNMOUNT, "UNMOUNT"),
    (Mask::Q_OVERFLOW, "Q_OVERFLOW"),
    (Mask::IGNORED, "IGNORED"),
    (Mask::ONLYDIR, "ONLYDIR"),
    (Mask::DONT_FOLLOW, "DONT_FOLLOW"),
    (Mask::EXCL_UNLINK, "EXCL_UNLINK"),
    (Mask::MASK_CREATE, "MASK_CREATE"),
    (Mask::MASK_ADD, "MASK_ADD"),
    (Mask::ISDIR, "ISDIR"),
    (Mask::ONESHOT, "ONESHOT"),
];

impl std::fmt::Debug for Mask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;

        write!(f, "({:X}) ", self.0)?;

        for (mask, repr) in CHECK {
            if (*self & *mask).0 != 0 {
                if !first {
                    write!(f, " | ")?;
                } else {
                    first = false;
                }

                write!(f, "{}", repr)?;
            }
        }

        Ok(())
    }
}
