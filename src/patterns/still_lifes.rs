use super::Pattern;

pub const BLOCK: Pattern = &[(0, 0), (1, 0), (0, 1), (1, 1)];

pub const BEEHIVE: Pattern = &[(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (2, 2)];

pub const LOAF: Pattern = &[(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (3, 2), (2, 3)];

pub const BOAT: Pattern = &[(0, 0), (1, 0), (0, 1), (2, 1), (1, 2)];

pub const TUB: Pattern = &[(1, 0), (0, 1), (2, 1), (1, 2)];
