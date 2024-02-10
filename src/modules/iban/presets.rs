use once_cell::{self, sync::Lazy};

use rand::Rng;

#[derive(Debug, Clone)]
pub struct Bban {
    pub r#type: String,
    pub count: u32,
}

#[derive(Debug, Clone)]
pub struct Iban {
    pub country: String,
    pub total: u32,
    pub bban: Vec<Bban>,
    pub format: String,
}

pub const COUNTRY_CODES: [&str; 68] = [
    "al", "ad", "at", "az", "bh", "be", "ba", "br", "bg", "cr", "hr", "cy", "cz", "dk", "do", "tl",
    "ee", "fo", "fi", "fr", "ge", "de", "gi", "gr", "gl", "gt", "hu", "is", "ie", "il", "it", "jo",
    "kz", "xk", "kw", "lv", "lb", "li", "lt", "lu", "mk", "mt", "mr", "mu", "mc", "md", "me", "nl",
    "no", "pk", "ps", "pl", "pt", "qa", "ro", "sm", "sa", "rs", "sk", "si", "es", "se", "ch", "tn",
    "tr", "ae", "gb", "vg",
];

pub fn random_country_code() -> String {
    COUNTRY_CODES[from_range!(0..COUNTRY_CODES.len())].to_string()
}

pub fn iban_from_country_code(cc: Option<String>) -> Iban {
    let cc = cc.unwrap_or(random_country_code()).to_lowercase();
    let cc = COUNTRY_CODES
        .iter()
        .find(|&c| c == &cc)
        .map(|c| c.to_string())
        .unwrap_or(random_country_code());

    match cc.to_lowercase().as_str() {
        "al" => AL.clone(),
        "ad" => AD.clone(),
        "at" => AT.clone(),
        "az" => AZ.clone(),
        "bh" => BH.clone(),
        "be" => BE.clone(),
        "ba" => BA.clone(),
        "br" => BR.clone(),
        "bg" => BG.clone(),
        "cr" => CR.clone(),
        "hr" => HR.clone(),
        "cy" => CY.clone(),
        "cz" => CZ.clone(),
        "dk" => DK.clone(),
        "do" => DO.clone(),
        "tl" => TL.clone(),
        "ee" => EE.clone(),
        "fo" => FO.clone(),
        "fi" => FI.clone(),
        "fr" => FR.clone(),
        "ge" => GE.clone(),
        "de" => DE.clone(),
        "gi" => GI.clone(),
        "gr" => GR.clone(),
        "gl" => GL.clone(),
        "gt" => GT.clone(),
        "hu" => HU.clone(),
        "is" => IS.clone(),
        "ie" => IE.clone(),
        "il" => IL.clone(),
        "it" => IT.clone(),
        "jo" => JO.clone(),
        "kz" => KZ.clone(),
        "xk" => XK.clone(),
        "kw" => KW.clone(),
        "lv" => LV.clone(),
        "lb" => LB.clone(),
        "li" => LI.clone(),
        "lt" => LT.clone(),
        "lu" => LU.clone(),
        "mk" => MK.clone(),
        "mt" => MT.clone(),
        "mr" => MR.clone(),
        "mu" => MU.clone(),
        "mc" => MC.clone(),
        "md" => MD.clone(),
        "me" => ME.clone(),
        "nl" => NL.clone(),
        "no" => NO.clone(),
        "pk" => PK.clone(),
        "ps" => PS.clone(),
        "pl" => PL.clone(),
        "pt" => PT.clone(),
        "qa" => QA.clone(),
        "ro" => RO.clone(),
        "sm" => SM.clone(),
        "sa" => SA.clone(),
        "rs" => RS.clone(),
        "sk" => SK.clone(),
        "si" => SI.clone(),
        "es" => ES.clone(),
        "se" => SE.clone(),
        "ch" => CH.clone(),
        "tn" => TN.clone(),
        "tr" => TR.clone(),
        "ae" => AE.clone(),
        "gb" => GB.clone(),
        "vg" => VG.clone(),
        _ => PL.clone(),
    }
}

