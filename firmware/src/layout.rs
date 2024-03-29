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
        [n Q  W    E     R     T     Y    U  I     O   P n],
        [n A  S    D     F     G     H    J  K     L   ; n],
        [n Z  X    C     V     B     N    M  ,     .   / n],
        [n n LGui LAlt{L1_SP}LCtrl RShift(2)RAlt{AL_SH}n n],
    }{
        [n Pause CapsLock ScrollLock PScreen n  n BSpace Delete Space Tab n],
        [n LGui     LAlt   {CT_ES}   LShift Tab n   Left   Down  Up Right n],
        [n Undo    {CUT}    {COPY}  {PASTE}  n Enter Home PgDown PgUp End n],
        [n   n       t         t        n    t  t    (3)     t    t    n  n],
    }{
        [n{s!(Kb1)}{s!(Kb2)}{s!(Kb3)}{s!(Kb4)}{s!(Kb5)}{s!(Kb6)}{s!(Kb7)}{s!(Kb8)}{s!(Kb9)}{s!(Kb0)}n],
        [n{ k(Kb1)}{ k(Kb2)}{ k(Kb3)}{ k(Kb4)}{ k(Kb5)}{ k(Kb6)}{ k(Kb7)}{ k(Kb8)}{ k(Kb9)}{ k(Kb0)}n],
        [n{t!(Kb1)}{t!(Kb2)}{ s!(N) }     .   { a!(G) }     N     KpPlus KpMinus KpSlash KpAsterisk n],
        [n     n        t        t      {CSP}      t        t        n        t        t        n   n],
    }{
        [n  F1   F2    F3    F4  F5 F6  F7    F8    F9  F10  n],
        [n LGui LAlt LCtrl LShift n n RShift RCtrl LAlt RGui n],
        [n F11  F12     n     n   n n   n      n    n    n   n],
        [n n{Custom(())}t     n   t t   n      t    t    n   n],
    }
};
