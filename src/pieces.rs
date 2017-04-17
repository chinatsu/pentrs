use rand::distributions::{Weighted};
use piece::*;

pub const PIECE_I: [[Point; 5]; 4] = [
    [
        Point{x: -2.0, y: 0.0},
        Point{x: -1.0, y: 0.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 1.0, y: 0.0},
        Point{x: 2.0, y: 0.0}
    ],
    [
        Point{x: 0.0, y: -2.0},
        Point{x: 0.0, y: -1.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 0.0, y: 1.0},
        Point{x: 0.0, y: 2.0}
    ],
    [
        Point{x: -2.0, y: 1.0},
        Point{x: -1.0, y: 1.0},
        Point{x: 0.0, y: 1.0},
        Point{x: 1.0, y: 1.0},
        Point{x: 2.0, y: 1.0}
    ],
    [
        Point{x: 1.0, y: -2.0},
        Point{x: 1.0, y: -1.0},
        Point{x: 1.0, y: 0.0},
        Point{x: 1.0, y: 1.0},
        Point{x: 1.0, y: 2.0}
    ]
];

pub const PIECE_J: [[Point; 5]; 4] = [
    [
        Point{x: -2.0, y: 0.0},
        Point{x: -1.0, y: 0.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 1.0, y: 0.0},
        Point{x: 1.0, y: 1.0}
    ],
    [
        Point{x: 0.0, y: -2.0},
        Point{x: 0.0, y: -1.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 0.0, y: 1.0},
        Point{x: -1.0, y: 1.0}
    ],
    [
        Point{x: -2.0, y: 0.0},
        Point{x: -2.0, y: 1.0},
        Point{x: -1.0, y: 1.0},
        Point{x: 0.0, y: 1.0},
        Point{x: 1.0, y: 1.0}
    ],
    [
        Point{x: 1.0, y: -2.0},
        Point{x: 0.0, y: -2.0},
        Point{x: 0.0, y: -1.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 0.0, y: 1.0}
    ]
];

pub const PIECE_L: [[Point; 5]; 4] = [
    [
        Point{x: -2.0, y: 1.0},
        Point{x: -2.0, y: 0.0},
        Point{x: -1.0, y: 0.0},
        Point{x: -0.0, y: 0.0},
        Point{x: 1.0, y: 0.0}
    ],
    [
        Point{x: -2.0, y: -2.0},
        Point{x: -1.0, y: -2.0},
        Point{x: -1.0, y: -1.0},
        Point{x: -1.0, y: 0.0},
        Point{x: -1.0, y: 1.0}
    ],
    [
        Point{x: 1.0, y: 0.0},
        Point{x: 1.0, y: 1.0},
        Point{x: 0.0, y: 1.0},
        Point{x: -1.0, y: 1.0},
        Point{x: -2.0, y: 1.0}
    ],
    [
        Point{x: -1.0, y: -2.0},
        Point{x: -1.0, y: -1.0},
        Point{x: -1.0, y: 0.0},
        Point{x: -1.0, y: 1.0},
        Point{x: 0.0, y: 1.0}
    ]
];

pub const PIECE_X: [[Point; 5]; 4] = [
    [
        Point{x: -1.0, y: 0.0},
        Point{x: 0.0, y: -1.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 0.0, y: 1.0},
        Point{x: 1.0, y: 0.0}
    ],
    [
        Point{x: 0.0, y: 0.0},
        Point{x: 1.0, y: -1.0},
        Point{x: 1.0, y: 0.0},
        Point{x: 1.0, y: 1.0},
        Point{x: 2.0, y: 0.0}
    ],
    [
        Point{x: 0.0, y: 1.0},
        Point{x: 1.0, y: 0.0},
        Point{x: 1.0, y: 1.0},
        Point{x: 1.0, y: 2.0},
        Point{x: 2.0, y: 1.0}
    ],
    [
        Point{x: -1.0, y: 1.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 0.0, y: 1.0},
        Point{x: 0.0, y: 2.0},
        Point{x: 1.0, y: 1.0}
    ]
];

pub const PIECE_S: [[Point; 5]; 4] = [
    [
        Point{x: -2.0, y: -1.0},
        Point{x: -2.0, y: 0.0},
        Point{x: -1.0, y: 0.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 0.0, y: 1.0}
    ],
    [
        Point{x: -2.0, y: 1.0},
        Point{x: -1.0, y: 1.0},
        Point{x: -1.0, y: 0.0},
        Point{x: -1.0, y: -1.0},
        Point{x: 0.0, y: -1.0}
    ],
    [
        Point{x: -2.0, y: -1.0},
        Point{x: -2.0, y: 0.0},
        Point{x: -1.0, y: 0.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 0.0, y: 1.0}
    ],
    [
        Point{x: -2.0, y: 1.0},
        Point{x: -1.0, y: 1.0},
        Point{x: -1.0, y: 0.0},
        Point{x: -1.0, y: -1.0},
        Point{x: 0.0, y: -1.0}
    ]
];

