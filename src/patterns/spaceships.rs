use super::Pattern;

pub const GLIDER: Pattern = &[(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)];

pub const LWSS: Pattern = &[
    (1, 0),
    (4, 0),
    (0, 1),
    (0, 2),
    (4, 2),
    (0, 3),
    (1, 3),
    (2, 3),
    (3, 3),
];