pub static AL: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "AL".into(),
    total: 28,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 8,
        },
        Bban {
            r#type: "c".to_string(),
            count: 16,
        },
    ],
    format: "ALkk bbbs sssx cccc cccc cccc cccc".into(),
});

pub static AD: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "AD".into(),
    total: 24,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 8,
        },
        Bban {
            r#type: "c".to_string(),
            count: 12,
        },
    ],
    format: "ADkk bbbb ssss cccc cccc cccc".into(),
});

pub static AT: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "AT".into(),
    total: 20,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 5,
        },
        Bban {
            r#type: "n".to_string(),
            count: 11,
        },
    ],
    format: "ATkk bbbb bccc cccc cccc".into(),
});

pub static AZ: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "AZ".into(),
    total: 28,
    bban: vec![
        Bban {
            r#type: "a".to_string(),
            count: 4,
        },
        Bban {
            r#type: "n".to_string(),
            count: 20,
        },
    ],
    format: "AZkk bbbb cccc cccc cccc cccc cccc".into(),
});

pub static BH: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "BH".into(),
    total: 22,
    bban: vec![
        Bban {
            r#type: "a".to_string(),
            count: 4,
        },
        Bban {
            r#type: "c".to_string(),
            count: 14,
        },
    ],
    format: "BHkk bbbb cccc cccc cccc cc".into(),
});

pub static BE: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "BE".into(),
    total: 16,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 3,
        },
        Bban {
            r#type: "n".to_string(),
            count: 9,
        },
    ],
    format: "BEkk bbbc cccc ccxx".into(),
});

pub static BA: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "BA".into(),
    total: 20,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 6,
        },
        Bban {
            r#type: "n".to_string(),
            count: 10,
        },
    ],
    format: "BAkk bbbs sscc cccc ccxx".into(),
});

pub static BR: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "BR".into(),
    total: 29,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 13,
        },
        Bban {
            r#type: "n".to_string(),
            count: 10,
        },
        Bban {
            r#type: "a".to_string(),
            count: 1,
        },
        Bban {
            r#type: "c".to_string(),
            count: 1,
        },
    ],
    format: "BRkk bbbb bbbb ssss sccc cccc ccct n".into(),
});

pub static BG: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "BG".into(),
    total: 22,
    bban: vec![
        Bban {
            r#type: "a".to_string(),
            count: 4,
        },
        Bban {
            r#type: "n".to_string(),
            count: 6,
        },
        Bban {
            r#type: "c".to_string(),
            count: 8,
        },
    ],
    format: "BGkk bbbb ssss ddcc cccc cc".into(),
});

pub static CR: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "CR".into(),
    total: 22,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 1,
        },
        Bban {
            r#type: "n".to_string(),
            count: 3,
        },
        Bban {
            r#type: "n".to_string(),
            count: 14,
        },
    ],
    format: "CRkk xbbb cccc cccc cccc cc".into(),
});

pub static HR: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "HR".into(),
    total: 21,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 7,
        },
        Bban {
            r#type: "n".to_string(),
            count: 10,
        },
    ],
    format: "HRkk bbbb bbbc cccc cccc c".into(),
});

pub static CY: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "CY".into(),
    total: 28,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 8,
        },
        Bban {
            r#type: "c".to_string(),
            count: 16,
        },
    ],
    format: "CYkk bbbs ssss cccc cccc cccc cccc".into(),
});

pub static CZ: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "CZ".into(),
    total: 24,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 10,
        },
        Bban {
            r#type: "n".to_string(),
            count: 10,
        },
    ],
    format: "CZkk bbbb ssss sscc cccc cccc".into(),
});

pub static DK: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "DK".into(),
    total: 18,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 4,
        },
        Bban {
            r#type: "n".to_string(),
            count: 10,
        },
    ],
    format: "DKkk bbbb cccc cccc cc".into(),
});

