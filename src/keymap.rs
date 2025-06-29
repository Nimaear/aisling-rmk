use rmk::action::KeyAction;
use rmk::keycode::ModifierCombination;
use rmk::{k, layer, mo, wm};

pub(crate) const COL: usize = 12;
pub(crate) const COL_TOTAL: usize = 24;
pub(crate) const ROW: usize = 5;
pub(crate) const NUM_LAYER: usize = 3;

const L_SHIFT: ModifierCombination = ModifierCombination::from_bits(1);
const ____: KeyAction = KeyAction::Transparent;
const XXXX: KeyAction = KeyAction::No;

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL_TOTAL]; ROW]; NUM_LAYER] {
    [
        layer!([
            [k!(F1),  k!(F2),  k!(F3),  k!(F4),     k!(Escape),    k!(Kc1),   k!(Kc2),   k!(Kc3),    k!(Kc4),  k!(Kc5),  XXXX,       XXXX,             XXXX,       XXXX,      k!(Kc6),  k!(Kc7),  k!(Kc8),    k!(Kc9),     k!(Kc0),       k!(A),           k!(A),  k!(A),  k!(A),  k!(A)],
            [k!(F5),  k!(F6),  k!(F7),  k!(F8),     k!(Tab),       k!(Q),     k!(W),     k!(F),      k!(P),    k!(G),    XXXX,       XXXX,             XXXX,       XXXX,      k!(J),    k!(L),    k!(U),      k!(Y),       k!(Semicolon), k!(Backslash),   k!(A),  k!(A),  k!(A),  k!(A)],
            [k!(F9),  k!(F10), k!(F11), k!(F12),    k!(Backspace), k!(A),     k!(A),     k!(A),      k!(T),    k!(D),    XXXX,       XXXX,             XXXX,       XXXX,      k!(H),    k!(N),    k!(E),      k!(I),       k!(O),         k!(Quote),       k!(A),  k!(A),  k!(A),  k!(A)],
            [k!(F13), k!(F14), k!(F15), k!(F16),    k!(LShift),    k!(Z),     k!(X),     k!(C),      k!(V),    k!(D),    XXXX,       XXXX,             XXXX,       XXXX,      k!(K),    k!(M),    k!(Comma),  k!(Dot),     k!(Slash),     k!(Return),      k!(A),  k!(A),  k!(A),  k!(A)],
            [k!(F17), k!(F18), k!(F19), k!(F20),    k!(LCtrl),     k!(LGui),  k!(LAlt),  mo!(1),     mo!(3),    k!(B),   k!(Space),  k!(Return),       k!(Delete), k!(Space), mo!(2),   mo!(4),   XXXX,       XXXX,        XXXX,          k!(A),           k!(A),  k!(A),  k!(A),  k!(A)]
        ]),
        // Lower
        layer!([
            [____, ____, ____, ____,   ____, ____, ____,             ____,                     ____,              ____,                 ____,  ____,   ____,  ____, ____,                  ____,              ____,                       ____,              ____, ____,   ____,  ____, ____, ____  ],
            [____, ____, ____, ____,   ____, ____, ____,             ____,                     ____,              ____,                 ____,  ____,   ____,  ____, ____,                  ____,              ____,                       ____,              ____, ____,   ____,  ____, ____, ____  ],
            [____, ____, ____, ____,   ____, ____, k!(LeftBracket),  wm!(LeftBracket, L_SHIFT),wm!(Kc9, L_SHIFT), k!(Minus),            ____,  ____,   ____,  ____, k!(Equal),             wm!(Kc0, L_SHIFT), wm!(RightBracket, L_SHIFT), k!(RightBracket),  ____, ____,   ____,  ____, ____, ____  ],
            [____, ____, ____, ____,   ____, ____, ____,             ____,                     ____,              wm!( Minus, L_SHIFT), ____,  ____,   ____,  ____, wm!(Equal, L_SHIFT),   ____,              ____,                       ____,              ____, ____,   ____,  ____, ____, ____  ],
            [____, ____, ____, ____,   ____, ____, ____,             ____,                     ____,              ____,                 ____,  ____,   ____,  ____, ____,                  ____,              ____,                       ____,              ____, ____,   ____,  ____, ____, ____ ]
        ]),
        // Lower
        layer!([
            [____,  ____,  ____,  ____,     ____,  ____,  ____,  ____,  ____,  ____,  ____,  ____,        ____,  ____,  ____,          ____,      ____,      ____,       ____,  ____,      ____,  ____,  ____,  ____   ],
            [____,  ____,  ____,  ____,     ____,  ____,  ____,  ____,  ____,  ____,  ____,  ____,        ____,  ____,  k!(PageUp),    k!(Home),  k!(Up),    k!(End),    ____,  ____,      ____,  ____,  ____,  ____   ],
            [____,  ____,  ____,  ____,     ____,  ____,  ____,  ____,  ____,  ____,  ____,  ____,        ____,  ____,  k!(PageDown),  k!(Left),  k!(Down),  k!(Right),  ____,  ____,      ____,  ____,  ____,  ____   ],
            [____,  ____,  ____,  ____,     ____,  ____,  ____,  ____,  ____,  ____,  ____,  ____,        ____,  ____,  ____,          ____,      ____,      ____,       ____,  ____,      ____,  ____,  ____,  ____   ],
            [____,  ____,  ____,  ____,     ____,  ____,  ____,  ____,  ____,  ____,  ____,  ____,        ____,  ____,  ____,          ____,      ____,      ____,       ____,  ____,      ____,  ____,  ____,  ____   ]
        ]),
    ]
}
