//! Defines the `Federation` field for the `meets` table.

use crate::Country;
use crate::Date;
use crate::PointsSystem;

/// Enum of federations.
///
/// `Display` derivation provided by strum.
/// `EnumString` (`FromStr`) derivation provided by strum.
///
/// Note that the deserialization source string (as in the CSV data)
/// may differ from the FromStr source string, which comes from a URL
/// and is generally lowercase.
///
/// The strum `to_string` value defines the default .to_string() result,
/// while *all* of to_string and serialize are parseable.
/// So Federation::APF can be parsed from the strings "APF" and "apf".
#[derive(
    Copy,
    Clone,
    Debug,
    Deserialize,
    Display,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
    Serialize,
    EnumIter,
    EnumString,
)]
pub enum Federation {
    /// 365 Strong Powerlifting Federation.
    #[serde(rename = "365Strong")]
    #[strum(to_string = "365Strong", serialize = "365strong")]
    _365Strong,

    /// Alianza Argentina Powerlifting, GPA/IPO.
    #[strum(to_string = "AAP", serialize = "aap")]
    AAP,

    /// Australian Amateur Powerlifting Federation, defunct IPF affiliate.
    #[strum(to_string = "AAPLF", serialize = "aaplf")]
    AAPLF,

    /// Amateur Athletic Union.
    #[strum(to_string = "AAU", serialize = "aau")]
    AAU,

    /// Alianza Chilena Powerlifting, GPA/IPO.
    #[strum(to_string = "ACHIPO", serialize = "achipo")]
    ACHIPO,

    /// Anti-Drug Athletes United.
    #[strum(to_string = "ADAU", serialize = "adau")]
    ADAU,

    /// American Drug-Free Powerlifting Association, predecessor of USAPL.
    #[strum(to_string = "ADFPA", serialize = "adfpa")]
    ADFPA,

    /// American Drug-Free Powerlifting Federation, WDFPF.
    #[strum(to_string = "ADFPF", serialize = "adfpf")]
    ADFPF,

    /// Asociación Española de Powerlifting, IPF.
    #[strum(to_string = "AEP", serialize = "aep")]
    AEP,

    /// American Frantz Powerlifting Federation
    #[strum(to_string = "AFPF", serialize = "afpf")]
    AFPF,

    /// African Powerlifting Federation, IPF.
    #[strum(to_string = "AfricanPF", serialize = "africanpf")]
    AfricanPF,

    /// All Indonesia Weightlifting, Bodybuilding and Powerlifting
    /// Association, IPF.
    #[strum(to_string = "AIWBPA", serialize = "aiwbpa")]
    AIWBPA,

    /// American Strength Association, an unaffiliated local federation
    /// created to avoid membership fees for local competitions.
    #[strum(to_string = "AmericanSA", serialize = "americansa")]
    AmericanSA,

    /// American Powerlifting Association, WPA.
    #[strum(to_string = "APA", serialize = "apa")]
    APA,

    /// American Powerlifting Committee.
    #[strum(to_string = "APC", serialize = "apc")]
    APC,

    /// American Powerlifting Federation.
    #[strum(to_string = "APF", serialize = "apf")]
    APF,

    /// Australian Powerlifting Union.
    #[strum(to_string = "APU", serialize = "apu")]
    APU,

    /// Asian Powerlifting Federation, IPF.
    #[strum(to_string = "AsianPF", serialize = "asianpf")]
    AsianPF,

    /// Series of Raw meets hosted by the Atlantis company.
    #[strum(to_string = "Atlantis", serialize = "atlantis")]
    Atlantis,

    /// Australian Drug-Free Powerlifting Federation, WDFPF.
    #[strum(to_string = "AusDFPF", serialize = "ausdfpf")]
    AusDFPF,

    /// Australian Powerlifting League, IPL.
    #[strum(to_string = "AusPL", serialize = "auspl")]
    AusPL,

    /// British Amateur Weightlifting Association, predecessor to BP.
    #[strum(to_string = "BAWLA", serialize = "bawla")]
    BAWLA,

    /// Bogatyr Brotherhood, a stand-alone and short-lived Russian federation.
    #[strum(to_string = "BB", serialize = "bb")]
    BB,

    /// Baddest Bench, Deadliest Deadlift. Yearly meets run by John Inzer.
    #[strum(to_string = "BBDD", serialize = "bbdd")]
    BBDD,

    /// British Drug-Free Powerlifting Assocation.
    #[strum(to_string = "BDFPA", serialize = "bdfpa")]
    BDFPA,

    /// Defunct British WPC affiliate.
    #[strum(to_string = "BPC", serialize = "bpc")]
    BPC,

    /// British WPU affiliate.
    #[strum(to_string = "BPF", serialize = "bpf")]
    BPF,

    /// British Powerlifting Organization, WPF.
    #[strum(to_string = "BPO", serialize = "bpo")]
    BPO,

    /// British Powerlifting Union.
    #[strum(to_string = "BPU", serialize = "bpu")]
    BPU,

    /// British Powerlifting, IPF. Formerly named GBPF.
    #[strum(to_string = "BP", serialize = "bp")]
    BP,

    /// Bundesverband Deutscher Kraftdreikämpf, IPF.
    #[strum(to_string = "BVDK", serialize = "bvdk")]
    BVDK,

    /// Australian WPC/GPA affiliate.
    #[strum(to_string = "CAPO", serialize = "capo")]
    CAPO,

    /// Shortlived NZ branch of CAPO.
    #[serde(rename = "CAPO-NZ")]
    #[strum(to_string = "CAPO-NZ", serialize = "capo-nz")]
    CAPONZ,

    /// Česká Asociace Silového Trojboje, GPC/WPC.
    #[strum(to_string = "CAST", serialize = "cast")]
    CAST,

    /// Chinese Powerlifting Association, GPA.
    #[strum(to_string = "ChinaPA", serialize = "chinapa")]
    ChinaPA,

    /// Commonwealth Powerlifting Federation, IPF.
    #[strum(to_string = "CommonwealthPF", serialize = "commonwealthpf")]
    CommonwealthPF,

    /// Canadian Powerlifting Congress, WPC.
    #[strum(to_string = "CPC", serialize = "cpc")]
    CPC,