pub static DO: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "DO".into(),
    total: 28,
    bban: vec![
        Bban {
            r#type: "a".to_string(),
            count: 4,
        },
        Bban {
            r#type: "n".to_string(),
            count: 20,
        },
    ],
    format: "DOkk bbbb cccc cccc cccc cccc cccc".into(),
});

pub static TL: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "TL".into(),
    total: 23,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 3,
        },
        Bban {
            r#type: "n".to_string(),
            count: 16,
        },
    ],
    format: "TLkk bbbc cccc cccc cccc cxx".into(),
});

pub static EE: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "EE".into(),
    total: 20,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 4,
        },
        Bban {
            r#type: "n".to_string(),
            count: 12,
        },
    ],
    format: "EEkk bbss cccc cccc cccx".into(),
});

pub static FO: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "FO".into(),
    total: 18,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 4,
        },
        Bban {
            r#type: "n".to_string(),
            count: 10,
        },
    ],
    format: "FOkk bbbb cccc cccc cx".into(),
});

pub static FI: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "FI".into(),
    total: 18,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 6,
        },
        Bban {
            r#type: "n".to_string(),
            count: 8,
        },
    ],
    format: "FIkk bbbb bbcc cccc cx".into(),
});

pub static FR: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "FR".into(),
    total: 27,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 10,
        },
        Bban {
            r#type: "c".to_string(),
            count: 11,
        },
        Bban {
            r#type: "n".to_string(),
            count: 2,
        },
    ],
    format: "FRkk bbbb bggg ggcc cccc cccc cxx".into(),
});

pub static GE: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "GE".into(),
    total: 22,
    bban: vec![
        Bban {
            r#type: "a".to_string(),
            count: 2,
        },
        Bban {
            r#type: "n".to_string(),
            count: 16,
        },
    ],
    format: "GEkk bbcc cccc cccc cccc cc".into(),
});

pub static DE: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "DE".into(),
    total: 22,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 8,
        },
        Bban {
            r#type: "n".to_string(),
            count: 10,
        },
    ],
    format: "DEkk bbbb bbbb cccc cccc cc".into(),
});

pub static GI: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "GI".into(),
    total: 23,
    bban: vec![
        Bban {
            r#type: "a".to_string(),
            count: 4,
        },
        Bban {
            r#type: "c".to_string(),
            count: 15,
        },
    ],
    format: "GIkk bbbb cccc cccc cccc ccc".into(),
});

pub static GR: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "GR".into(),
    total: 27,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 7,
        },
        Bban {
            r#type: "c".to_string(),
            count: 16,
        },
    ],
    format: "GRkk bbbs sssc cccc cccc cccc ccc".into(),
});

pub static GL: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "GL".into(),
    total: 18,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 4,
        },
        Bban {
            r#type: "n".to_string(),
            count: 10,
        },
    ],
    format: "GLkk bbbb cccc cccc cc".into(),
});

pub static GT: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "GT".into(),
    total: 28,
    bban: vec![
        Bban {
            r#type: "c".to_string(),
            count: 4,
        },
        Bban {
            r#type: "c".to_string(),
            count: 4,
        },
        Bban {
            r#type: "c".to_string(),
            count: 16,
        },
    ],
    format: "GTkk bbbb mmtt cccc cccc cccc cccc".into(),
});

pub static HU: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "HU".into(),
    total: 28,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 8,
        },
        Bban {
            r#type: "n".to_string(),
            count: 16,
        },
    ],
    format: "HUkk bbbs sssk cccc cccc cccc cccx".into(),
});

pub static IS: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "IS".into(),
    total: 26,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 6,
        },
        Bban {
            r#type: "n".to_string(),
            count: 16,
        },
    ],
    format: "ISkk bbbb sscc cccc iiii iiii ii".into(),
});