pub const PIECE_Z: [[Point; 5]; 4] = [
    [
        Point{x: -2.0, y: 1.0},
        Point{x: -2.0, y: 0.0},
        Point{x: -1.0, y: 0.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 0.0, y: -1.0}
    ],
    [
        Point{x: 0.0, y: 1.0},
        Point{x: -1.0, y: 1.0},
        Point{x: -1.0, y: 0.0},
        Point{x: -1.0, y: -1.0},
        Point{x: -2.0, y: -1.0}
    ],
    [
        Point{x: -2.0, y: 1.0},
        Point{x: -2.0, y: 0.0},
        Point{x: -1.0, y: 0.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 0.0, y: -1.0}
    ],
    [
        Point{x: 0.0, y: 1.0},
        Point{x: -1.0, y: 1.0},
        Point{x: -1.0, y: 0.0},
        Point{x: -1.0, y: -1.0},
        Point{x: -2.0, y: -1.0}
    ]
];

pub const PIECE_N: [[Point; 5]; 4] = [
    [
        Point{x: -2.0, y: 0.0},
        Point{x: -1.0, y: 0.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 0.0, y: 1.0},
        Point{x: 1.0, y: 1.0},
    ],
    [
        Point{x: 0.0, y: -2.0},
        Point{x: 0.0, y: -1.0},
        Point{x: 0.0, y: 0.0},
        Point{x: -1.0, y: 0.0},
        Point{x: -1.0, y: 1.0},
    ],
    [
        Point{x: -2.0, y: 0.0},
        Point{x: -1.0, y: 0.0},
        Point{x: -1.0, y: 1.0},
        Point{x: 0.0, y: 1.0},
        Point{x: 1.0, y: 1.0},
    ],
    [
        Point{x: 0.0, y: -2.0},
        Point{x: 0.0, y: -1.0},
        Point{x: -1.0, y: -1.0},
        Point{x: -1.0, y: 0.0},
        Point{x: -1.0, y: 1.0},
    ]
];

pub const PIECE_G: [[Point; 5]; 4] = [
    [
        Point{x: -2.0, y: 1.0},
        Point{x: -1.0, y: 1.0},
        Point{x: -1.0, y: 0.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 1.0, y: 0.0},
    ],
    [
        Point{x: -1.0, y: -2.0},
        Point{x: -1.0, y: -1.0},
        Point{x: 0.0, y: -1.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 0.0, y: 1.0},
    ],
    [
        Point{x: -2.0, y: 1.0},
        Point{x: -1.0, y: 1.0},
        Point{x: 0.0, y: 1.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 1.0, y: 0.0},
    ],
    [
        Point{x: -1.0, y: -2.0},
        Point{x: -1.0, y: -1.0},
        Point{x: -1.0, y: 0.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 0.0, y: 1.0},
    ]
];

pub const PIECE_U: [[Point; 5]; 4] = [
    [
        Point{x: -2.0, y: 1.0},
        Point{x: -2.0, y: 0.0},
        Point{x: -1.0, y: 0.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 0.0, y: 1.0}
    ],
    [
        Point{x: -2.0, y: -1.0},
        Point{x: -1.0, y: -1.0},
        Point{x: -1.0, y: 0.0},
        Point{x: -1.0, y: 1.0},
        Point{x: -2.0, y: 1.0}
    ],
    [
        Point{x: -2.0, y: 2.0},
        Point{x: -2.0, y: 1.0},
        Point{x: -1.0, y: 2.0},
        Point{x: 0.0, y: 1.0},
        Point{x: 0.0, y: 2.0}
    ],
    [
        Point{x: -1.0, y: 0.0},
        Point{x: -2.0, y: 0.0},
        Point{x: -2.0, y: 1.0},
        Point{x: -2.0, y: 2.0},
        Point{x: -1.0, y: 2.0}
    ]
];