    /// Canadian Powerlifting Federation, WPF.
    #[strum(to_string = "CPF", serialize = "cpf")]
    CPF,

    /// Canadian Powerlifting League, IPL.
    #[strum(to_string = "CPL", serialize = "cpl")]
    CPL,

    /// Canadian Powerlifting Organization, defunct WPC affiliate.
    #[strum(to_string = "CPO", serialize = "cpo")]
    CPO,

    /// Canadian Powerlifting Union, IPF.
    #[strum(to_string = "CPU", serialize = "cpu")]
    CPU,

    /// Český svaz silového trojboje, Czech IPF affiliate.
    #[strum(to_string = "CSST", serialize = "csst")]
    CSST,

    /// Danish IPF affiliate.
    #[strum(to_string = "DSF", serialize = "dsf")]
    DSF,

    /// English Powerlifting Association, IPF.
    #[strum(to_string = "EPA", serialize = "epa")]
    EPA,

    /// European Powerlifting Federation, IPF.
    #[strum(to_string = "EPF", serialize = "epf")]
    EPF,

    /// Federación Argentina de Levantamiento de Potencia, IPF.
    #[strum(to_string = "FALPO", serialize = "falpo")]
    FALPO,

    /// Federace českého silového trojboje, GPC.
    #[strum(to_string = "FCST", serialize = "fcst")]
    FCST,

    /// Federación Mexicana de Powerlifting A.C., IPF.
    #[strum(to_string = "FEMEPO", serialize = "femepo")]
    FEMEPO,

    /// Federación de Powerlifting Argentino, GPC.
    #[strum(to_string = "FEPOA", serialize = "fepoa")]
    FEPOA,

    /// Federación Sudamericana de Powerlifting, IPF.
    #[strum(to_string = "FESUPO", serialize = "fesupo")]
    FESUPO,

    /// Federation Francaise de Force, IPF.
    #[strum(to_string = "FFForce", serialize = "ffforce")]
    FFForce,

    /// Finland Powerlifting Organization, IPA.
    #[strum(to_string = "FPO", serialize = "fpo")]
    FPO,

    /// Powerlifting Federation of Russia, IPF.
    #[strum(to_string = "FPR", serialize = "fpr")]
    FPR,

    /// Russian stand-alone competition.
    #[strum(to_string = "GoldenDouble", serialize = "goldendouble")]
    GoldenDouble,

    /// Global Powerlifting Association.
    #[strum(to_string = "GPA", serialize = "gpa")]
    GPA,

    /// Croatian branch of the GPA.
    #[serde(rename = "GPA-CRO")]
    #[strum(to_string = "GPA-CRO", serialize = "gpa-cro")]
    GPACRO,

    /// Global Powerlifting Committee.
    #[strum(to_string = "GPC", serialize = "gpc")]
    GPC,

    /// Australian branch of the GPC.
    #[serde(rename = "GPC-AUS")]
    #[strum(to_string = "GPC-AUS", serialize = "gpc-aus")]
    GPCAUS,

    /// British branch of the GPC.
    #[serde(rename = "GPC-GB")]
    #[strum(to_string = "GPC-GB", serialize = "gpc-gb")]
    GPCGB,

    /// Irish branch of the GPC.
    #[serde(rename = "GPC-IRL")]
    #[strum(to_string = "GPC-IRL", serialize = "gpc-irl")]
    GPCIRL,

    /// Latvian branch of the GPC.
    #[serde(rename = "GPC-LAT")]
    #[strum(to_string = "GPC-LAT", serialize = "gpc-lat")]
    GPCLAT,

    /// New Zealand branch of the GPC.
    #[serde(rename = "GPC-NZ")]
    #[strum(to_string = "GPC-NZ", serialize = "gpc-nz")]
    GPCNZ,

    /// Russian branch of the GPC.
    #[serde(rename = "GPC-RUS")]
    #[strum(to_string = "GPC-RUS", serialize = "gpc-rus")]
    GPCRUS,

    /// Global Powerlifting Federation
    #[strum(to_string = "GPF", serialize = "gpf")]
    GPF,

    /// German Powerlifting Union, WPU.
    #[strum(to_string = "GPU", serialize = "gpu")]
    GPU,

    /// Defunct stand-alone US federation.
    #[strum(to_string = "Hardcore", serialize = "hardcore")]
    Hardcore,

    /// Hercules Gym in Syracuse, NY. Run by Rheta West.
    #[strum(to_string = "HERC", serialize = "herc")]
    HERC,

    /// Croatia - Unaffiliated
    #[serde(rename = "Croatia-UA")]
    #[strum(to_string = "Croatia-UA", serialize = "croatia-ua")]
    CroatiaUA,

    /// Croatian IPF affiliate
    #[strum(to_string = "HPLS", serialize = "hpls")]
    HPLS,

    /// Croatian Powerlifting Federation before getting affiliated with IPF
    #[serde(rename = "HPLS-UA")]
    #[strum(to_string = "HPLS-UA", serialize = "hpls-ua")]
    HPLSUA,

    /// Croatian Powerlifting Organization
    #[strum(to_string = "HPO", serialize = "hpo")]
    HPO,

    /// International Blind Sport Assocation.
    #[strum(to_string = "IBSA", serialize = "ibsa")]
    IBSA,

    /// Irish Drug-Free Powerlifting Association.
    #[strum(to_string = "IDFPA", serialize = "idfpa")]
    IDFPA,

    /// Irish Drug-Free Powerlifting Federation.
    #[strum(to_string = "IDFPF", serialize = "idfpf")]
    IDFPF,

    /// A Canadian federation.
    #[strum(to_string = "IndependentPA", serialize = "independentpa")]
    IndependentPA,

    #[strum(to_string = "IPA", serialize = "ipa")]
    IPA,

    /// Israel Powerlifting Community.
    #[strum(to_string = "IPC", serialize = "ipc")]
    IPC,

    /// International Powerlifting Federation.
    #[strum(to_string = "IPF", serialize = "ipf")]
    IPF,

    /// International Powerlifting League.
    #[strum(to_string = "IPL", serialize = "ipl")]
    IPL,