pub static IE: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "IE".into(),
    total: 22,
    bban: vec![
        Bban {
            r#type: "c".to_string(),
            count: 4,
        },
        Bban {
            r#type: "n".to_string(),
            count: 6,
        },
        Bban {
            r#type: "n".to_string(),
            count: 8,
        },
    ],
    format: "IEkk aaaa bbbb bbcc cccc cc".into(),
});

pub static IL: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "IL".into(),
    total: 23,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 6,
        },
        Bban {
            r#type: "n".to_string(),
            count: 13,
        },
    ],
    format: "ILkk bbbn nncc cccc cccc ccc".into(),
});

pub static IT: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "IT".into(),
    total: 27,
    bban: vec![
        Bban {
            r#type: "a".to_string(),
            count: 1,
        },
        Bban {
            r#type: "n".to_string(),
            count: 10,
        },
        Bban {
            r#type: "c".to_string(),
            count: 12,
        },
    ],
    format: "ITkk xaaa aabb bbbc cccc cccc ccc".into(),
});

pub static JO: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "JO".into(),
    total: 30,
    bban: vec![
        Bban {
            r#type: "a".to_string(),
            count: 4,
        },
        Bban {
            r#type: "n".to_string(),
            count: 4,
        },
        Bban {
            r#type: "n".to_string(),
            count: 18,
        },
    ],
    format: "JOkk bbbb nnnn cccc cccc cccc cccc cc".into(),
});

pub static KZ: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "KZ".into(),
    total: 20,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 3,
        },
        Bban {
            r#type: "c".to_string(),
            count: 13,
        },
    ],
    format: "KZkk bbbc cccc cccc cccc".into(),
});

pub static XK: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "XK".into(),
    total: 20,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 4,
        },
        Bban {
            r#type: "n".to_string(),
            count: 12,
        },
    ],
    format: "XKkk bbbb cccc cccc cccc".into(),
});

pub static KW: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "KW".into(),
    total: 30,
    bban: vec![
        Bban {
            r#type: "a".to_string(),
            count: 4,
        },
        Bban {
            r#type: "c".to_string(),
            count: 22,
        },
    ],
    format: "KWkk bbbb cccc cccc cccc cccc cccc cc".into(),
});

pub static LV: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "LV".into(),
    total: 21,
    bban: vec![
        Bban {
            r#type: "a".to_string(),
            count: 4,
        },
        Bban {
            r#type: "c".to_string(),
            count: 13,
        },
    ],
    format: "LVkk bbbb cccc cccc cccc c".into(),
});

pub static LB: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "LB".into(),
    total: 28,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 4,
        },
        Bban {
            r#type: "c".to_string(),
            count: 20,
        },
    ],
    format: "LBkk bbbb cccc cccc cccc cccc cccc".into(),
});

pub static LI: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "LI".into(),
    total: 21,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 5,
        },
        Bban {
            r#type: "c".to_string(),
            count: 12,
        },
    ],
    format: "LIkk bbbb bccc cccc cccc c".into(),
});

pub static LT: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "LT".into(),
    total: 20,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 5,
        },
        Bban {
            r#type: "n".to_string(),
            count: 11,
        },
    ],
    format: "LTkk bbbb bccc cccc cccc".into(),
});

pub static LU: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "LU".into(),
    total: 20,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 3,
        },
        Bban {
            r#type: "c".to_string(),
            count: 13,
        },
    ],
    format: "LUkk bbbc cccc cccc cccc".into(),
});

pub static MK: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "MK".into(),
    total: 19,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 3,
        },
        Bban {
            r#type: "c".to_string(),
            count: 10,
        },
        Bban {
            r#type: "n".to_string(),
            count: 2,
        },
    ],
    format: "MKkk bbbc cccc cccc cxx".into(),
});

pub static MT: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "MT".into(),
    total: 31,
    bban: vec![
        Bban {
            r#type: "a".to_string(),
            count: 4,
        },
        Bban {
            r#type: "n".to_string(),
            count: 5,
        },
        Bban {
            r#type: "c".to_string(),
            count: 18,
        },
    ],
    format: "MTkk bbbb ssss sccc cccc cccc cccc ccc".into(),
});

