//! Defines the `Federation` field for the `meets` table.

use std::fmt;

#[derive(Copy, Clone, Deserialize, PartialEq, Serialize, EnumIterator)]
pub enum Federation {
    #[serde(rename = "365Strong")]
    _365Strong,
    AAPF,
    AAU,
    ADFPA,
    APA,
    APC,
    APF,
    AsianPF,
    BB,
    BPU,
    BP,
    CAPO,
    CommonwealthPF,
    CPF,
    CPL,
    CPU,
    EPA,
    EPF,
    FEMEPO,
    FESUPO,
    FFForce,
    FPO,
    GPA,
    GPC,
    #[serde(rename = "GPC-AUS")]
    GPCAUS,
    #[serde(rename = "GPC-GB")]
    GPCGB,
    #[serde(rename = "GPC-NZ")]
    GPCNZ,
    HERC,
    IDFPF,
    IPA,
    IPF,
    IPL,
    IrishPF,
    MHP,
    MM,
    NAPF,
    NASA,
    NIPF,
    NPA,
    NSF,
    NZPF,
    OceaniaPF,
    ParaPL,
    PA,
    ProRaw,
    RAW,
    RAWU,
    RPS,
    RUPC,
    ScottishPL,
    SCT,
    SPA,
    SPF,
    THSPA,
    UPA,
    USAPL,
    USPF,
    USPA,
    WABDL,
    WDFPF,
    WelshPA,
    WPC,
    WNPF,
    WRPF,
    #[serde(rename = "WRPF-AUS")]
    WRPFAUS,
    #[serde(rename = "WRPF-CAN")]
    WRPFCAN,
    WUAP,
    XPC,
}

impl fmt::Display for Federation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Federation::_365Strong => write!(f, "365Strong"),
            Federation::AAPF => write!(f, "AAPF"),
            Federation::AAU => write!(f, "AAU"),
            Federation::ADFPA => write!(f, "ADFPA"),
            Federation::APA => write!(f, "APA"),
            Federation::APC => write!(f, "APC"),
            Federation::APF => write!(f, "APF"),
            Federation::AsianPF => write!(f, "AsianPF"),
            Federation::BB => write!(f, "BB"),
            Federation::BPU => write!(f, "BPU"),
            Federation::BP => write!(f, "BP"),
            Federation::CAPO => write!(f, "CAPO"),
            Federation::CommonwealthPF => write!(f, "CommonwealthPF"),
            Federation::CPF => write!(f, "CPF"),
            Federation::CPL => write!(f, "CPL"),
            Federation::CPU => write!(f, "CPU"),
            Federation::EPA => write!(f, "EPA"),
            Federation::EPF => write!(f, "EPF"),
            Federation::FEMEPO => write!(f, "FEMEPO"),
            Federation::FESUPO => write!(f, "FESUPO"),
            Federation::FFForce => write!(f, "FFForce"),
            Federation::FPO => write!(f, "FPO"),
            Federation::GPA => write!(f, "GPA"),
            Federation::GPC => write!(f, "GPC"),
            Federation::GPCGB => write!(f, "GPC-GB"),
            Federation::GPCAUS => write!(f, "GPC-AUS"),
            Federation::GPCNZ => write!(f, "GPC-NZ"),
            Federation::HERC => write!(f, "HERC"),
            Federation::IDFPF => write!(f, "IDFPF"),
            Federation::IPA => write!(f, "IPA"),
            Federation::IPF => write!(f, "IPF"),
            Federation::IPL => write!(f, "IPL"),
            Federation::IrishPF => write!(f, "IrishPF"),
            Federation::MHP => write!(f, "MHP"),
            Federation::MM => write!(f, "MM"),
            Federation::NAPF => write!(f, "NAPF"),
            Federation::NASA => write!(f, "NASA"),
            Federation::NIPF => write!(f, "NIPF"),
            Federation::NPA => write!(f, "NPA"),
            Federation::NSF => write!(f, "NSF"),
            Federation::NZPF => write!(f, "NZPF"),
            Federation::OceaniaPF => write!(f, "OceaniaPF"),
            Federation::ParaPL => write!(f, "ParaPL"),
            Federation::PA => write!(f, "PA"),
            Federation::ProRaw => write!(f, "ProRaw"),
            Federation::RAW => write!(f, "RAW"),
            Federation::RAWU => write!(f, "RAWU"),
            Federation::RPS => write!(f, "RPS"),
            Federation::RUPC => write!(f, "RUPC"),
            Federation::ScottishPL => write!(f, "ScottishPL"),
            Federation::SCT => write!(f, "SCT"),
            Federation::SPA => write!(f, "SPA"),
            Federation::SPF => write!(f, "SPF"),
            Federation::THSPA => write!(f, "THSPA"),
            Federation::UPA => write!(f, "UPA"),
            Federation::USAPL => write!(f, "USAPL"),
            Federation::USPF => write!(f, "USPF"),
            Federation::USPA => write!(f, "USPA"),
            Federation::WABDL => write!(f, "WABDL"),
            Federation::WDFPF => write!(f, "WDFPF"),
            Federation::WelshPA => write!(f, "WelshPA"),
            Federation::WPC => write!(f, "WPC"),
            Federation::WNPF => write!(f, "WNPF"),
            Federation::WRPF => write!(f, "WRPF"),
            Federation::WRPFAUS => write!(f, "WRPF-AUS"),
            Federation::WRPFCAN => write!(f, "WRPF-CAN"),
            Federation::WUAP => write!(f, "WUAP"),
            Federation::XPC => write!(f, "XPC"),
        }
    }
}