    /// International Powerlifting League, New Zealand
    #[serde(rename = "IPL-NZ")]
    #[strum(to_string = "IPL-NZ", serialize = "ipl-nz")]
    IPLNZ,

    /// Irish Powerlifting Federation, IPF.
    #[strum(to_string = "IrishPF", serialize = "irishpf")]
    IrishPF,

    /// Irish Powerlifting Organization, WPU/IPL.
    #[strum(to_string = "IrishPO", serialize = "irishpo")]
    IrishPO,

    /// International RAW Powerlifting
    #[strum(to_string = "IRP", serialize = "irp")]
    IRP,
    /// Japan Powerlifting Federation, IPF.
    #[strum(to_string = "JPA", serialize = "jpa")]
    JPA,

    /// Kazakhstan IPF affiliate.
    #[strum(to_string = "KPF", serialize = "kpf")]
    KPF,

    /// Icelandic IPF affiliate.
    #[strum(to_string = "KRAFT", serialize = "kraft")]
    KRAFT,

    /// Latvian IPF affiliate.
    #[strum(to_string = "LPF", serialize = "lpf")]
    LPF,

    /// Maximum Human Performance, a vitamin company.
    #[strum(to_string = "MHP", serialize = "mhp")]
    MHP,

    /// Metal Militia, a small, independent federation.
    #[strum(to_string = "MM", serialize = "mm")]
    MM,

    /// Malaysian Powerlifting Alliance.
    #[strum(to_string = "MPA", serialize = "mpa")]
    MPA,

    /// National Association of Powerlifting Russia, IPA.
    #[strum(to_string = "NAP", serialize = "nap")]
    NAP,

    /// North American Powerlifting Federation, IPF.
    #[strum(to_string = "NAPF", serialize = "napf")]
    NAPF,

    /// Natural Athlete Strength Assocation.
    #[strum(to_string = "NASA", serialize = "nasa")]
    NASA,

    /// Northern Ireland Powerlifting Federation.
    #[strum(to_string = "NIPF", serialize = "nipf")]
    NIPF,

    /// NORCAL Powerlifting Federation
    #[strum(to_string = "NORCAL", serialize = "norcal")]
    NORCAL,

    /// Nordic Powerlifting Federation, IPF.
    #[strum(to_string = "NordicPF", serialize = "nordicpf")]
    NordicPF,

    /// Night of the Living Deadlift.
    #[strum(to_string = "NOTLD", serialize = "notld")]
    NOTLD,

    /// National Powerlifting Association of Israel.
    #[strum(to_string = "NPA", serialize = "npa")]
    NPA,

    /// Norwegian IPF affiliate.
    #[strum(to_string = "NSF", serialize = "nsf")]
    NSF,

    /// New Zealand Open competition, standalone.
    #[strum(to_string = "NZOpen", serialize = "nzopen")]
    NZOpen,

    /// New Zealand Powerlifting Federation, IPF.
    #[strum(to_string = "NZPF", serialize = "nzpf")]
    NZPF,

    /// Oceania Powerlifting Federation, IPF.
    #[strum(to_string = "OceaniaPF", serialize = "oceaniapf")]
    OceaniaPF,

    /// Olomoucký Silák, a yearly bench competition in Czechia.
    #[strum(to_string = "OlomouckySilak", serialize = "olomouckysilak")]
    OlomouckySilak,

    /// Paralympic Powerlifting.
    #[strum(to_string = "ParaPL", serialize = "parapl")]
    ParaPL,

    /// Powerlifting Australia, formerly IPF, now WP.
    #[strum(to_string = "PA", serialize = "pa")]
    PA,

    /// Powerlifting Association of the Philippines, IPF.
    #[strum(to_string = "PAP", serialize = "pap")]
    PAP,

    /// Powerlifting zveza Slovenije, IPF.
    #[strum(to_string = "PLZS", serialize = "plzs")]
    PLZS,

    /// A defunct stand-alone US federation.
    #[strum(to_string = "PRIDE", serialize = "pride")]
    PRIDE,

    /// Australian stand-alone meets run by Markos Markopoulos.
    #[strum(to_string = "ProRaw", serialize = "proraw")]
    ProRaw,

    /// Professional Raw Powerlifting Assocation.
    #[strum(to_string = "PRPA", serialize = "prpa")]
    PRPA,

    /// Polish IPF affiliate.
    #[strum(to_string = "PZKFiTS", serialize = "pzkfits")]
    PZKFiTS,

    /// 100% RAW Federation, WP.
    #[strum(to_string = "RAW", serialize = "100raw")]
    RAW,

    /// 100% RAW Federation Canada.
    #[serde(rename = "RAW-CAN")]
    #[strum(to_string = "RAW-CAN", serialize = "raw-can")]
    RAWCAN,

    /// 100% RAW Federation Ukraine.
    #[serde(rename = "RAW-UKR")]
    #[strum(to_string = "RAW-UKR", serialize = "raw-ukr")]
    RAWUKR,

    /// Raw United Federation
    #[strum(to_string = "RAWU", serialize = "rawu")]
    RAWU,

    /// Revolution Powerlifting Syndicate.
    #[strum(to_string = "RPS", serialize = "rps")]
    RPS,

    /// Russian Powerlifting Union.
    #[strum(to_string = "RPU", serialize = "rpu")]
    RPU,

    /// Raw Unity.
    #[strum(to_string = "RUPC", serialize = "rupc")]
    RUPC,

    /// Slovenská asociásia silového trojboja, Slovakian GPC Affiliate.
    #[strum(to_string = "SAST", serialize = "sast")]
    SAST,

    /// Scottish Powerlifting, IPF.
    #[strum(to_string = "ScottishPL", serialize = "scottishpl")]
    ScottishPL,

    /// Old US based federation, no idea what this stands for.
    #[strum(to_string = "SCI", serialize = "sci")]
    SCI,

    /// Super-Cup of Titans, a defunct Russian single-ply meet.
    #[strum(to_string = "SCT", serialize = "sct")]
    SCT,

    /// Singapore Powerlifting Alliance.
    #[strum(to_string = "SPA", serialize = "spa")]
    SPA,

    /// Southern Powerlifting Federation.
    #[strum(to_string = "SPF", serialize = "spf")]
    SPF,