pub static MR: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "MR".into(),
    total: 27,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 10,
        },
        Bban {
            r#type: "n".to_string(),
            count: 13,
        },
    ],
    format: "MRkk bbbb bsss sscc cccc cccc cxx".into(),
});

pub static MU: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "MU".into(),
    total: 30,
    bban: vec![
        Bban {
            r#type: "a".to_string(),
            count: 4,
        },
        Bban {
            r#type: "n".to_string(),
            count: 4,
        },
        Bban {
            r#type: "n".to_string(),
            count: 15,
        },
        Bban {
            r#type: "a".to_string(),
            count: 3,
        },
    ],
    format: "MUkk bbbb bbss cccc cccc cccc 000d dd".into(),
});

pub static MC: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "MC".into(),
    total: 27,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 10,
        },
        Bban {
            r#type: "c".to_string(),
            count: 11,
        },
        Bban {
            r#type: "n".to_string(),
            count: 2,
        },
    ],
    format: "MCkk bbbb bsss sscc cccc cccc cxx".into(),
});

pub static MD: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "MD".into(),
    total: 24,
    bban: vec![
        Bban {
            r#type: "c".to_string(),
            count: 2,
        },
        Bban {
            r#type: "c".to_string(),
            count: 18,
        },
    ],
    format: "MDkk bbcc cccc cccc cccc cccc".into(),
});

pub static ME: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "ME".into(),
    total: 22,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 3,
        },
        Bban {
            r#type: "n".to_string(),
            count: 15,
        },
    ],
    format: "MEkk bbbc cccc cccc cccc xx".into(),
});

pub static NL: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "NL".into(),
    total: 18,
    bban: vec![
        Bban {
            r#type: "a".to_string(),
            count: 4,
        },
        Bban {
            r#type: "n".to_string(),
            count: 10,
        },
    ],
    format: "NLkk bbbb cccc cccc cc".into(),
});

pub static NO: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "NO".into(),
    total: 15,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 4,
        },
        Bban {
            r#type: "n".to_string(),
            count: 7,
        },
    ],
    format: "NOkk bbbb cccc ccx".into(),
});

pub static PK: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "PK".into(),
    total: 24,
    bban: vec![
        Bban {
            r#type: "a".to_string(),
            count: 4,
        },
        Bban {
            r#type: "n".to_string(),
            count: 16,
        },
    ],
    format: "PKkk bbbb cccc cccc cccc cccc".into(),
});

pub static PS: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "PS".into(),
    total: 29,
    bban: vec![
        Bban {
            r#type: "c".to_string(),
            count: 4,
        },
        Bban {
            r#type: "n".to_string(),
            count: 9,
        },
        Bban {
            r#type: "n".to_string(),
            count: 12,
        },
    ],
    format: "PSkk bbbb xxxx xxxx xccc cccc cccc c".into(),
});

pub static PL: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "PL".into(),
    total: 28,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 8,
        },
        Bban {
            r#type: "n".to_string(),
            count: 16,
        },
    ],
    format: "PLkk bbbs sssx cccc cccc cccc cccc".into(),
});

pub static PT: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "PT".into(),
    total: 25,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 8,
        },
        Bban {
            r#type: "n".to_string(),
            count: 13,
        },
    ],
    format: "PTkk bbbb ssss cccc cccc cccx x".into(),
});

pub static QA: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "QA".into(),
    total: 29,
    bban: vec![
        Bban {
            r#type: "a".to_string(),
            count: 4,
        },
        Bban {
            r#type: "c".to_string(),
            count: 21,
        },
    ],
    format: "QAkk bbbb cccc cccc cccc cccc cccc c".into(),
});

