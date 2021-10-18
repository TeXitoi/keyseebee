use keyberon::action::{k, l, m, Action::*, HoldTapConfig};
use keyberon::key_code::KeyCode::*;

type Action = keyberon::action::Action<()>;

const CUT: Action = m(&[LShift, Delete]);
const COPY: Action = m(&[LCtrl, Insert]);
const PASTE: Action = m(&[LShift, Insert]);
const L2_ENTER: Action = HoldTap {
    timeout: 200,
    tap_hold_interval: 0,
    config: HoldTapConfig::HoldOnOtherKeyPress,
    hold: &l(2),
    tap: &k(Enter),
};
const L3_ENTER: Action = HoldTap {
    timeout: 200,
    tap_hold_interval: 0,
    config: HoldTapConfig::HoldOnOtherKeyPress,
    hold: &l(3),
    tap: &k(Enter),
};
const L1_SP: Action = HoldTap {
    timeout: 200,
    tap_hold_interval: 0,
    config: HoldTapConfig::Default,
    hold: &l(1),
    tap: &k(Space),
};
const CSPACE: Action = m(&[LCtrl, Space]);

const SHIFT_ESC: Action = HoldTap {
    timeout: 200,
    tap_hold_interval: 0,
    config: HoldTapConfig::Default,
    hold: &k(LShift),
    tap: &k(Escape),
};
const CTRL_INS: Action = HoldTap {
    timeout: 200,
    tap_hold_interval: 0,
    config: HoldTapConfig::Default,
    hold: &k(LCtrl),
    tap: &k(Insert),
};

macro_rules! s {
    ($k:ident) => {
        m(&[LShift, $k])
    };
}
macro_rules! a {
    ($k:ident) => {
        m(&[RAlt, $k])
    };
}

#[rustfmt::skip]
pub static LAYERS: keyberon::layout::Layers<()> = &[
    &[
        &[k(Tab),     k(Q), k(W),  k(E),    k(R), k(T),    k(Y),     k(U),    k(I),   k(O),    k(P),     k(LBracket)],
        &[k(RBracket),k(A), k(S),  k(D),    k(F), k(G),    k(H),     k(J),    k(K),   k(L),    k(SColon),k(Quote)   ],
        &[k(Equal),   k(Z), k(X),  k(C),    k(V), k(B),    k(N),     k(M),    k(Comma),k(Dot), k(Slash), k(Bslash)  ],
        &[Trans,      Trans,k(LGui),k(LAlt),L1_SP,k(LCtrl),k(RShift),L2_ENTER,k(RAlt),k(BSpace),Trans,   Trans      ],
    ], &[
        &[Custom(()),    k(Pause),k(ScrollLock),k(PScreen),Trans,    Trans,Trans,      k(BSpace),k(Delete),Trans,  Trans,   Trans],
        &[Trans,         k(LGui), k(LAlt),      CTRL_INS,  SHIFT_ESC,Trans,k(CapsLock),k(Left),  k(Down),  k(Up),  k(Right),Trans],
        &[k(NonUsBslash),k(Undo), CUT,          COPY,      PASTE,    Trans,Trans,      k(Home),  k(PgDown),k(PgUp),k(End),  Trans],
        &[Trans,         Trans,   Trans,        Trans,     NoOp,     Trans,Trans,      L3_ENTER, Trans,    Trans,  Trans,   Trans],
    ], &[
        &[s!(Grave),s!(Kb1),s!(Kb2),s!(Kb3),s!(Kb4),s!(Kb5),s!(Kb6),s!(Kb7),s!(Kb8),s!(Kb9),s!(Kb0),s!(Minus)],
        &[ k(Grave), k(Kb1), k(Kb2), k(Kb3), k(Kb4), k(Kb5), k(Kb6), k(Kb7), k(Kb8), k(Kb9), k(Kb0), k(Minus)],
        &[a!(Grave),a!(Kb1),a!(Kb2),a!(Kb3),a!(Kb4),a!(Kb5),a!(Kb6),a!(Kb7),a!(Kb8),a!(Kb9),a!(Kb0),a!(Minus)],
        &[Trans,    Trans,  Trans,  Trans,  CSPACE, Trans,  Trans,  NoOp,   Trans,  Trans,  Trans,  Trans    ],
    ], &[
        &[k(F1),k(F2),  k(F3),  k(F4),   k(F5),    k(F6),k(F7),k(F8),    k(F9),   k(F10), k(F11), k(F12)],
        &[Trans,k(LGui),k(LAlt),k(LCtrl),k(LShift),Trans,Trans,k(RShift),k(RCtrl),k(LAlt),k(RGui),Trans ],
        &[Custom(()),Trans,  Trans,  Trans,   Trans,    Trans,Trans,Trans,    Trans,   Trans,  Trans,  Trans ],
        &[Trans,Trans,  Trans,  Trans,   Trans,    Trans,Trans,Trans,    Trans,   Trans,  Trans,  Trans ],
    ],
];