pub const PIECE_T: [[Point; 5]; 4] = [
    [
        Point{x: -1.0, y: -1.0},
        Point{x: 0.0, y: -1.0},
        Point{x: 1.0, y: -1.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 0.0, y: 1.0}
    ],
    [
        Point{x: -1.0, y: 0.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 1.0, y: 0.0},
        Point{x: 1.0, y: -1.0},
        Point{x: 1.0, y: 1.0}
    ],
    [
        Point{x: -1.0, y: 1.0},
        Point{x: 0.0, y: 1.0},
        Point{x: 1.0, y: 1.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 0.0, y: -1.0}
    ],
    [
        Point{x: -1.0, y: 0.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 1.0, y: 0.0},
        Point{x: -1.0, y: -1.0},
        Point{x: -1.0, y: 1.0}
    ]
];

pub const PIECE_FA: [[Point; 5]; 4] = [
    [
        Point{x: -1.0, y: -1.0},
        Point{x: -1.0, y: 0.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 1.0, y: 0.0},
        Point{x: 0.0, y: 1.0}
    ],
    [
        Point{x: 1.0, y: -1.0},
        Point{x: -1.0, y: 0.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 0.0, y: -1.0},
        Point{x: 0.0, y: 1.0}
    ],
    [
        Point{x: 1.0, y: 1.0},
        Point{x: -1.0, y: 0.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 1.0, y: 0.0},
        Point{x: 0.0, y: -1.0}
    ],
    [
        Point{x: -1.0, y: 1.0},
        Point{x: 0.0, y: -1.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 1.0, y: 0.0},
        Point{x: 0.0, y: 1.0}
    ]
];

pub const PIECE_FB: [[Point; 5]; 4] = [
    [
        Point{x: -1.0, y: 0.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 1.0, y: 0.0},
        Point{x: 0.0, y: 1.0},
        Point{x: 1.0, y: -1.0}
    ],
    [
        Point{x: -1.0, y: 0.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 0.0, y: -1.0},
        Point{x: 0.0, y: 1.0},
        Point{x: 1.0, y: 1.0}
    ],
    [
        Point{x: -1.0, y: 0.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 1.0, y: 0.0},
        Point{x: -1.0, y: 1.0},
        Point{x: 0.0, y: -1.0}
    ],
    [
        Point{x: -1.0, y: -1.0},
        Point{x: 0.0, y: -1.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 0.0, y: 1.0},
        Point{x: 1.0, y: 0.0}
    ]
];

pub const PIECE_P: [[Point; 5]; 4] = [
    [
        Point{x: -1.0, y: 0.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 1.0, y: 0.0},
        Point{x: 0.0, y: 1.0},
        Point{x: 1.0, y: 1.0},
    ],
    [
        Point{x: 0.0, y: -1.0},
        Point{x: 0.0, y: 0.0},
        Point{x: -1.0, y: 0.0},
        Point{x: 0.0, y: 1.0},
        Point{x: -1.0, y: 1.0},
    ],
    [
        Point{x: 1.0, y: 0.0},
        Point{x: 0.0, y: 0.0},
        Point{x: -1.0, y: 0.0},
        Point{x: 0.0, y: -1.0},
        Point{x: -1.0, y: -1.0},
    ],
    [
        Point{x: 0.0, y: 1.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 1.0, y: 0.0},
        Point{x: 0.0, y: -1.0},
        Point{x: 1.0, y: -1.0},
    ]
];

pub const PIECE_Q: [[Point; 5]; 4] = [
    [
        Point{x: 1.0, y: 0.0},
        Point{x: 0.0, y: 0.0},
        Point{x: -1.0, y: 0.0},
        Point{x: 0.0, y: 1.0},
        Point{x: -1.0, y: 1.0},
    ],
    [
        Point{x: 0.0, y: 1.0},
        Point{x: 0.0, y: 0.0},
        Point{x: -1.0, y: 0.0},
        Point{x: 0.0, y: -1.0},
        Point{x: -1.0, y: -1.0},
    ],
    [
        Point{x: -1.0, y: 0.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 1.0, y: 0.0},
        Point{x: 0.0, y: -1.0},
        Point{x: 1.0, y: -1.0},
    ],
    [
        Point{x: 0.0, y: -1.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 1.0, y: 0.0},
        Point{x: 0.0, y: 1.0},
        Point{x: 1.0, y: 1.0},
    ]
];

pub const PIECE_W: [[Point; 5]; 4] = [
    [
        Point{x: -1.0, y: -1.0},
        Point{x: 0.0, y: -1.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 1.0, y: 0.0},
        Point{x: 1.0, y: 1.0}
    ],
    [
        Point{x: 1.0, y: -1.0},
        Point{x: 1.0, y: 0.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 0.0, y: 1.0},
        Point{x: -1.0, y: 1.0}
    ],
    [
        Point{x: -1.0, y: -1.0},
        Point{x: -1.0, y: 0.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 0.0, y: 1.0},
        Point{x: 1.0, y: 1.0}
    ],
    [
        Point{x: 1.0, y: -1.0},
        Point{x: 0.0, y: -1.0},
        Point{x: 0.0, y: 0.0},
        Point{x: -1.0, y: 0.0},
        Point{x: -1.0, y: 1.0}
    ]
];

