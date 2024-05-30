use keyberon::action::{k, m, Action::*, HoldTapAction, HoldTapConfig};
use keyberon::key_code::KeyCode::*;

type Action = keyberon::action::Action<()>;

macro_rules! hold_tap {
    ($hold:expr, $tap:expr) => {
        HoldTap(&HoldTapAction {
            timeout: 200,
            tap_hold_interval: 0,
            config: HoldTapConfig::Default,
            hold: $hold,
            tap: k($tap),
        })
    };
}

const CUT: Action = m(&[LShift, Delete].as_slice());
const COPY: Action = m(&[LCtrl, Insert].as_slice());
const PASTE: Action = m(&[LShift, Insert].as_slice());

const C_ESC: Action = hold_tap!(k(LCtrl), Escape);

const G_A: Action = hold_tap!(k(LGui), A);
const M_S: Action = hold_tap!(k(LAlt), S);
const C_D: Action = hold_tap!(k(LCtrl), D);
const S_F: Action = hold_tap!(k(LShift), F);
const S_J: Action = hold_tap!(k(RShift), J);
const C_K: Action = hold_tap!(k(RCtrl), K);
const M_L: Action = hold_tap!(k(LAlt), L);
const G_SC: Action = hold_tap!(k(RGui), SColon);

const G_1: Action = hold_tap!(k(LGui), Kb1);
const M_2: Action = hold_tap!(k(LAlt), Kb2);
const C_3: Action = hold_tap!(k(LCtrl), Kb3);
const S_4: Action = hold_tap!(k(LShift), Kb4);
const S_7: Action = hold_tap!(k(RShift), Kb7);
const C_8: Action = hold_tap!(k(RCtrl), Kb8);
const M_9: Action = hold_tap!(k(LAlt), Kb9);
const G_0: Action = hold_tap!(k(RGui), Kb0);

const STAB: Action = m(&[LShift, Tab].as_slice());
const AGS: Action = m(&[RAlt, LShift].as_slice());

const COLON: Action = m(&[LShift, N].as_slice());
const EQUAL: Action = m(&[RAlt, G].as_slice());
const NBSP: Action = m(&[RAlt, LShift, Space].as_slice());

#[rustfmt::skip]
pub static LAYERS: keyberon::layout::Layers<12, 4, 4, ()> = keyberon::layout::layout! {
    { //[+· ···+··· ···+··· ···+··· ···+··· ···+···|···+··· ···+··· ···+··· ···+··· ···+··· ···+],
        [Tab   Q       W       E       R       T       Y       U       I       O       P     '['],
        [']' {G_A}   {M_S}   {C_D}   {S_F}     G       H     {S_J}   {C_K}   {M_L}  {G_SC}  '\''],
        [=     Z       X       C       V       B       N       M       ,       .       /    '\\'],
        [n     n       n       n     Space    (1)     (2)   RShift   RAlt    {AGS}     n       n],
    }{//[+· ···+··· ···+··· ···+··· ···+··· ···+···|···+··· ···+··· ···+··· ···+··· ···+··· ···+],
        [n Pause CapsLock ScrollLock PScreen {STAB}    n    BSpace  Delete  Insert     n       n],
        [n   LGui    LAlt   {C_ESC} LShift    Tab      n     Left    Down     Up     Right     n],
        [n   Undo    {CUT}  {COPY}  {PASTE}    n     Enter   Home   PgDown   PgUp     End      n],
        [n     n       t       t       t       n      (3)      t       t       t       n       n],
    }{//[+· ···+··· ···+··· ···+··· ···+··· ···+···|···+··· ···+··· ···+··· ···+··· ···+··· ···+],
        [~     !       @       #       $       %       ^       &       *      '('     ')'    '_'],
        ['`' {G_1}   {M_2}   {C_3}   {S_4}     5       6     {S_7}   {C_8}   {M_9}   {G_0}     -],
        [n     n    {NBSP}  {COLON}    .    {EQUAL}    N    KpPlus  KpMinus KpSlash KpAsterisk n],
        [n     n       t       t       t      (3)      n       t       t       t       n       n],
    }{//[+· ···+··· ···+··· ···+··· ···+··· ···+···|···+··· ···+··· ···+··· ···+··· ···+··· ···+],
        [n    F1      F2      F3      F4      F5      F6      F7      F8      F9      F10      n],
        [n   LGui    LAlt    LCtrl  LShift     n       n    RShift   RCtrl   LAlt    RGui      n],
        [n    F11     F12      n       n       n       n       n       n       n       n       n],
        [n     n       t  {Custom(())} t       n       n       t       t       t       n       n],
    } //[+· ···+··· ···+··· ···+··· ···+··· ···+···|···+··· ···+··· ···+··· ···+··· ···+··· ···+],
};
