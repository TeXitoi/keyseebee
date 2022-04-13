use keyberon::action::{k, l, m, Action::*, HoldTapConfig};
use keyberon::key_code::KeyCode::*;

type Action = keyberon::action::Action<()>;

const CUT: Action = m(&[LShift, Delete]);
const COPY: Action = m(&[LCtrl, Insert]);
const PASTE: Action = m(&[LShift, Insert]);
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
pub static LAYERS: keyberon::layout::Layers<12, 4, 4, ()> = keyberon::layout::layout! {
    {
        [Tab Q  W    E     R     T     Y    U  I   O P '[' ],
        [']' A  S    D     F     G     H    J  K   L ; '\''],
        [ =  Z  X    C     V     B     N    M  ,   . / '\\'],
        [ t  t LGui LAlt{L1_SP}LCtrl RShift(2)RAlt t t  t  ],
    }{
        [{Custom(())}Pause ScrollLock PScreen       t     t    t    BSpace Delete  t    t   t],
        [t           LGui     LAlt   {CTRL_INS}{SHIFT_ESC}t CapsLock Left   Down   Up Right t],
        [NonUsBslash Undo    {CUT}     {COPY}    {PASTE}  t  Enter   Home  PgDown PgUp End  t],
        [t             t       t         t          n     t    t  {L3_ENTER}  t    t    t   t],
    }{
        [{s!(Grave)}{s!(Kb1)}{s!(Kb2)}{s!(Kb3)}{s!(Kb4)}{s!(Kb5)}{s!(Kb6)}{s!(Kb7)}{s!(Kb8)}{s!(Kb9)}{s!(Kb0)}{s!(Minus)}],
        [{ k(Grave)}{ k(Kb1)}{ k(Kb2)}{ k(Kb3)}{ k(Kb4)}{ k(Kb5)}{ k(Kb6)}{ k(Kb7)}{ k(Kb8)}{ k(Kb9)}{ k(Kb0)}{ k(Minus)}],
        [{a!(Grave)}{a!(Kb1)}{a!(Kb2)}{a!(Kb3)}{a!(Kb4)}{a!(Kb5)}{a!(Kb6)}{a!(Kb7)}{a!(Kb8)}{a!(Kb9)}{a!(Kb0)}{a!(Minus)}],
        [t t t t {CSPACE} t t n t t t t],
    }{
        [F1           F2   F3    F4    F5  F6 F7  F8     F9  F10  F11 F12],
        [t           LGui LAlt LCtrl LShift t t RShift RCtrl LAlt RGui  t],
        [{Custom(())} t    t     t     t    t t   t      t    t    t    t],
        [t            t    t     t     n    t t   n      t    t    t    t],
    }
};