pub static RO: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "RO".into(),
    total: 24,
    bban: vec![
        Bban {
            r#type: "a".to_string(),
            count: 4,
        },
        Bban {
            r#type: "c".to_string(),
            count: 16,
        },
    ],
    format: "ROkk bbbb cccc cccc cccc cccc".into(),
});

pub static SM: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "SM".into(),
    total: 27,
    bban: vec![
        Bban {
            r#type: "a".to_string(),
            count: 1,
        },
        Bban {
            r#type: "n".to_string(),
            count: 10,
        },
        Bban {
            r#type: "c".to_string(),
            count: 12,
        },
    ],
    format: "SMkk xaaa aabb bbbc cccc cccc ccc".into(),
});

pub static SA: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "SA".into(),
    total: 24,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 2,
        },
        Bban {
            r#type: "c".to_string(),
            count: 18,
        },
    ],
    format: "SAkk bbcc cccc cccc cccc cccc".into(),
});

pub static RS: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "RS".into(),
    total: 22,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 3,
        },
        Bban {
            r#type: "n".to_string(),
            count: 15,
        },
    ],
    format: "RSkk bbbc cccc cccc cccc xx".into(),
});

pub static SK: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "SK".into(),
    total: 24,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 10,
        },
        Bban {
            r#type: "n".to_string(),
            count: 10,
        },
    ],
    format: "SKkk bbbb ssss sscc cccc cccc".into(),
});

pub static SI: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "SI".into(),
    total: 19,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 5,
        },
        Bban {
            r#type: "n".to_string(),
            count: 10,
        },
    ],
    format: "SIkk bbss sccc cccc cxx".into(),
});

pub static ES: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "ES".into(),
    total: 24,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 10,
        },
        Bban {
            r#type: "n".to_string(),
            count: 10,
        },
    ],
    format: "ESkk bbbb gggg xxcc cccc cccc".into(),
});

pub static SE: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "SE".into(),
    total: 24,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 3,
        },
        Bban {
            r#type: "n".to_string(),
            count: 17,
        },
    ],
    format: "SEkk bbbc cccc cccc cccc cccc".into(),
});

pub static CH: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "CH".into(),
    total: 21,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 5,
        },
        Bban {
            r#type: "c".to_string(),
            count: 12,
        },
    ],
    format: "CHkk bbbb bccc cccc cccc c".into(),
});

pub static TN: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "TN".into(),
    total: 24,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 5,
        },
        Bban {
            r#type: "n".to_string(),
            count: 15,
        },
    ],
    format: "TNkk bbss sccc cccc cccc cccc".into(),
});

pub static TR: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "TR".into(),
    total: 26,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 5,
        },
        Bban {
            r#type: "n".to_string(),
            count: 1,
        },
        Bban {
            r#type: "n".to_string(),
            count: 16,
        },
    ],
    format: "TRkk bbbb bxcc cccc cccc cccc cc".into(),
});

pub static AE: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "AE".into(),
    total: 23,
    bban: vec![
        Bban {
            r#type: "n".to_string(),
            count: 3,
        },
        Bban {
            r#type: "n".to_string(),
            count: 16,
        },
    ],
    format: "AEkk bbbc cccc cccc cccc ccc".into(),
});

pub static GB: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "GB".into(),
    total: 22,
    bban: vec![
        Bban {
            r#type: "a".to_string(),
            count: 4,
        },
        Bban {
            r#type: "n".to_string(),
            count: 6,
        },
        Bban {
            r#type: "n".to_string(),
            count: 8,
        },
    ],
    format: "GBkk bbbb ssss sscc cccc cc".into(),
});

pub static VG: Lazy<Iban> = once_cell::sync::Lazy::new(|| Iban {
    country: "VG".into(),
    total: 24,
    bban: vec![
        Bban {
            r#type: "a".to_string(),
            count: 4,
        },
        Bban {
            r#type: "n".to_string(),
            count: 16,
        },
    ],
    format: "VGkk bbbb cccc cccc cccc cccc".into(),
});