    /// Societatem Potentis Species Sports, a defunct Russian raw federation.
    #[strum(to_string = "SPSS", serialize = "spss")]
    SPSS,

    /// Syndicated Strength Alliance.
    #[strum(to_string = "SSA", serialize = "ssa")]
    SSA,

    /// Swedish IPF affiliate.
    #[strum(to_string = "SSF", serialize = "ssf")]
    SSF,

    /// Finnish IPF affiliate.
    #[strum(to_string = "SVNL", serialize = "svnl")]
    SVNL,

    /// Thai IPF affiliate.
    #[strum(to_string = "ThaiPF", serialize = "thaipf")]
    ThaiPF,

    /// Texas High School Powerlifting Association.
    #[strum(to_string = "THSPA", serialize = "thspa")]
    THSPA,

    /// Ukraine Powerlifting Association.
    #[strum(to_string = "UkrainePA", serialize = "ukrainepa")]
    UkrainePA,

    /// Ukraine Powerlifting Organisation.
    #[strum(to_string = "UkrainePO", serialize = "ukrainepo")]
    UkrainePO,

    /// United Powerlifting Association.
    #[strum(to_string = "UPA", serialize = "upa")]
    UPA,

    /// Ukrainian Powerlifting Committee.
    #[strum(to_string = "UPC", serialize = "upc")]
    UPC,

    /// Ukrainian Powerlifting Federation, IPF.
    #[strum(to_string = "UkrainePF", serialize = "ukrainepf")]
    UkrainePF,

    /// USA Powerlifting, IPF.
    #[strum(to_string = "USAPL", serialize = "usapl")]
    USAPL,

    /// Ujedinjeni Srpski powerlifting savez.
    #[strum(to_string = "USPS", serialize = "usps")]
    USPS,

    /// US Powerlifting Federation.
    #[strum(to_string = "USPF", serialize = "uspf")]
    USPF,

    /// United States Powerlifting Assocation, IPL.
    #[strum(to_string = "USPA", serialize = "uspa")]
    USPA,

    /// Unified Strength Sports Federation
    #[strum(to_string = "USSF", serialize = "ussf")]
    USSF,

    /// Vietnam Powerlifting Alliance, GPA.
    #[strum(to_string = "VietnamPA", serialize = "vietnampa")]
    VietnamPA,

    #[strum(to_string = "Vityaz", serialize = "vityaz")]
    Vityaz,

    /// World Association of Bench Pressers and Deadlifters.
    #[strum(to_string = "WABDL", serialize = "wabdl")]
    WABDL,

    /// Not sure what this stands for, Anthony Clark set a bench record in this fed.
    #[strum(to_string = "WBC", serialize = "wbc")]
    WBC,


    /// World Drug-Free Powerlifting Association.
    #[strum(to_string = "WDFPF", serialize = "wdfpf")]
    WDFPF,

    /// Welsh Powerlifting Association, IPF.
    #[strum(to_string = "WelshPA", serialize = "welshpa")]
    WelshPA,

    /// World Powerlifting, Robert Wilks' federation.
    #[strum(to_string = "WP", serialize = "wp")]
    WP,

    /// World Powerlifting Alliance.
    #[strum(to_string = "WPA", serialize = "wpa")]
    WPA,

    /// World Powerlifting Alliance Russia.
    #[serde(rename = "WPA-RUS")]
    #[strum(to_string = "WPA-RUS", serialize = "wpa-rus")]
    WPARUS,

    /// World Powerlifting Alliance Ukraine.
    #[strum(to_string = "WPAU", serialize = "wpau")]
    WPAU,

    /// World Powerlifting Committee.
    #[strum(to_string = "WPC", serialize = "wpc")]
    WPC,

    /// German WPC affiliate.
    #[serde(rename = "WPC-Germany")]
    #[strum(to_string = "WPC-Germany", serialize = "wpc-germany")]
    WPCGermany,

    /// Latvian WPC affiliate.
    #[serde(rename = "WPC-Latvia")]
    #[strum(to_string = "WPC-Latvia", serialize = "wpc-latvia")]
    WPCLatvia,

    /// Moldovan WPC affiliate.
    #[serde(rename = "WPC-Moldova")]
    #[strum(to_string = "WPC-Moldova", serialize = "wpc-moldova")]
    WPCMoldova,

    /// Portuguese WPC affiliate.
    #[serde(rename = "WPC-Portugal")]
    #[strum(to_string = "WPC-Portugal", serialize = "wpc-portugal")]
    WPCPortugal,

    /// Russian WPC affiliate.
    #[serde(rename = "WPC-RUS")]
    #[strum(to_string = "WPC-RUS", serialize = "wpc-rus")]
    WPCRUS,

    /// Ukrainian WPC affiliate.
    #[serde(rename = "WPC-UKR")]
    #[strum(to_string = "WPC-UKR", serialize = "wpc-ukr")]
    WPCUKR,

    /// World Powerlifting Federation.
    #[strum(to_string = "WPF", serialize = "wpf")]
    WPF,

    /// World Powerlifting Union.
    #[strum(to_string = "WPU", serialize = "wpu")]
    WPU,

    /// World Powerlifting Union of Federations.
    #[strum(to_string = "WPUF", serialize = "wpuf")]
    WPUF,

    /// World Natural Powerlifting Federation.
    #[strum(to_string = "WNPF", serialize = "wnpf")]
    WNPF,

    /// World Raw Powerlifting Federation.
    #[strum(to_string = "WRPF", serialize = "wrpf")]
    WRPF,

    /// Australian WRPF affiliate.
    #[serde(rename = "WRPF-AUS")]
    #[strum(to_string = "WRPF-AUS", serialize = "wrpf-aus")]
    WRPFAUS,

    /// Canadian WRPF affiliate.
    #[serde(rename = "WRPF-CAN")]
    #[strum(to_string = "WRPF-CAN", serialize = "wrpf-can")]
    WRPFCAN,

    /// World United Amateur Powerlifting.
    #[strum(to_string = "WUAP", serialize = "wuap")]
    WUAP,