pub const PIECE_YA: [[Point; 5]; 4] = [
    [
        Point{x: -1.0, y: 0.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 1.0, y: 0.0},
        Point{x: 2.0, y: 0.0},
        Point{x: 0.0, y: 1.0}
    ],
    [
        Point{x: -1.0, y: 0.0},
        Point{x: 0.0, y: -1.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 0.0, y: 1.0},
        Point{x: 0.0, y: 2.0}
    ],
    [
        Point{x: -2.0, y: 0.0},
        Point{x: -1.0, y: 0.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 1.0, y: 0.0},
        Point{x: 0.0, y: -1.0}
    ],
    [
        Point{x: 1.0, y: 0.0},
        Point{x: 0.0, y: -2.0},
        Point{x: 0.0, y: -1.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 0.0, y: 1.0}
    ]
];

pub const PIECE_YB: [[Point; 5]; 4] = [
    [
        Point{x: -1.0, y: 0.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 1.0, y: 0.0},
        Point{x: -2.0, y: 0.0},
        Point{x: 0.0, y: 1.0}
    ],
    [
        Point{x: -1.0, y: 0.0},
        Point{x: 0.0, y: -1.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 0.0, y: 1.0},
        Point{x: 0.0, y: -2.0}
    ],
    [
        Point{x: 2.0, y: 0.0},
        Point{x: -1.0, y: 0.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 1.0, y: 0.0},
        Point{x: 0.0, y: -1.0}
    ],
    [
        Point{x: 1.0, y: 0.0},
        Point{x: 0.0, y: 2.0},
        Point{x: 0.0, y: -1.0},
        Point{x: 0.0, y: 0.0},
        Point{x: 0.0, y: 1.0}
    ]
];

pub const PIECE_V: [[Point; 5]; 4] = [
    [
        Point{x: -1.0, y: -1.0},
        Point{x: 0.0, y: -1.0},
        Point{x: 1.0, y: -1.0},
        Point{x: -1.0, y: 0.0},
        Point{x: -1.0, y: 1.0}
    ],
    [
        Point{x: -1.0, y: -1.0},
        Point{x: 0.0, y: -1.0},
        Point{x: 1.0, y: -1.0},
        Point{x: 1.0, y: 0.0},
        Point{x: 1.0, y: 1.0}
    ],
    [
        Point{x: -1.0, y: 1.0},
        Point{x: 0.0, y: 1.0},
        Point{x: 1.0, y: 1.0},
        Point{x: 1.0, y: 0.0},
        Point{x: 1.0, y: -1.0}
    ],
    [
        Point{x: -1.0, y: 1.0},
        Point{x: 0.0, y: 1.0},
        Point{x: 1.0, y: 1.0},
        Point{x: -1.0, y: 0.0},
        Point{x: -1.0, y: -1.0}
    ]
];

pub const PIECES: [[[Point; 5]; 4]; 18] = [
    PIECE_I,
    PIECE_J,
    PIECE_L,
    PIECE_X,
    PIECE_S,
    PIECE_Z,
    PIECE_N,
    PIECE_G,
    PIECE_U,
    PIECE_T,
    PIECE_FA,
    PIECE_FB,
    PIECE_P,
    PIECE_Q,
    PIECE_W,
    PIECE_YA,
    PIECE_YB,
    PIECE_V
];

pub const WEIGHTS: [Weighted<usize>; 18] = [
    Weighted{ weight: 100, item: 0 },
    Weighted{ weight: 80, item: 1 },
    Weighted{ weight: 80, item: 2 },
    Weighted{ weight: 10, item: 3 }, // 10
    Weighted{ weight: 50, item: 4 },
    Weighted{ weight: 50, item: 5 },
    Weighted{ weight: 100, item: 6 },
    Weighted{ weight: 100, item: 7 },
    Weighted{ weight: 90, item: 8 }, // 90
    Weighted{ weight: 60, item: 9 },
    Weighted{ weight: 20, item: 10 },
    Weighted{ weight: 20, item: 11 },
    Weighted{ weight: 100, item: 12 },
    Weighted{ weight: 100, item: 13 },
    Weighted{ weight: 20, item: 14 },
    Weighted{ weight: 75, item: 15 },
    Weighted{ weight: 75, item: 16 },
    Weighted{ weight: 30, item: 17 }
];
