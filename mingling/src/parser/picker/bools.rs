use crate::parser::Pickable;

#[derive(Debug, Default)]
pub enum YesOrNo {
    Yes,
    #[default]
    No,
}

impl From<bool> for YesOrNo {
    fn from(b: bool) -> Self {
        if b { YesOrNo::Yes } else { YesOrNo::No }
    }
}

impl From<YesOrNo> for bool {
    fn from(val: YesOrNo) -> Self {
        match val {
            YesOrNo::Yes => true,
            YesOrNo::No => false,
        }
    }
}

impl std::ops::Deref for YesOrNo {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        static TRUE: bool = true;
        static FALSE: bool = false;
        match self {
            YesOrNo::Yes => &TRUE,
            YesOrNo::No => &FALSE,
        }
    }
}

impl YesOrNo {
    pub fn is_yes(&self) -> bool {
        matches!(self, YesOrNo::Yes)
    }

    pub fn is_no(&self) -> bool {
        matches!(self, YesOrNo::No)
    }
}

impl Pickable for YesOrNo {
    type Output = YesOrNo;

    fn pick(args: &mut crate::parser::Argument, flag: mingling_core::Flag) -> Option<Self::Output> {
        let value = pick_bool(args, flag, &["y", "yes"], &["n", "no"]);
        Some(value.into())
    }
}

#[derive(Debug, Default)]
pub enum TrueOrFalse {
    True,
    #[default]
    False,
}

impl From<bool> for TrueOrFalse {
    fn from(b: bool) -> Self {
        if b {
            TrueOrFalse::True
        } else {
            TrueOrFalse::False
        }
    }
}

impl From<TrueOrFalse> for bool {
    fn from(val: TrueOrFalse) -> Self {
        match val {
            TrueOrFalse::True => true,
            TrueOrFalse::False => false,
        }
    }
}

impl std::ops::Deref for TrueOrFalse {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        static TRUE: bool = true;
        static FALSE: bool = false;
        match self {
            TrueOrFalse::True => &TRUE,
            TrueOrFalse::False => &FALSE,
        }
    }
}

impl TrueOrFalse {
    pub fn is_true(&self) -> bool {
        matches!(self, TrueOrFalse::True)
    }

    pub fn is_false(&self) -> bool {
        matches!(self, TrueOrFalse::False)
    }
}

impl Pickable for TrueOrFalse {
    type Output = TrueOrFalse;

    fn pick(args: &mut crate::parser::Argument, flag: mingling_core::Flag) -> Option<Self::Output> {
        let value = pick_bool(args, flag, &["true", "t"], &["false", "f"]);
        Some(value.into())
    }
}

fn pick_bool(
    args: &mut crate::parser::Argument,
    flag: mingling_core::Flag,
    positive: &[&str],
    negative: &[&str],
) -> bool {
    let has_flag = args.pick_flag(flag.clone());
    if !has_flag {
        let content = args.pick_argument(flag);
        match content {
            Some(content) => {
                let s = content.as_str();
                if positive.contains(&s) {
                    true
                } else if negative.contains(&s) {
                    false
                } else {
                    false
                }
            }
            None => false,
        }
    } else {
        true
    }
}