    /// Austrian WUAP affiliate.
    #[serde(rename = "WUAP-AUT")]
    #[strum(to_string = "WUAP-AUT", serialize = "wuap-aut")]
    WUAPAUT,

    /// Xtreme Powerlifting Coalition.
    #[strum(to_string = "XPC", serialize = "xpc")]
    XPC,
}

impl Federation {
    /// True iff every division in the federation is drug-tested.
    pub fn is_fully_tested(self) -> bool {
        match self {
            Federation::_365Strong => false,
            Federation::AAP => false,
            Federation::AAPLF => true,
            Federation::AAU => true,
            Federation::ACHIPO => false,
            Federation::ADAU => true,
            Federation::ADFPA => true,
            Federation::ADFPF => true,
            Federation::AEP => true,
            Federation::AFPF => false,
            Federation::AfricanPF => true,
            Federation::AIWBPA => true,
            Federation::AmericanSA => false,
            Federation::APA => false,
            Federation::APC => false,
            Federation::APF => false,
            Federation::APU => true,
            Federation::AsianPF => true,
            Federation::Atlantis => false,
            Federation::AusDFPF => true,
            Federation::AusPL => false,
            Federation::BAWLA => true,
            Federation::BB => false,
            Federation::BBDD => false,
            Federation::BDFPA => true,
            Federation::BPC => false,
            Federation::BPF => false,
            Federation::BPO => false,
            Federation::BPU => false,
            Federation::BP => true,
            Federation::BVDK => true,
            Federation::CAPO => false,
            Federation::CAPONZ => false,
            Federation::CAST => false,
            Federation::ChinaPA => false,
            Federation::CommonwealthPF => true,
            Federation::CPC => false,
            Federation::CPF => false,
            Federation::CPL => false,
            Federation::CPO => false,
            Federation::CPU => true,
            Federation::CSST => true,
            Federation::DSF => true,
            Federation::EPA => true,
            Federation::EPF => true,
            Federation::FALPO => true,
            Federation::FCST => false,
            Federation::FEMEPO => true,
            Federation::FEPOA => false,
            Federation::FESUPO => true,
            Federation::FFForce => true,
            Federation::FPO => false,
            Federation::FPR => true,
            Federation::GoldenDouble => false,
            Federation::GPA => false,
            Federation::GPACRO => false,
            Federation::GPC => false,
            Federation::GPCAUS => false,
            Federation::GPCGB => false,
            Federation::GPCIRL => false,
            Federation::GPCLAT => false,
            Federation::GPCNZ => false,
            Federation::GPCRUS => false,
            Federation::GPF => false,
            Federation::GPU => false,
            Federation::Hardcore => false,
            Federation::HERC => false,
            Federation::CroatiaUA => false,
            Federation::HPLS => true,
            Federation::HPLSUA => false,
            Federation::HPO => false,
            Federation::IBSA => true,
            Federation::IDFPA => true,
            Federation::IDFPF => true,
            Federation::IndependentPA => false,
            Federation::IPA => false,
            Federation::IPC => false,
            Federation::IPF => true,
            Federation::IPL => false,
            Federation::IPLNZ => false,
            Federation::IrishPF => true,
            Federation::IrishPO => false,
            Federation::IRP => false,
            Federation::JPA => true,
            Federation::KRAFT => true,
            Federation::KPF => true,
            Federation::LPF => true,
            Federation::MHP => false,
            Federation::MM => false,
            Federation::MPA => false,
            Federation::NAP => false,
            Federation::NAPF => true,
            Federation::NASA => true,
            Federation::NORCAL => true,
            Federation::NIPF => true,
            Federation::NordicPF => true,
            Federation::NOTLD => false,
            Federation::NPA => false,
            Federation::NSF => true,
            Federation::NZOpen => false,
            Federation::NZPF => true,
            Federation::OceaniaPF => true,
            Federation::OlomouckySilak => false,
            Federation::ParaPL => true,
            Federation::PA => true,
            Federation::PAP => true,
            Federation::PLZS => true,
            Federation::PRIDE => false,
            Federation::ProRaw => false,
            Federation::PRPA => false,
            Federation::PZKFiTS => true,
            Federation::RAW => true,
            Federation::RAWCAN => true,
            Federation::RAWUKR => true,
            Federation::RAWU => false,
            Federation::RPS => false,
            Federation::RPU => false,
            Federation::RUPC => false,
            Federation::SAST => false,
            Federation::ScottishPL => true,
            Federation::SCI => false,
            Federation::SCT => false,
            Federation::SPA => false,
            Federation::SPF => false,
            Federation::SPSS => false,
            Federation::SSA => false,
            Federation::SSF => true,
            Federation::SVNL => true,
            Federation::ThaiPF => true,
            Federation::THSPA => true,
            Federation::UkrainePA => false,
            Federation::UkrainePO => false,
            Federation::UPA => false,
            Federation::UPC => false,
            Federation::UkrainePF => true,
            Federation::USAPL => true,
            Federation::USPS => false,
            Federation::USPF => false,
            Federation::USPA => false,
            Federation::USSF => false,
            Federation::VietnamPA => false,
            Federation::Vityaz => false,
            Federation::WABDL => true,
            Federation::WDFPF => true,
            Federation::WelshPA => true,
            Federation::WP => true,
            Federation::WPA => false,
            Federation::WPARUS => false,
            Federation::WPAU => false,
            Federation::WBC => false,
            Federation::WPC => false,
            Federation::WPCGermany => false,
            Federation::WPCLatvia => false,
            Federation::WPCMoldova => false,
            Federation::WPCPortugal => false,
            Federation::WPCRUS => false,
            Federation::WPCUKR => false,
            Federation::WPF => false,
            Federation::WPU => false,
            Federation::WPUF => false,
            Federation::WNPF => true,
            Federation::WRPF => false,
            Federation::WRPFAUS => false,
            Federation::WRPFCAN => false,
            Federation::WUAP => false,
            Federation::WUAPAUT => false,
            Federation::XPC => false,
        }
    }

