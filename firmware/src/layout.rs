use keyberon::action::{k, l, m, Action::*, HoldTapAction, HoldTapConfig};
use keyberon::key_code::KeyCode::*;

type Action = keyberon::action::Action<()>;

const TIMEOUT: u16 = 180;
const CUT: Action = m(&[LShift, Delete].as_slice());
const COPY: Action = m(&[LCtrl, Insert].as_slice());
const PASTE: Action = m(&[LShift, Insert].as_slice());
const L1_SP: Action = HoldTap(&HoldTapAction {
    timeout: TIMEOUT,
    tap_hold_interval: 0,
    config: HoldTapConfig::Default,
    hold: l(1),
    tap: k(Space),
});
const CSP: Action = m(&[LCtrl, Space].as_slice());
const STAB: Action = m(&[LShift, Tab].as_slice());
const AL_SH: Action = m(&[RAlt, LShift].as_slice());
const CT_ES: Action = HoldTap(&HoldTapAction {
    timeout: TIMEOUT,
    tap_hold_interval: 0,
    config: HoldTapConfig::Default,
    hold: k(LCtrl),
    tap: k(Escape),
});

macro_rules! s {
    ($k:ident) => {
        m(&[LShift, $k].as_slice())
    };
}
macro_rules! a {
    ($k:ident) => {
        m(&[RAlt, $k].as_slice())
    };
}
macro_rules! t {
    ($k:ident) => {
        m(&[O, $k].as_slice())
    };
}

#[rustfmt::skip]
pub static LAYERS: keyberon::layout::Layers<12, 4, 4, ()> = keyberon::layout::layout! {
    {
        [Tab Q  W    E     R     T     Y    U  I     O   P '[' ],
        [']' A  S    D     F     G     H    J  K     L   ; '\''],
        [ =  Z  X    C     V     B     N    M  ,     .   / '\\'],
        [ n  n LGui LAlt{L1_SP}LCtrl RShift(2)RAlt{AL_SH}n  n  ],
    }{
        [n Pause CapsLock ScrollLock PScreen{STAB}n BSpace Delete Insert n n],
        [n LGui     LAlt   {CT_ES}   LShift  Tab  n  Left   Down  Up Right n],
        [n Undo    {CUT}    {COPY}  {PASTE}   n Enter Home PgDown PgUp End n],
        [n   n       t         t        n     t   t   (3)     t    t     n n],
    }{
        [      ~         !        @       #  $    %  ^   &       *       '('      ')'        '_'   ],
        [     '`'        1        2       3  4    5  6   7       8        9        0          -    ],
        [{t!(Grave)}{t!(Kb1)}{t!(Kb2)}{s!(N)}.{a!(G)}N KpPlus KpMinus KpSlash KpAsterisk{a!(Minus)}],
        [      n         n        t       t{CSP}  t  t   n       t        t        n          n    ],
    }{
        [n  F1   F2    F3    F4  F5 F6  F7    F8    F9  F10  n],
        [n LGui LAlt LCtrl LShift n n RShift RCtrl LAlt RGui n],
        [n F11  F12     n     n   n n   n      n    n    n   n],
        [n n{Custom(())}t     n   t t   n      t    t    n   n],
    }
};
