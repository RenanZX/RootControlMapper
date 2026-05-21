use super::vk_types::KeyboardLayout;
use super::vk_types::KeyboardLayout::{
    AccentKeys, CircunflexKeys, CrasisKeys, GeneralKeys, OtherKeys, TildeKeys,
};

pub fn is_macro_key(key: &str) -> bool {
    match key {
        "Ctrl" | "Alt" | "F1" | "F2" | "F3" | "F4" | "F5" | "F6" | "F7" | "F8" | "F9" | "F10"
        | "F11" | "F12" => true,
        _ => false,
    }
}

pub fn is_function_key(key: &str) -> bool {
    match key {
        "F1" | "F2" | "F3" | "F4" | "F5" | "F6" | "F7" | "F8" | "F9" | "F10" | "F11" | "F12" => {
            true
        }
        _ => false,
    }
}

pub fn get_keyboard_layout(kb_layout: KeyboardLayout) -> Vec<Vec<(&'static str, f32)>> {
    let rt_cmds = vec![
        ("Change", 2.0),
        ("Change2", 2.0),
        ("Helper", 2.0),
        ("Backspace", 3.0),
    ];
    match kb_layout {
        GeneralKeys => vec![
            // Linha 1: LT, Números, RT
            vec![
                ("Change", 2.0),
                ("Change2", 2.0),
                ("1", 1.0),
                ("2", 1.0),
                ("3", 1.0),
                ("4", 1.0),
                ("5", 1.0),
                ("6", 1.0),
                ("7", 1.0),
                ("8", 1.0),
                ("9", 1.0),
                ("0", 1.0),
                ("Backspace", 3.0),
            ],
            // Linha 2: LB, Letras Q-P, RB
            vec![
                ("Left", 1.2),
                ("q", 1.0),
                ("w", 1.0),
                ("e", 1.0),
                ("r", 1.0),
                ("t", 1.0),
                ("y", 1.0),
                ("u", 1.0),
                ("i", 1.0),
                ("o", 1.0),
                ("p", 1.0),
                ("Right", 1.2),
            ],
            // Linha 3: Seta Esq, Letras A-L, Seta Dir
            vec![
                ("CapsLock", 1.8),
                ("a", 1.0),
                ("s", 1.0),
                ("d", 1.0),
                ("f", 1.0),
                ("g", 1.0),
                ("h", 1.0),
                ("j", 1.0),
                ("k", 1.0),
                ("l", 1.0),
                ("@", 1.0),
                ("!", 1.0),
            ],
            // Linha 4: Seta Cima, Letras Z-M, Seta Baixo
            vec![
                ("Shift", 1.2),
                ("z", 1.0),
                ("x", 1.0),
                ("c", 1.0),
                ("v", 1.0),
                ("b", 1.0),
                ("n", 1.0),
                ("m", 1.0),
                (",", 1.0),
                (".", 1.0),
                (";", 0.8),
                ("?", 0.8),
            ],
            // Linha 5: View, Menu, Espaço (com ícone Y)
            vec![
                ("Ctrl", 1.0),
                ("Space", 3.0), // Espaço maior para caber o ícone Y
                ("Alt", 1.0),
                (".com", 0.8),
                ("Close", 1.0),
                ("Enter", 1.0),
            ],
        ],
        AccentKeys => vec![
            // Linha 1: LT, Números, RT
            rt_cmds,
            // Linha 2: LB, Letras Q-P, RB
            vec![
                ("Left", 1.2),
                ("q", 1.0),
                ("w", 1.0),
                ("é", 1.0),
                ("ŕ", 1.0),
                ("t", 1.0),
                ("ý", 1.0),
                ("ú", 1.0),
                ("í", 1.0),
                ("ó", 1.0),
                ("ṕ", 1.0),
                ("Right", 1.2),
            ],
            // Linha 3: Seta Esq, Letras A-L, Seta Dir
            vec![
                ("CapsLock", 1.8),
                ("á", 1.0),
                ("ś", 1.0),
                ("d", 1.0),
                ("f", 1.0),
                ("ǵ", 1.0),
                ("h", 1.0),
                ("j́", 1.0),
                ("ḱ", 1.0),
                ("ĺ", 1.0),
                ("@", 1.0),
                ("!", 1.0),
            ],
            // Linha 4: Seta Cima, Letras Z-M, Seta Baixo
            vec![
                ("Shift", 1.2),
                ("ź", 1.0),
                ("x", 1.0),
                ("ć", 1.0),
                ("ç", 1.0),
                ("ǘ", 1.0),
                ("b", 1.0),
                ("ń", 1.0),
                ("ḿ", 1.0),
                (".", 1.0),
                (";", 0.8),
                ("?", 0.8),
            ],
            // Linha 5: View, Menu, Espaço (com ícone Y)
            vec![
                ("Ctrl", 1.0),
                ("Space", 3.0), // Espaço maior para caber o ícone Y
                ("Alt", 1.0),
                (".com", 0.8),
                ("Close", 1.0),
                ("Enter", 1.0),
            ],
        ],
        CircunflexKeys => vec![
            // Linha 1: LT, Números, RT
            rt_cmds,
            // Linha 2: LB, Letras Q-P, RB
            vec![
                ("Left", 1.2),
                ("q", 1.0),
                ("w", 1.0),
                ("ê", 1.0),
                ("r", 1.0),
                ("t", 1.0),
                ("ŷ", 1.0),
                ("û", 1.0),
                ("î", 1.0),
                ("ô", 1.0),
                ("p", 1.0),
                ("Right", 1.2),
            ],
            // Linha 3: Seta Esq, Letras A-L, Seta Dir
            vec![
                ("CapsLock", 1.8),
                ("â", 1.0),
                ("ŝ", 1.0),
                ("d", 1.0),
                ("f", 1.0),
                ("ĝ", 1.0),
                ("ĥ", 1.0),
                ("ĵ", 1.0),
                ("k", 1.0),
                ("l", 1.0),
                ("@", 1.0),
                ("!", 1.0),
            ],
            // Linha 4: Seta Cima, Letras Z-M, Seta Baixo
            vec![
                ("Shift", 1.2),
                ("ẑ", 1.0),
                ("x", 1.0),
                ("ĉ", 1.0),
                ("ç", 1.0),
                ("û", 1.0),
                ("b", 1.0),
                ("n", 1.0),
                ("m", 1.0),
                (".", 1.0),
                (";", 0.8),
                ("?", 0.8),
            ],
            // Linha 5: View, Menu, Espaço (com ícone Y)
            vec![
                ("Ctrl", 1.0),
                ("Space", 3.0), // Espaço maior para caber o ícone Y
                ("Alt", 1.0),
                (".com", 0.8),
                ("Close", 1.0),
                ("Enter", 1.0),
            ],
        ],
        TildeKeys => vec![
            // Linha 1: LT, Números, RT
            rt_cmds,
            // Linha 2: Letras Q-P com Til
            vec![
                ("Left", 1.2),
                ("q̃", 1.0),
                ("w̃", 1.0),
                ("ẽ", 1.0),
                ("r̃", 1.0),
                ("t̃", 1.0),
                ("ỹ", 1.0),
                ("ũ", 1.0),
                ("ĩ", 1.0),
                ("õ", 1.0),
                ("p̃", 1.0),
                ("Right", 1.2),
            ],
            // Linha 3: Letras A-L com Til
            vec![
                ("CapsLock", 1.8),
                ("ã", 1.0),
                ("s̃", 1.0),
                ("d̃", 1.0),
                ("f̃", 1.0),
                ("g̃", 1.0),
                ("h̃", 1.0),
                ("j̃", 1.0),
                ("k̃", 1.0),
                ("l̃", 1.0),
                ("@", 1.0),
                ("!", 1.0),
            ],
            // Linha 4: Letras Z-M com Til
            vec![
                ("Shift", 1.2),
                ("z̃", 1.0),
                ("x̃", 1.0),
                ("c̃", 1.0),
                ("ç", 1.0),
                ("ṽ", 1.0),
                ("b̃", 1.0),
                ("ñ", 1.0),
                ("m̃", 1.0),
                (".", 1.0),
                (";", 0.8),
                ("?", 0.8),
            ],
            // Linha 5: View, Menu, Espaço (com ícone Y)
            vec![
                ("Ctrl", 1.0),
                ("Space", 3.0), // Espaço maior para caber o ícone Y
                ("Alt", 1.0),
                (".com", 0.8),
                ("Close", 1.0),
                ("Enter", 1.0),
            ],
        ],
        CrasisKeys => vec![
            // Linha 1: LT, Números, RT
            rt_cmds,
            // Linha 2: Letras Q-P com Crase
            vec![
                ("Left", 1.2),
                ("q̀", 1.0),
                ("ẁ", 1.0),
                ("è", 1.0),
                ("r̀", 1.0),
                ("t̀", 1.0),
                ("ỳ", 1.0),
                ("ù", 1.0),
                ("ì", 1.0),
                ("ò", 1.0),
                ("p̀", 1.0),
                ("Right", 1.2),
            ],
            // Linha 3: Letras A-L com Crase
            vec![
                ("CapsLock", 1.8),
                ("à", 1.0),
                ("s̀", 1.0),
                ("d̀", 1.0),
                ("f̀", 1.0),
                ("g̀", 1.0),
                ("h̀", 1.0),
                ("j̀", 1.0),
                ("k̀", 1.0),
                ("l̀", 1.0),
                ("@", 1.0),
                ("!", 1.0),
            ],
            // Linha 4: Letras Z-M com Crase
            vec![
                ("Shift", 1.2),
                ("z̀", 1.0),
                ("x̀", 1.0),
                ("c̀", 1.0),
                ("ç", 1.0),
                ("v̀", 1.0),
                ("b̀", 1.0),
                ("ǹ", 1.0),
                ("m̀", 1.0),
                (".", 1.0),
                (";", 0.8),
                ("?", 0.8),
            ],
            // Linha 5: View, Menu, Espaço (com ícone Y)
            vec![
                ("Ctrl", 1.0),
                ("Space", 3.0), // Espaço maior para caber o ícone Y
                ("Alt", 1.0),
                (".com", 0.8),
                ("Close", 1.0),
                ("Enter", 1.0),
            ],
        ],
        OtherKeys => vec![
            vec![
                ("Change", 2.0),
                ("F1", 1.0),
                ("F2", 1.0),
                ("F3", 1.0),
                ("F4", 1.0),
                ("F5", 1.0),
                ("F6", 1.0),
                ("F7", 1.0),
                ("F8", 1.0),
                ("F9", 1.0),
                ("F10", 1.0),
                ("F11", 1.0),
                ("F12", 1.0),
            ],
            vec![
                ("+", 1.0),
                ("-", 1.0),
                ("*", 1.0),
                ("=", 1.0),
                (":", 1.0),
                ("§", 1.0),
                ("#", 1.0),
                ("$", 1.0),
                ("%", 1.0),
                ("^", 1.0),
                ("&", 1.0),
                ("(", 1.0),
                (")", 1.0),
            ],
            vec![
                ("'", 1.0),
                ("\"", 1.0),
                ("´", 1.0),
                ("~", 1.0),
                ("\\", 1.0),
                ("_", 1.0),
                ("|", 1.0),
                ("/", 1.0),
                ("<", 1.0),
                (">", 1.0),
                ("ª", 1.0),
                ("º", 1.0),
                ("{", 1.0),
                ("}", 1.0),
                ("[", 1.0),
                ("]", 1.0),
            ],
            vec![
                ("Ctrl", 1.0),
                ("Space", 3.0), // Espaço maior para caber o ícone Y
                ("Alt", 1.0),
                ("www.", 0.8),
                ("Close", 0.8),
                ("Enter", 1.0),
            ],
        ],
    }
}