    /// Country out of which the federation operates.
    pub fn home_country(self) -> Option<Country> {
        match self {
            Federation::_365Strong => Some(Country::USA),
            Federation::AAP => Some(Country::Argentina),
            Federation::AAPLF => Some(Country::Australia),
            Federation::AAU => Some(Country::USA),
            Federation::ACHIPO => Some(Country::Chile),
            Federation::ADAU => Some(Country::USA),
            Federation::ADFPA => Some(Country::USA),
            Federation::ADFPF => Some(Country::USA),
            Federation::AEP => Some(Country::Spain),
            Federation::AFPF => Some(Country::USA),
            Federation::AfricanPF => None,
            Federation::AIWBPA => Some(Country::Indonesia),
            Federation::AmericanSA => Some(Country::USA),
            Federation::APA => Some(Country::USA),
            Federation::APC => Some(Country::USA),
            Federation::APF => Some(Country::USA),
            Federation::APU => Some(Country::Australia),
            Federation::AsianPF => None,
            Federation::Atlantis => Some(Country::USA),
            Federation::AusDFPF => Some(Country::Australia),
            Federation::AusPL => Some(Country::Australia),
            Federation::BAWLA => Some(Country::UK),
            Federation::BB => Some(Country::Russia),
            Federation::BBDD => Some(Country::USA),
            Federation::BDFPA => Some(Country::UK),
            Federation::BPC => Some(Country::UK),
            Federation::BPF => Some(Country::UK),
            Federation::BPO => Some(Country::UK),
            Federation::BPU => Some(Country::UK),
            Federation::BP => Some(Country::UK),
            Federation::BVDK => Some(Country::Germany),
            Federation::CAPO => Some(Country::Australia),
            Federation::CAPONZ => Some(Country::NewZealand),
            Federation::CAST => Some(Country::Czechia),
            Federation::ChinaPA => Some(Country::China),
            Federation::CommonwealthPF => None,
            Federation::CPC => Some(Country::Canada),
            Federation::CPF => Some(Country::Canada),
            Federation::CPL => Some(Country::Canada),
            Federation::CPO => Some(Country::Canada),
            Federation::CPU => Some(Country::Canada),
            Federation::CSST => Some(Country::Czechia),
            Federation::DSF => Some(Country::Denmark),
            Federation::EPA => Some(Country::England),
            Federation::EPF => None,
            Federation::FALPO => Some(Country::Argentina),
            Federation::FCST => Some(Country::Czechia),
            Federation::FEMEPO => Some(Country::Mexico),
            Federation::FEPOA => Some(Country::Argentina),
            Federation::FESUPO => None,
            Federation::FFForce => Some(Country::France),
            Federation::FPO => Some(Country::Finland),
            Federation::FPR => Some(Country::Russia),
            Federation::GoldenDouble => Some(Country::Russia),
            Federation::GPA => None,
            Federation::GPACRO => Some(Country::Croatia),
            Federation::GPC => None,
            Federation::GPCAUS => Some(Country::Australia),
            Federation::GPCGB => Some(Country::UK),
            Federation::GPCIRL => Some(Country::Ireland),
            Federation::GPCLAT => Some(Country::Latvia),
            Federation::GPCNZ => Some(Country::NewZealand),
            Federation::GPCRUS => Some(Country::Russia),
            Federation::GPF => None,
            Federation::GPU => Some(Country::Germany),
            Federation::Hardcore => Some(Country::USA),
            Federation::HERC => Some(Country::USA),
            Federation::CroatiaUA => Some(Country::Croatia),
            Federation::HPLS => Some(Country::Croatia),
            Federation::HPLSUA => Some(Country::Croatia),
            Federation::HPO => Some(Country::Croatia),
            Federation::IBSA => None,
            Federation::IDFPA => Some(Country::Ireland),
            Federation::IDFPF => Some(Country::Ireland),
            Federation::IndependentPA => Some(Country::Canada),
            Federation::IPA => Some(Country::USA),
            Federation::IPC => Some(Country::Israel),
            Federation::IPF => None,
            Federation::IPL => None,
            Federation::IPLNZ => Some(Country::NewZealand),
            Federation::IrishPF => Some(Country::Ireland),
            Federation::IrishPO => Some(Country::Ireland),
            Federation::IRP => None,
            Federation::JPA => Some(Country::Japan),
            Federation::KRAFT => Some(Country::Iceland),
            Federation::KPF => Some(Country::Kazakhstan),
            Federation::LPF => Some(Country::Latvia),
            Federation::MHP => Some(Country::USA),
            Federation::MM => Some(Country::USA),
            Federation::MPA => Some(Country::Malaysia),
            Federation::NAP => Some(Country::Russia),
            Federation::NAPF => None,
            Federation::NASA => Some(Country::USA),
            Federation::NORCAL => Some(Country::USA),
            Federation::NIPF => Some(Country::NorthernIreland),
            Federation::NordicPF => None,
            Federation::NOTLD => Some(Country::USA),
            Federation::NPA => Some(Country::Israel),
            Federation::NSF => Some(Country::Norway),
            Federation::NZOpen => Some(Country::NewZealand),
            Federation::NZPF => Some(Country::NewZealand),
            Federation::OceaniaPF => None,
            Federation::OlomouckySilak => Some(Country::Czechia),
            Federation::ParaPL => None,
            Federation::PA => Some(Country::Australia),
            Federation::PAP => Some(Country::Philippines),
            Federation::PLZS => Some(Country::Slovenia),
            Federation::PRIDE => Some(Country::USA),
            Federation::ProRaw => Some(Country::Australia),
            Federation::PRPA => Some(Country::USA),
            Federation::PZKFiTS => Some(Country::Poland),
            Federation::RAW => Some(Country::USA),
            Federation::RAWCAN => Some(Country::Canada),
            Federation::RAWUKR => Some(Country::Ukraine),
            Federation::RAWU => Some(Country::USA),
            Federation::RPS => Some(Country::USA),
            Federation::RPU => Some(Country::Russia),
            Federation::RUPC => Some(Country::USA),
            Federation::SAST => Some(Country::Slovakia),
            Federation::ScottishPL => Some(Country::Scotland),
            Federation::SCI => Some(Country::USA),
            Federation::SCT => Some(Country::Russia),
            Federation::SPA => Some(Country::Singapore),
            Federation::SPF => Some(Country::USA),
            Federation::SPSS => Some(Country::Russia),
            Federation::SSA => Some(Country::USA),
            Federation::SSF => Some(Country::Sweden),
            Federation::SVNL => Some(Country::Finland),
            Federation::ThaiPF => Some(Country::Thailand),
            Federation::THSPA => Some(Country::USA),
            Federation::UkrainePA => Some(Country::Ukraine),
            Federation::UkrainePO => Some(Country::Ukraine),
            Federation::UPA => Some(Country::USA),
            Federation::UPC => Some(Country::Ukraine),
            Federation::UkrainePF => Some(Country::Ukraine),
            Federation::USAPL => Some(Country::USA),
            Federation::USPS => Some(Country::Serbia),
            Federation::USPF => Some(Country::USA),
            Federation::USPA => Some(Country::USA),
            Federation::USSF => Some(Country::USA),
            Federation::VietnamPA => Some(Country::Vietnam),
            Federation::Vityaz => Some(Country::Russia),
            Federation::WABDL => Some(Country::USA),
            Federation::WBC => Some(Country::USA),
            Federation::WDFPF => None,
            Federation::WelshPA => Some(Country::Wales),
            Federation::WP => None,
            Federation::WPA => None,
            Federation::WPARUS => Some(Country::Russia),
            Federation::WPAU => Some(Country::Ukraine),
            Federation::WPC => None,
            Federation::WPCGermany => Some(Country::Germany),
            Federation::WPCLatvia => Some(Country::Latvia),
            Federation::WPCMoldova => Some(Country::Moldova),
            Federation::WPCPortugal => Some(Country::Portugal),
            Federation::WPCRUS => Some(Country::Russia),
            Federation::WPCUKR => Some(Country::Ukraine),
            Federation::WPF => None,
            Federation::WPU => None,
            Federation::WPUF => Some(Country::Ukraine),
            Federation::WNPF => Some(Country::USA),
            Federation::WRPF => Some(Country::Russia),
            Federation::WRPFAUS => Some(Country::Australia),
            Federation::WRPFCAN => Some(Country::Canada),
            Federation::WUAP => None,
            Federation::WUAPAUT => Some(Country::Austria),
            Federation::XPC => Some(Country::USA),
        }
    }

    /// Which points system is default for a federation's meet.
    pub fn default_points(self, _meetdate: Date) -> PointsSystem {
        match self {
            Federation::_365Strong => PointsSystem::Wilks,
            Federation::AAP => PointsSystem::Wilks,
            Federation::AAPLF => PointsSystem::Wilks,
            Federation::AAU => PointsSystem::Wilks,
            Federation::ACHIPO => PointsSystem::Wilks,
            Federation::ADAU => PointsSystem::Wilks,
            Federation::ADFPA => PointsSystem::Wilks,
            Federation::ADFPF => PointsSystem::Wilks,
            Federation::AEP => PointsSystem::Wilks,
            Federation::AFPF => PointsSystem::Wilks,
            Federation::AfricanPF => PointsSystem::Wilks,
            Federation::AIWBPA => PointsSystem::Wilks,
            Federation::AmericanSA => PointsSystem::Wilks,
            Federation::APA => PointsSystem::Wilks,
            Federation::APC => PointsSystem::Wilks,
            Federation::APF => PointsSystem::Glossbrenner,
            Federation::APU => PointsSystem::Wilks,
            Federation::AsianPF => PointsSystem::Wilks,
            Federation::Atlantis => PointsSystem::Wilks,
            Federation::AusDFPF => PointsSystem::Wilks,
            Federation::AusPL => PointsSystem::Wilks,
            Federation::BAWLA => PointsSystem::Wilks,
            Federation::BB => PointsSystem::Wilks,
            Federation::BBDD => PointsSystem::Wilks,
            Federation::BDFPA => PointsSystem::Wilks,
            Federation::BPC => PointsSystem::Wilks,
            Federation::BPF => PointsSystem::Wilks,
            Federation::BPO => PointsSystem::Wilks,
            Federation::BPU => PointsSystem::Wilks,
            Federation::BP => PointsSystem::Wilks,
            Federation::BVDK => PointsSystem::Wilks,
            Federation::CAPO => PointsSystem::Glossbrenner,
            Federation::CAPONZ => PointsSystem::Glossbrenner,
            Federation::CAST => PointsSystem::Wilks,
            Federation::ChinaPA => PointsSystem::Wilks,
            Federation::CommonwealthPF => PointsSystem::Wilks,
            Federation::CPC => PointsSystem::Wilks,
            Federation::CPF => PointsSystem::Wilks,
            Federation::CPL => PointsSystem::Wilks,
            Federation::CPO => PointsSystem::Wilks,
            Federation::CPU => PointsSystem::Wilks,
            Federation::CSST => PointsSystem::Wilks,
            Federation::DSF => PointsSystem::Wilks,
            Federation::EPA => PointsSystem::Wilks,
            Federation::EPF => PointsSystem::Wilks,
            Federation::FALPO => PointsSystem::Wilks,
            Federation::FCST => PointsSystem::Wilks,
            Federation::FEMEPO => PointsSystem::Wilks,
            Federation::FEPOA => PointsSystem::Wilks,
            Federation::FESUPO => PointsSystem::Wilks,
            Federation::FFForce => PointsSystem::Wilks,
            Federation::FPO => PointsSystem::Wilks,
            Federation::FPR => PointsSystem::Wilks,
            Federation::GoldenDouble => PointsSystem::Wilks,
            Federation::GPA => PointsSystem::Wilks,
            Federation::GPACRO => PointsSystem::Wilks,
            Federation::GPC => PointsSystem::Glossbrenner,
            Federation::GPCAUS => PointsSystem::Glossbrenner,
            Federation::GPCGB => PointsSystem::Glossbrenner,
            Federation::GPCIRL => PointsSystem::Glossbrenner,
            Federation::GPCLAT => PointsSystem::Glossbrenner,
            Federation::GPCNZ => PointsSystem::Glossbrenner,
            Federation::GPCRUS => PointsSystem::Glossbrenner,
            Federation::GPF => PointsSystem::Wilks,
            Federation::GPU => PointsSystem::Wilks,
            Federation::Hardcore => PointsSystem::Wilks,
            Federation::HERC => PointsSystem::Wilks,
            Federation::CroatiaUA => PointsSystem::Wilks,
            Federation::HPLS => PointsSystem::Wilks,
            Federation::HPLSUA => PointsSystem::Wilks,
            Federation::HPO => PointsSystem::Wilks,
            Federation::IBSA => PointsSystem::Wilks,
            Federation::IDFPA => PointsSystem::Wilks,
            Federation::IDFPF => PointsSystem::Wilks,
            Federation::IndependentPA => PointsSystem::Wilks,
            Federation::IPA => PointsSystem::Wilks,
            Federation::IPC => PointsSystem::Wilks,
            Federation::IPF => PointsSystem::Wilks,
            Federation::IPL => PointsSystem::Wilks,
            Federation::IPLNZ => PointsSystem::Wilks,
            Federation::IrishPF => PointsSystem::Wilks,
            Federation::IrishPO => PointsSystem::Wilks,
            Federation::IRP => PointsSystem::Wilks,
            Federation::JPA => PointsSystem::Wilks,
            Federation::KRAFT => PointsSystem::Wilks,
            Federation::KPF => PointsSystem::Wilks,
            Federation::LPF => PointsSystem::Wilks,
            Federation::MHP => PointsSystem::Wilks,
            Federation::MM => PointsSystem::Wilks,
            Federation::MPA => PointsSystem::Wilks,
            Federation::NAP => PointsSystem::Wilks,
            Federation::NAPF => PointsSystem::Wilks,
            Federation::NASA => PointsSystem::Wilks,
            Federation::NORCAL => PointsSystem::Wilks,
            Federation::NIPF => PointsSystem::Wilks,
            Federation::NordicPF => PointsSystem::Wilks,
            Federation::NOTLD => PointsSystem::Wilks,
            Federation::NPA => PointsSystem::Wilks,
            Federation::NSF => PointsSystem::Wilks,
            Federation::NZOpen => PointsSystem::Wilks,
            Federation::NZPF => PointsSystem::Wilks,
            Federation::OceaniaPF => PointsSystem::Wilks,
            Federation::OlomouckySilak => PointsSystem::Wilks,
            Federation::ParaPL => PointsSystem::Wilks,
            Federation::PA => PointsSystem::Wilks,
            Federation::PAP => PointsSystem::Wilks,
            Federation::PLZS => PointsSystem::Wilks,
            Federation::PRIDE => PointsSystem::Wilks,
            Federation::ProRaw => PointsSystem::Glossbrenner,
            Federation::PRPA => PointsSystem::Wilks,
            Federation::PZKFiTS => PointsSystem::Wilks,
            Federation::RAW => PointsSystem::Wilks,
            Federation::RAWCAN => PointsSystem::Wilks,
            Federation::RAWUKR => PointsSystem::Wilks,
            Federation::RAWU => PointsSystem::Wilks,
            Federation::RPS => PointsSystem::Wilks,
            Federation::RPU => PointsSystem::Wilks,
            Federation::RUPC => PointsSystem::Wilks,
            Federation::SAST => PointsSystem::Glossbrenner,
            Federation::ScottishPL => PointsSystem::Wilks,
            Federation::SCI => PointsSystem::Wilks,
            Federation::SCT => PointsSystem::Wilks,
            Federation::SPA => PointsSystem::Wilks,
            Federation::SPF => PointsSystem::Wilks,
            Federation::SPSS => PointsSystem::Wilks,
            Federation::SSA => PointsSystem::Wilks,
            Federation::SSF => PointsSystem::Wilks,
            Federation::SVNL => PointsSystem::Wilks,
            Federation::ThaiPF => PointsSystem::Wilks,
            Federation::THSPA => PointsSystem::Wilks,
            Federation::UkrainePA => PointsSystem::Wilks,
            Federation::UkrainePO => PointsSystem::Wilks,
            Federation::UPA => PointsSystem::Wilks,
            Federation::UPC => PointsSystem::Wilks,
            Federation::UkrainePF => PointsSystem::Wilks,
            Federation::USAPL => PointsSystem::Wilks,
            Federation::USPS => PointsSystem::Wilks,
            Federation::USPF => PointsSystem::Wilks,
            Federation::USPA => PointsSystem::Wilks,
            Federation::USSF => PointsSystem::Wilks,
            Federation::VietnamPA => PointsSystem::Wilks,
            Federation::Vityaz => PointsSystem::Wilks,
            Federation::WABDL => PointsSystem::Wilks,
            Federation::WBC => PointsSystem::Wilks,
            Federation::WDFPF => PointsSystem::Wilks,
            Federation::WelshPA => PointsSystem::Wilks,
            Federation::WP => PointsSystem::Wilks,
            Federation::WPA => PointsSystem::Wilks,
            Federation::WPARUS => PointsSystem::Wilks,
            Federation::WPAU => PointsSystem::Wilks,
            Federation::WPC => PointsSystem::Glossbrenner,
            Federation::WPCGermany => PointsSystem::Glossbrenner,
            Federation::WPCLatvia => PointsSystem::Glossbrenner,
            Federation::WPCMoldova => PointsSystem::Glossbrenner,
            Federation::WPCPortugal => PointsSystem::Glossbrenner,
            Federation::WPCRUS => PointsSystem::Glossbrenner,
            Federation::WPCUKR => PointsSystem::Glossbrenner,
            Federation::WPF => PointsSystem::Wilks,
            Federation::WPU => PointsSystem::Wilks,
            Federation::WPUF => PointsSystem::Wilks,
            Federation::WNPF => PointsSystem::Wilks,
            Federation::WRPF => PointsSystem::Wilks,
            Federation::WRPFAUS => PointsSystem::Wilks,
            Federation::WRPFCAN => PointsSystem::Wilks,
            Federation::WUAP => PointsSystem::Wilks,
            Federation::WUAPAUT => PointsSystem::Wilks,
            Federation::XPC => PointsSystem::Wilks,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_strings() {
        // The lowercase form should parse.
        assert_eq!("wrpf".parse::<Federation>().unwrap(), Federation::WRPF);

        // The default to_string() should be the upper-case form.
        assert_eq!(Federation::WRPF.to_string(), "WRPF");
    }
}
