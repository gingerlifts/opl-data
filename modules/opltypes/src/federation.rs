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

    /// African Continental Powerlifting Alliance, WPA.
    #[strum(to_string = "ACPA", serialize = "acpa")]
    ACPA,

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

    /// All Natural Physique and Power Conference (Defunct).
    #[strum(to_string = "ANPPC", serialize = "anppc")]
    ANPPC,

    /// American Powerlifting Association, WPA.
    #[strum(to_string = "APA", serialize = "apa")]
    APA,

    /// American Powerlifting Committee, WUAP.
    #[strum(to_string = "APC", serialize = "apc")]
    APC,

    /// American Powerlifting Federation.
    #[strum(to_string = "APF", serialize = "apf")]
    APF,

    /// Australian Powerlifting Union, IPF.
    #[strum(to_string = "APU", serialize = "apu")]
    APU,

    /// Asian Powerlifting Federation, IPF.
    #[strum(to_string = "AsianPF", serialize = "asianpf")]
    AsianPF,

    /// Australian Drug-Free Powerlifting Federation, WDFPF.
    #[strum(to_string = "AusDFPF", serialize = "ausdfpf")]
    AusDFPF,

    /// Australian Powerlifting Federation, IPF.
    /// PA precursor
    #[strum(to_string = "AusPF", serialize = "auspf")]
    AusPF,

    /// Australian Powerlifting League, IPL.
    #[strum(to_string = "AusPL", serialize = "auspl")]
    AusPL,

    /// Australian Weightlifting Federation, meets pre AAPLF.
    #[strum(to_string = "AWF", serialize = "awf")]
    AWF,

    /// British Amateur Weightlifting Association, predecessor to BP.
    #[strum(to_string = "BAWLA", serialize = "bawla")]
    BAWLA,

    /// Bogatyr Brotherhood, a stand-alone and short-lived Russian federation.
    #[strum(to_string = "BB", serialize = "bb")]
    BB,

    /// British Drug-Free Powerlifting Assocation, WDFPF.
    #[strum(to_string = "BDFPA", serialize = "bdfpa")]
    BDFPA,

    /// Belarus Powerlifting Federation, IPF.
    #[strum(to_string = "BelPF", serialize = "belpf")]
    BelPF,

    /// British Powerlifting, IPF. Formerly named GBPF.
    #[strum(to_string = "BP", serialize = "bp")]
    BP,

    /// Defunct British WPC affiliate.
    #[strum(to_string = "BPC", serialize = "bpc")]
    BPC,

    /// British Powerlifting Federation, WPU/WRPF.
    #[strum(to_string = "BPF", serialize = "bpf")]
    BPF,

    /// British Powerlifting Organization, WPF.
    #[strum(to_string = "BPO", serialize = "bpo")]
    BPO,

    /// British Powerlifting Union.
    #[strum(to_string = "BPU", serialize = "bpu")]
    BPU,

    /// Bundesverbandes Deutscher Gewichtheber, pre-BVDK.
    #[strum(to_string = "BVDG", serialize = "bvdg")]
    BVDG,

    /// Bundesverband Deutscher Kraftdreikämpf, IPF.
    #[strum(to_string = "BVDK", serialize = "bvdk")]
    BVDK,

    /// Unaffiliated meets held in Canada.
    #[serde(rename = "Canada-UA")]
    #[strum(to_string = "Canada-UA", serialize = "canada-ua")]
    CanadaUA,

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

    /// Confederação Brasileira de Levantamentos Básicos, IPF.
    #[strum(to_string = "CBLB", serialize = "cblb")]
    CBLB,

    /// Chinese Powerlifting Association, GPA.
    #[strum(to_string = "ChinaPA", serialize = "chinapa")]
    ChinaPA,

    /// Colombian Powerlifting Federation, IPF.
    #[strum(to_string = "ColPF", serialize = "colpf")]
    ColPF,

    /// Commonwealth Powerlifting Federation, IPF.
    #[strum(to_string = "CommonwealthPF", serialize = "commonwealthpf")]
    CommonwealthPF,

    /// Confederacao Brasileira De Powerlifting, GPC.
    #[strum(to_string = "CONBRAP", serialize = "conbrap")]
    CONBRAP,

    /// Canadian Powerlifting Association, WPA.
    #[strum(to_string = "CPA", serialize = "cpa")]
    CPA,

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

    /// Crimean Republic Powerlifting and Extreme Bench Press Association.
    #[strum(to_string = "CRPEBA", serialize = "crpeba")]
    CRPEBA,

    /// Unaffiliated meets held in Croatia.
    #[serde(rename = "Croatia-UA")]
    #[strum(to_string = "Croatia-UA", serialize = "croatia-ua")]
    CroatiaUA,

    /// Český svaz silového trojboje, Czech IPF affiliate.
    #[strum(to_string = "CSST", serialize = "csst")]
    CSST,

    /// Unaffiliated meets held in Czechia.
    #[serde(rename = "Czechia-UA")]
    #[strum(to_string = "Czechia-UA", serialize = "chzechia-ua")]
    CzechiaUA,

    /// Deutscher Bodybuilding und Kraftsport Verband, first German federation.
    #[strum(to_string = "DBKV", serialize = "dbkv")]
    DBKV,

    /// Danish IPF affiliate.
    #[strum(to_string = "DSF", serialize = "dsf")]
    DSF,

    /// Elite Powerlifting Canada, IPL-affiliated prior to 2018.
    #[strum(to_string = "EPC", serialize = "epc")]
    EPC,

    /// Unaffiliated meets held in England.
    #[serde(rename = "England-UA")]
    #[strum(to_string = "England-UA", serialize = "england-ua")]
    EnglandUA,

    /// English Powerlifting Association, IPF.
    #[strum(to_string = "EPA", serialize = "epa")]
    EPA,

    /// European Powerlifting Federation, IPF.
    #[strum(to_string = "EPF", serialize = "epf")]
    EPF,

    /// Ελληνικό Σωματείο Δυναμικού Τριάθλου, multi-fed Greek affiliate.
    #[strum(to_string = "ESDT", serialize = "esdt")]
    ESDT,

    /// Federación Argentina de Levantamiento de Potencia, IPF.
    #[strum(to_string = "FALPO", serialize = "falpo")]
    FALPO,

    /// Federation Bench Press Double-event, Russian fed.
    #[strum(to_string = "FBPD", serialize = "fbpd")]
    FBPD,

    /// Fellowship of Christian Athletes, Defunct US based federation.
    #[strum(to_string = "FCA", serialize = "fca")]
    FCA,

    /// Federace českého silového trojboje, GPC.
    #[strum(to_string = "FCST", serialize = "fcst")]
    FCST,

    /// Federation Camerounaise de Powerlifting et Disciplines Affinitaires.
    /// Cameroon IPF and WDFPF affiliate.
    #[strum(to_string = "FECAPOLIF", serialize = "fecapolif")]
    FECAPOLIF,

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

    /// Florida High School Athletics Association.
    #[strum(to_string = "FHSAA", serialize = "fhsaa")]
    FHSAA,

    /// Federazione Italiana Atletica Pesante.
    #[strum(to_string = "FIAP", serialize = "fiap")]
    FIAP,

    /// Federazione Italiana Powerlifting, IPF.
    #[strum(to_string = "FIPL", serialize = "fipl")]
    FIPL,

    /// Finland Powerlifting Organization, IPA.
    #[strum(to_string = "FPO", serialize = "fpo")]
    FPO,

    /// Powerlifting Federation of Russia, IPF.
    #[strum(to_string = "FPR", serialize = "fpr")]
    FPR,

    /// Federatia Romana de Powerlifting, Romanian IPF affiliate.
    #[strum(to_string = "FRPL", serialize = "frpl")]
    FRPL,

    /// German Drug-Free Powerlifting Federation, WDFPF.
    #[strum(to_string = "GDFPF", serialize = "gdfpf")]
    GDFPF,

    /// Unaffiliated meets held in Germany.
    #[serde(rename = "Germany-UA")]
    #[strum(to_string = "Germany-UA", serialize = "germany-ua")]
    GermanyUA,

    /// Global Powerlifting Union, Ukrainian GPC affiliate.
    #[strum(to_string = "GlobalPU", serialize = "globalpu")]
    GlobalPU,

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

    /// Canadian branch of the GPC.
    #[serde(rename = "GPC-CAN")]
    #[strum(to_string = "GPC-CAN", serialize = "gpc-can")]
    GPCCAN,

    /// British branch of the GPC.
    #[serde(rename = "GPC-GB")]
    #[strum(to_string = "GPC-GB", serialize = "gpc-gb")]
    GPCGB,

    /// Irish branch of the GPC.
    #[serde(rename = "GPC-IRL")]
    #[strum(to_string = "GPC-IRL", serialize = "gpc-irl")]
    GPCIRL,

    /// Israeli branch of the GPC.
    #[serde(rename = "GPC-ISR")]
    #[strum(to_string = "GPC-ISR", serialize = "gpc-isr")]
    GPCISR,

    /// Latvian branch of the GPC.
    #[serde(rename = "GPC-LAT")]
    #[strum(to_string = "GPC-LAT", serialize = "gpc-lat")]
    GPCLAT,

    /// New Zealand branch of the GPC.
    #[serde(rename = "GPC-NZ")]
    #[strum(to_string = "GPC-NZ", serialize = "gpc-nz")]
    GPCNZ,

    /// Ukrainian branch of the GPC.
    #[serde(rename = "GPC-UKR")]
    #[strum(to_string = "GPC-UKR", serialize = "gpc-ukr")]
    GPCUKR,

    /// USA branch of the GPC.
    #[serde(rename = "GPC-USA")]
    #[strum(to_string = "GPC-USA", serialize = "gpc-usa")]
    GPCUSA,

    /// Russian branch of the GPC.
    #[serde(rename = "GPC-RUS")]
    #[strum(to_string = "GPC-RUS", serialize = "gpc-rus")]
    GPCRUS,

    /// Croatian branch of the GPA and WUAP. Successor to HPO.
    #[serde(rename = "GPC-WUAP-CRO")]
    #[strum(to_string = "GPC-WUAP-CRO", serialize = "gpc-wuap-cro")]
    GPCWUAPCRO,

    /// Global Powerlifting Federation.
    #[strum(to_string = "GPF", serialize = "gpf")]
    GPF,

    /// German Powerlifting Union, WPU.
    #[strum(to_string = "GPU", serialize = "gpu")]
    GPU,

    /// German RAW Association, IRP.
    #[strum(to_string = "GRAWA", serialize = "grawa")]
    GRAWA,

    /// GSF-Belarus.
    #[serde(rename = "GSF-Belarus")]
    #[strum(to_string = "GSF-Belarus", serialize = "gsf-belarus")]
    GSFBelarus,

    /// Defunct stand-alone US federation.
    #[strum(to_string = "Hardcore", serialize = "hardcore")]
    Hardcore,

    /// Hercules Gym in Syracuse, NY. Run by Rheta West.
    #[strum(to_string = "HERC", serialize = "herc")]
    HERC,

    /// Hong Kong Weightlifting and Powerlifting, IPF.
    #[strum(to_string = "HKWPA", serialize = "hkwpa")]
    HKWPA,

    /// Hungarian Powerlifting Congress, WPC.
    #[strum(to_string = "HPC", serialize = "hpc")]
    HPC,

    /// Croatian IPF affiliate.
    #[strum(to_string = "HPLS", serialize = "hpls")]
    HPLS,

    /// Croatian Powerlifting Federation before getting affiliated with the IPF.
    #[serde(rename = "HPLS-UA")]
    #[strum(to_string = "HPLS-UA", serialize = "hpls-ua")]
    HPLSUA,

    /// Croatian Powerlifting Organization. Defunct: became GPC-WUAP-CRO.
    #[strum(to_string = "HPO", serialize = "hpo")]
    HPO,

    /// Hantang Powerlifting, from China.
    #[strum(to_string = "HTPL", serialize = "htpl")]
    HTPL,

    /// Magyar Erőemelő Szövetség, Hungarian IPF affiliate.
    ///
    /// They call themselves "Hunpower" for short.
    #[strum(to_string = "Hunpower", serialize = "hunpower")]
    Hunpower,

    /// International Blind Sport Assocation.
    #[strum(to_string = "IBSA", serialize = "ibsa")]
    IBSA,

    /// Irish Drug-Free Powerlifting Association.
    #[strum(to_string = "IDFPA", serialize = "idfpa")]
    IDFPA,

    /// Irish Drug-Free Powerlifting Federation.
    #[strum(to_string = "IDFPF", serialize = "idfpf")]
    IDFPF,

    /// Islenska Kraftlyfingafelagid, Icelandic GPC? affiliate.
    #[strum(to_string = "IKF", serialize = "ikf")]
    IKF,

    /// Israel Powerlifting Federation.
    #[strum(to_string = "ILPA", serialize = "ilpa")]
    ILPA,

    /// International Powerlifting Association.
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

    /// Unaffiliated meets held in Ireland.
    #[serde(rename = "Ireland-UA")]
    #[strum(to_string = "Ireland-UA", serialize = "ireland-ua")]
    IrelandUA,

    /// Irish Powerlifting Federation, IPF.
    #[strum(to_string = "IrishPF", serialize = "irishpf")]
    IrishPF,

    /// Irish Powerlifting Organization, WPU/IPL.
    #[strum(to_string = "IrishPO", serialize = "irishpo")]
    IrishPO,

    /// Iron Boy Powerlifting
    #[strum(to_string = "IronBoy", serialize = "ironboy")]
    IronBoy,

    /// International RAW Powerlifting.
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

    /// Kuwait Powerlifting League, IPL.
    #[strum(to_string = "KuwaitPL", serialize = "kuwaitpl")]
    KuwaitPL,

    /// LGBT Powerlifting.
    #[strum(to_string = "LGBT", serialize = "lgbt")]
    LGBT,

    /// Louisiana High School Powerlifting Association.
    #[strum(to_string = "LHSPLA", serialize = "lhspla")]
    LHSPLA,

    /// Lietuvos Jėgos Trikovės Federacija, Lithuanian IPF affiliate.
    #[strum(to_string = "LJTF", serialize = "ljtf")]
    LJTF,

    /// Liga Mexicana de Powerlifting, Mexican IPL affiliate.
    #[strum(to_string = "LMP", serialize = "lmp")]
    LMP,

    /// Latvian IPF affiliate.
    #[strum(to_string = "LPF", serialize = "lpf")]
    LPF,

    /// Mississippi High School Athletic Association.
    #[strum(to_string = "MHSAA", serialize = "mhsaa")]
    MHSAA,

    /// Michigan High School Powerlifting Association.
    #[strum(to_string = "MHSPLA", serialize = "mhspla")]
    MHSPLA,

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

    /// Natural Powerlifting Assocation, early 80's Drug Free Fed
    #[strum(to_string = "NaturalPA", serialize = "naturalpa")]
    NaturalPA,

    /// Nauru Powerlifting Federation, IPF.
    #[strum(to_string = "NauruPF", serialize = "naurupf")]
    NauruPF,

    /// NextGenPF, a USA-IN Push/Pull/Curl federation.
    #[strum(to_string = "NextGenPF", serialize = "nextgenpf")]
    NextGenPF,

    /// Northern Ireland Powerlifting Federation.
    #[strum(to_string = "NIPF", serialize = "nipf")]
    NIPF,

    /// Northern California Powerlifting Federation.
    #[strum(to_string = "NORCAL", serialize = "norcal")]
    NORCAL,

    /// Nordic Powerlifting Federation, IPF.
    #[strum(to_string = "NordicPF", serialize = "nordicpf")]
    NordicPF,

    /// National Powerlifting Association of Israel.
    #[strum(to_string = "NPA", serialize = "npa")]
    NPA,

    /// Nederlandse Powerlifting Bond, IPF.
    #[strum(to_string = "NPB", serialize = "npb")]
    NPB,

    /// Norwegian IPF affiliate.
    #[strum(to_string = "NSF", serialize = "nsf")]
    NSF,

    /// New Zealand Amateur Weightlifting Association, IPF. (NPZF Precursor)
    #[strum(to_string = "NZAWLA", serialize = "nzawla")]
    NZAWLA,

    /// New Zealand Powerlifting Federation, IPF.
    #[strum(to_string = "NZPF", serialize = "nzpf")]
    NZPF,

    /// Unaffiliated meets held in NZ.
    #[serde(rename = "NZ-UA")]
    #[strum(to_string = "NZ-UA", serialize = "nz-ua")]
    NZUA,

    /// Oceania Powerlifting Federation, WP.
    #[strum(to_string = "OceaniaPF", serialize = "oceaniapf")]
    OceaniaPF,

    /// Oceania Regional Powerlifting Federation, IPF.
    #[strum(to_string = "ORPF", serialize = "orpf")]
    ORPF,

    /// Österreichischer Verband für Kraftdreikampf, IPF.
    #[strum(to_string = "OEVK", serialize = "oevk")]
    OEVK,

    /// Powerlifting Australia, formerly IPF, now WP.
    #[strum(to_string = "PA", serialize = "pa")]
    PA,

    /// Powerlifting Association of the Philippines, IPF.
    #[strum(to_string = "PAP", serialize = "pap")]
    PAP,

    /// Philippine Powerlifting, GPA/APA.
    #[strum(to_string = "PHPL", serialize = "phpl")]
    PHPL,

    /// Power Lifting Savez Srbije, IPF.
    #[strum(to_string = "PLSS", serialize = "plss")]
    PLSS,

    /// Powerlifting zveza Slovenije, IPF.
    #[strum(to_string = "PLZS", serialize = "plzs")]
    PLZS,

    /// Papua New Guinea Powerlifting Federation, IPF.
    #[strum(to_string = "PNGPF", serialize = "pngpf")]
    PNGPF,

    /// Unaffiliated meets held in Poland.
    #[serde(rename = "Poland-UA")]
    #[strum(to_string = "Poland-UA", serialize = "poland-ua")]
    PolandUA,

    /// Police Athletic League, a defunct US Fed.
    #[strum(to_string = "PoliceAL", serialize = "policeal")]
    PoliceAL,

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

    /// Icelandic 100% Raw affiliate, not drug tested.
    #[serde(rename = "RAW-Iceland")]
    #[strum(to_string = "RAW-Iceland", serialize = "raw-iceland")]
    RAWIceland,

    /// Raw Iron Powerlifting League, an independent tested Texas federation.
    #[strum(to_string = "RawIronPL", serialize = "rawironpl")]
    RawIronPL,

    /// 100% RAW Federation Ukraine.
    #[serde(rename = "RAW-UKR")]
    #[strum(to_string = "RAW-UKR", serialize = "raw-ukr")]
    RAWUKR,

    /// Raw United Federation.
    #[strum(to_string = "RAWU", serialize = "rawu")]
    RAWU,

    /// Rhino Powerlifting Club, South African GPC Affiliate.
    #[strum(to_string = "RhinoPC", serialize = "rhinopc")]
    RhinoPC,

    /// Revolution Powerlifting Syndicate.
    #[strum(to_string = "RPS", serialize = "rps")]
    RPS,

    /// Russian Powerlifting Union.
    #[strum(to_string = "RPU", serialize = "rpu")]
    RPU,

    /// Raw Unity.
    #[strum(to_string = "RUPC", serialize = "rupc")]
    RUPC,

    /// Unaffiliated meets held in Russia.
    #[serde(rename = "Russia-UA")]
    #[strum(to_string = "Russia-UA", serialize = "russia-ua")]
    RussiaUA,

    /// South African Powerlifting Federation, IPF.
    #[strum(to_string = "SAPF", serialize = "sapf")]
    SAPF,

    /// Slovenská asociásia silového trojboja, Slovakian GPC Affiliate.
    #[strum(to_string = "SAST", serialize = "sast")]
    SAST,

    /// Scottish Powerlifting, IPF.
    #[strum(to_string = "ScottishPL", serialize = "scottishpl")]
    ScottishPL,

    /// State Correctional Institution, I think.
    #[strum(to_string = "SCI", serialize = "sci")]
    SCI,

    /// Super-Cup of Titans, a defunct Russian single-ply meet.
    #[strum(to_string = "SCT", serialize = "sct")]
    SCT,

    /// Swiss Drug-Free Powerlifting Federation, Swiss WDFPF Affiliate.
    #[strum(to_string = "SDFPF", serialize = "sdfpf")]
    SDFPF,

    /// Son Light Power, US based federation.
    #[strum(to_string = "SLP", serialize = "slp")]
    SLP,

    /// Singapore Powerlifting Alliance.
    #[strum(to_string = "SPA", serialize = "spa")]
    SPA,

    /// Southern Powerlifting Federation.
    #[strum(to_string = "SPF", serialize = "spf")]
    SPF,

    /// Southern Powerlifting Federation Ireland.
    #[serde(rename = "SPF-IRL")]
    #[strum(to_string = "SPF-IRL", serialize = "spf-irl")]
    SPFIRL,

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

    /// Swiss IPF affiliate. Previously affiliated to many untested federations.
    #[strum(to_string = "SwissPL", serialize = "swisspl")]
    SwissPL,

    /// Thai IPF affiliate.
    #[strum(to_string = "ThaiPF", serialize = "thaipf")]
    ThaiPF,

    /// Texas High School Powerlifting Association.
    #[strum(to_string = "THSPA", serialize = "thspa")]
    THSPA,

    /// Texas High School Women's Powerlifting Association.
    #[strum(to_string = "THSWPA", serialize = "thswpa")]
    THSWPA,

    /// Ukrainian Drug-Free Powerlifting Federation
    #[strum(to_string = "UDFPF", serialize = "udfpf")]
    UDFPF,

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

    /// United Powerlifting Congress Germany. WPC, GPC, WUAP.
    #[serde(rename = "UPC-Germany")]
    #[strum(to_string = "UPC-Germany", serialize = "upc-germany")]
    UPCGermany,

    /// Ukrainian Powerlifting Federation, IPF.
    #[strum(to_string = "UkrainePF", serialize = "ukrainepf")]
    UkrainePF,

    /// Ukrainian Powerlifting League, IPL.
    #[strum(to_string = "UPL", serialize = "upl")]
    UPL,

    /// United States Association of Blind Athletes, IBSA.
    #[strum(to_string = "USABA", serialize = "usaba")]
    USABA,

    /// USA Bench Press Association, unaffiliated.
    #[strum(to_string = "USABPA", serialize = "usabpa")]
    USABPA,

    /// Unaffiliated meets held in the USA.
    #[serde(rename = "USA-UA")]
    #[strum(to_string = "USA-UA", serialize = "usa-ua")]
    USAUA,

    /// USA Powerlifting, IPF.
    #[strum(to_string = "USAPL", serialize = "usapl")]
    USAPL,

    /// USA Raw Bench Press Federation (Defunct).
    #[strum(to_string = "USARawBP", serialize = "usarawbp")]
    USARawBP,

    /// Catch-all for overseas meets done by US Military members.
    #[strum(to_string = "USMilAbroad", serialize = "usmilabroad")]
    USMilAbroad,

    /// Ujedinjeni Srpski powerlifting savez.
    #[strum(to_string = "USPS", serialize = "usps")]
    USPS,

    /// US Powerlifting Federation.
    #[strum(to_string = "USPF", serialize = "uspf")]
    USPF,

    /// United States Powerlifting Assocation, IPL.
    #[strum(to_string = "USPA", serialize = "uspa")]
    USPA,

    /// United States Strengthlifting Federation.
    #[strum(to_string = "USSF", serialize = "ussf")]
    USSF,

    /// Unified Strength Sports Federation.
    #[strum(to_string = "USSports", serialize = "ussports")]
    USSports,

    /// US Virgin Islands Powerlifting Federation, IPF.
    #[strum(to_string = "USVIPF", serialize = "usvipf")]
    USVIPF,

    /// Vietnam Powerlifting Alliance, GPA.
    #[strum(to_string = "VietnamPA", serialize = "vietnampa")]
    VietnamPA,

    /// Defunct Russian meet.
    #[strum(to_string = "Vityaz", serialize = "vityaz")]
    Vityaz,

    /// Vietnam Powerlifting Federation, IPF.
    #[strum(to_string = "VPF", serialize = "vpf")]
    VPF,

    /// World Association of Bench Pressers and Deadlifters.
    #[strum(to_string = "WABDL", serialize = "wabdl")]
    WABDL,

    /// Warrior Powerlifting Federation, the continuation of Son Light Power.
    ///
    /// The federation renamed itself to WarriorPLF around August 2019.
    #[strum(to_string = "WarriorPLF", serialize = "warriorplf")]
    WarriorPLF,

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

    /// World Powerlifting Alliance Georgia.
    #[serde(rename = "WPA-GEO")]
    #[strum(to_string = "WPA-GEO", serialize = "wpa-geo")]
    WPAGEO,

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

    /// WPC meets hosted by METAL gym Finland.
    #[serde(rename = "WPC-Finland")]
    #[strum(to_string = "WPC-Finland", serialize = "wpc-finland")]
    WPCFinland,

    /// French WPC affiliate.
    #[serde(rename = "WPC-France")]
    #[strum(to_string = "WPC-France", serialize = "wpc-france")]
    WPCFrance,

    /// German WPC affiliate.
    #[serde(rename = "WPC-Germany")]
    #[strum(to_string = "WPC-Germany", serialize = "wpc-germany")]
    WPCGermany,

    /// Icelandic WPC affiliate.
    #[serde(rename = "WPC-Iceland")]
    #[strum(to_string = "WPC-Iceland", serialize = "wpc-iceland")]
    WPCIceland,

    /// Israeli WPC affiliate.
    #[serde(rename = "WPC-Israel")]
    #[strum(to_string = "WPC-Israel", serialize = "wpc-israel")]
    WPCIsrael,

    /// Italian WPC affiliate.
    #[serde(rename = "WPC-Italy")]
    #[strum(to_string = "WPC-Italy", serialize = "wpc-italy")]
    WPCItaly,

    /// Kazakh WPC affiliate.
    #[serde(rename = "WPC-KAZ")]
    #[strum(to_string = "WPC-KAZ", serialize = "wpc-kaz")]
    WPCKAZ,

    /// Kyrgyzstan WPC affiliate.
    #[serde(rename = "WPC-KGZ")]
    #[strum(to_string = "WPC-KGZ", serialize = "wpc-kgz")]
    WPCKGZ,

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

    /// South African WPC affiliate.
    #[serde(rename = "WPC-SA")]
    #[strum(to_string = "WPC-SA", serialize = "wpc-sa")]
    WPCSA,

    /// Ukrainian WPC affiliate.
    #[serde(rename = "WPC-UKR")]
    #[strum(to_string = "WPC-UKR", serialize = "wpc-ukr")]
    WPCUKR,

    /// World Powerlifting Federation.
    #[strum(to_string = "WPF", serialize = "wpf")]
    WPF,

    /// World Police and Fire Games.
    #[strum(to_string = "WPFG", serialize = "wpfg")]
    WPFG,

    /// World Powerlifting League.
    #[strum(to_string = "WPLeague", serialize = "wpleague")]
    WPLeague,

    /// World Powerlifting New Zealand.
    #[serde(rename = "WP-NZ")]
    #[strum(to_string = "WP-NZ", serialize = "wp-nz")]
    WPNZ,

    /// World Paralympic Powerlifting (formerly ParaPL).
    #[strum(to_string = "WPPO", serialize = "wppo")]
    WPPO,

    /// World Powerlifting Raw Organisation.
    #[strum(to_string = "WPRO", serialize = "wpro")]
    WPRO,

    /// World Power Sport Federation.
    #[strum(to_string = "WPSF", serialize = "wpsf")]
    WPSF,

    /// World Power Sport Federation, Belarus.
    #[serde(rename = "WPSF-Belarus")]
    #[strum(to_string = "WPSF-Belarus", serialize = "wpsf-belarus")]
    WPSFBelarus,

    /// World Powerlifting Union.
    #[strum(to_string = "WPU", serialize = "wpu")]
    WPU,

    /// World Powerlifting Union of Federations.
    #[strum(to_string = "WPUF", serialize = "wpuf")]
    WPUF,

    /// World Powerlifting Union Russia.
    #[serde(rename = "WPU-RUS")]
    #[strum(to_string = "WPU-RUS", serialize = "wpu-rus")]
    WPURUS,

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

    /// Belarusian WRPF affiliate.
    #[serde(rename = "WRPF-Belarus")]
    #[strum(to_string = "WRPF-Belarus", serialize = "wrpf-belarus")]
    WRPFBelarus,

    /// Canadian WRPF affiliate.
    #[serde(rename = "WRPF-CAN")]
    #[strum(to_string = "WRPF-CAN", serialize = "wrpf-can")]
    WRPFCAN,

    /// Icelandic WRPF affiliate.
    #[serde(rename = "WRPF-Iceland")]
    #[strum(to_string = "WRPF-Iceland", serialize = "wrpf-iceland")]
    WRPFIceland,

    /// Irish WRPF affiliate.
    #[serde(rename = "WRPF-Ireland")]
    #[strum(to_string = "WRPF-Ireland", serialize = "wrpf-ireland")]
    WRPFIreland,

    /// Italian WRPF affiliate.
    #[serde(rename = "WRPF-Italy")]
    #[strum(to_string = "WRPF-Italy", serialize = "wrpf-italy")]
    WRPFItaly,

    /// Kazakh WRPF affiliate.
    #[serde(rename = "WRPF-KAZ")]
    #[strum(to_string = "WRPF-KAZ", serialize = "wrpf-kaz")]
    WRPFKAZ,

    /// Lithuanian WRPF affiliate.
    #[serde(rename = "WRPF-Lithuania")]
    #[strum(to_string = "WRPF-Lithuania", serialize = "wrpf-lithuania")]
    WRPFLithuania,

    /// Polish WRPF affiliate.
    #[serde(rename = "WRPF-POL")]
    #[strum(to_string = "WRPF-POL", serialize = "wrpf-pol")]
    WRPFPOL,

    /// Portugese WRPF affiliate.
    #[serde(rename = "WRPF-Portugal")]
    #[strum(to_string = "WRPF-Portugal", serialize = "wrpf-portugal")]
    WRPFPortugal,

    /// Slovakian WRPF affiliate.
    #[serde(rename = "WRPF-Slovakia")]
    #[strum(to_string = "WRPF-Slovakia", serialize = "wrpf-slovakia")]
    WRPFSlovakia,

    /// Slovenian WRPF affiliate.
    #[serde(rename = "WRPF-Slovenia")]
    #[strum(to_string = "WRPF-Slovenia", serialize = "wrpf-slovenia")]
    WRPFSlovenia,

    /// Spanish WRPF affiliate.
    #[serde(rename = "WRPF-Spain")]
    #[strum(to_string = "WRPF-Spain", serialize = "wrpf-spain")]
    WRPFSpain,

    /// Serbian WRPF affiliate.
    #[serde(rename = "WRPF-SRB")]
    #[strum(to_string = "WRPF-SRB", serialize = "wrpf-srb")]
    WRPFSRB,

    /// Swedish WRPF affiliate.
    #[serde(rename = "WRPF-Sweden")]
    #[strum(to_string = "WRPF-Sweden", serialize = "wrpf-sweden")]
    WRPFSweden,

    /// UK WRPF affiliate.
    #[serde(rename = "WRPF-UK")]
    #[strum(to_string = "WRPF-UK", serialize = "wrpf-uk")]
    WRPFUK,

    /// World United Amateur Powerlifting.
    #[strum(to_string = "WUAP", serialize = "wuap")]
    WUAP,

    /// Austrian WUAP affiliate.
    #[serde(rename = "WUAP-AUT")]
    #[strum(to_string = "WUAP-AUT", serialize = "wuap-aut")]
    WUAPAUT,

    /// Slovakian WUAP affiliate.
    #[serde(rename = "WUAP-SVK")]
    #[strum(to_string = "WUAP-SVK", serialize = "wuap-svk")]
    WUAPSVK,

    /// Xtreme Powerlifting Coalition.
    #[strum(to_string = "XPC", serialize = "xpc")]
    XPC,

    /// Polish version of the XPC
    #[serde(rename = "XPC-Poland")]
    #[strum(to_string = "XPC-Poland", serialize = "xpc-poland")]
    XPCPoland,
}

impl Federation {
    /// True iff every division in the federation is drug-tested.
    pub fn is_fully_tested(self) -> bool {
        const FULLY_TESTED: bool = true;

        match self {
            Federation::_365Strong => false,
            Federation::AAP => false,
            Federation::AAPLF => FULLY_TESTED,
            Federation::AAU => FULLY_TESTED,
            Federation::ACHIPO => false,
            Federation::ACPA => false,
            Federation::ADAU => FULLY_TESTED,
            Federation::ADFPA => FULLY_TESTED,
            Federation::ADFPF => FULLY_TESTED,
            Federation::AEP => FULLY_TESTED,
            Federation::AFPF => false,
            Federation::AfricanPF => FULLY_TESTED,
            Federation::AIWBPA => FULLY_TESTED,
            Federation::AmericanSA => false,
            Federation::ANPPC => false,
            Federation::APA => false,
            Federation::APC => false,
            Federation::APF => false,
            Federation::APU => FULLY_TESTED,
            Federation::AsianPF => FULLY_TESTED,
            Federation::AusDFPF => FULLY_TESTED,
            Federation::AusPF => false,
            Federation::AusPL => false,
            Federation::AWF => false,
            Federation::BAWLA => FULLY_TESTED,
            Federation::BB => false,
            Federation::BDFPA => FULLY_TESTED,
            Federation::BelPF => FULLY_TESTED,
            Federation::BP => FULLY_TESTED,
            Federation::BPC => false,
            Federation::BPF => false,
            Federation::BPO => false,
            Federation::BPU => false,
            Federation::BVDG => FULLY_TESTED,
            Federation::BVDK => FULLY_TESTED,
            Federation::CanadaUA => false,
            Federation::CAPO => false,
            Federation::CAPONZ => false,
            Federation::CAST => false,
            Federation::CBLB => FULLY_TESTED,
            Federation::ChinaPA => false,
            Federation::ColPF => FULLY_TESTED,
            Federation::CommonwealthPF => FULLY_TESTED,
            Federation::CONBRAP => false,
            Federation::CPA => false,
            Federation::CPC => false,
            Federation::CPF => false,
            Federation::CPL => false,
            Federation::CPO => false,
            Federation::CPU => FULLY_TESTED,
            Federation::CRPEBA => false,
            Federation::CSST => FULLY_TESTED,
            Federation::DBKV => false,
            Federation::CzechiaUA => false,
            Federation::DSF => FULLY_TESTED,
            Federation::EPC => false,
            Federation::EnglandUA => false,
            Federation::EPA => FULLY_TESTED,
            Federation::EPF => FULLY_TESTED,
            Federation::ESDT => false,
            Federation::FALPO => FULLY_TESTED,
            Federation::FBPD => false,
            Federation::FCA => false,
            Federation::FCST => false,
            Federation::FECAPOLIF => FULLY_TESTED,
            Federation::FEMEPO => FULLY_TESTED,
            Federation::FEPOA => false,
            Federation::FESUPO => FULLY_TESTED,
            Federation::FFForce => FULLY_TESTED,
            Federation::FHSAA => FULLY_TESTED,
            Federation::FIAP => FULLY_TESTED,
            Federation::FIPL => FULLY_TESTED,
            Federation::FPO => false,
            Federation::FPR => FULLY_TESTED,
            Federation::FRPL => FULLY_TESTED,
            Federation::GDFPF => FULLY_TESTED,
            Federation::GermanyUA => false,
            Federation::GlobalPU => false,
            Federation::GPA => false,
            Federation::GPACRO => false,
            Federation::GPC => false,
            Federation::GPCAUS => false,
            Federation::GPCGB => false,
            Federation::GPCIRL => false,
            Federation::GPCISR => false,
            Federation::GPCLAT => false,
            Federation::GPCNZ => false,
            Federation::GPCUKR => false,
            Federation::GPCUSA => false,
            Federation::GPCRUS => false,
            Federation::GPCWUAPCRO => false,
            Federation::GPF => false,
            Federation::GPU => false,
            Federation::GRAWA => false,
            Federation::GSFBelarus => false,
            Federation::Hardcore => false,
            Federation::HERC => false,
            Federation::CroatiaUA => false,
            Federation::HKWPA => FULLY_TESTED,
            Federation::HPC => false,
            Federation::HPLS => FULLY_TESTED,
            Federation::HPLSUA => false,
            Federation::HPO => false,
            Federation::HTPL => FULLY_TESTED,
            Federation::Hunpower => FULLY_TESTED,
            Federation::IBSA => FULLY_TESTED,
            Federation::IDFPA => FULLY_TESTED,
            Federation::IDFPF => FULLY_TESTED,
            Federation::IKF => false,
            Federation::GPCCAN => false,
            Federation::ILPA => false,
            Federation::IPA => false,
            Federation::IPC => false,
            Federation::IPF => FULLY_TESTED,
            Federation::IPL => false,
            Federation::IPLNZ => false,
            Federation::IrelandUA => false,
            Federation::IrishPF => FULLY_TESTED,
            Federation::IrishPO => false,
            Federation::IronBoy => FULLY_TESTED,
            Federation::IRP => false,
            Federation::JPA => FULLY_TESTED,
            Federation::KPF => FULLY_TESTED,
            Federation::KRAFT => FULLY_TESTED,
            Federation::KuwaitPL => false,
            Federation::LGBT => false,
            Federation::LHSPLA => false,
            Federation::LJTF => FULLY_TESTED,
            Federation::LMP => false,
            Federation::LPF => FULLY_TESTED,
            Federation::MHSAA => false,
            Federation::MHSPLA => false,
            Federation::MM => false,
            Federation::MPA => false,
            Federation::NAP => false,
            Federation::NAPF => FULLY_TESTED,
            Federation::NASA => FULLY_TESTED,
            Federation::NaturalPA => FULLY_TESTED,
            Federation::NauruPF => FULLY_TESTED,
            Federation::NextGenPF => false,
            Federation::NIPF => FULLY_TESTED,
            Federation::NORCAL => FULLY_TESTED,
            Federation::NordicPF => FULLY_TESTED,
            Federation::NPA => false,
            Federation::NPB => FULLY_TESTED,
            Federation::NSF => FULLY_TESTED,
            Federation::NZPF => FULLY_TESTED,
            Federation::NZAWLA => FULLY_TESTED,
            Federation::NZUA => false,
            Federation::OceaniaPF => FULLY_TESTED,
            Federation::ORPF => FULLY_TESTED,
            Federation::OEVK => FULLY_TESTED,
            Federation::PA => FULLY_TESTED,
            Federation::PoliceAL => false,
            Federation::PAP => FULLY_TESTED,
            Federation::PHPL => false,
            Federation::PLSS => FULLY_TESTED,
            Federation::PLZS => FULLY_TESTED,
            Federation::PNGPF => FULLY_TESTED,
            Federation::PolandUA => false,
            Federation::PRIDE => false,
            Federation::ProRaw => false,
            Federation::PRPA => false,
            Federation::PZKFiTS => FULLY_TESTED,
            Federation::RAW => FULLY_TESTED,
            Federation::RAWCAN => FULLY_TESTED,
            Federation::RAWIceland => false,
            Federation::RawIronPL => FULLY_TESTED,
            Federation::RAWUKR => FULLY_TESTED,
            Federation::RAWU => false,
            Federation::RhinoPC => false,
            Federation::RPS => false,
            Federation::RPU => false,
            Federation::RUPC => false,
            Federation::RussiaUA => false,
            Federation::SAPF => FULLY_TESTED,
            Federation::SAST => false,
            Federation::ScottishPL => FULLY_TESTED,
            Federation::SCI => false,
            Federation::SCT => false,
            Federation::SDFPF => FULLY_TESTED,
            Federation::SLP => false,
            Federation::SPA => false,
            Federation::SPF => false,
            Federation::SPFIRL => false,
            Federation::SPSS => false,
            Federation::SSA => false,
            Federation::SSF => FULLY_TESTED,
            Federation::SVNL => FULLY_TESTED,
            Federation::SwissPL => false,
            Federation::ThaiPF => FULLY_TESTED,
            Federation::THSPA => FULLY_TESTED,
            Federation::THSWPA => FULLY_TESTED,
            Federation::UDFPF => FULLY_TESTED,
            Federation::UkrainePA => false,
            Federation::UkrainePO => false,
            Federation::UPA => false,
            Federation::UPC => false,
            Federation::UPCGermany => false,
            Federation::UkrainePF => FULLY_TESTED,
            Federation::UPL => false,
            Federation::USABA => FULLY_TESTED,
            Federation::USABPA => false,
            Federation::USAUA => false,
            Federation::USAPL => FULLY_TESTED,
            Federation::USARawBP => false,
            Federation::USMilAbroad => false,
            Federation::USPS => false,
            Federation::USPF => false,
            Federation::USPA => false,
            Federation::USSF => false,
            Federation::USSports => false,
            Federation::USVIPF => FULLY_TESTED,
            Federation::VietnamPA => false,
            Federation::Vityaz => false,
            Federation::VPF => FULLY_TESTED,
            Federation::WABDL => FULLY_TESTED,
            Federation::WarriorPLF => false,
            Federation::WDFPF => FULLY_TESTED,
            Federation::WelshPA => FULLY_TESTED,
            Federation::WNPF => FULLY_TESTED,
            Federation::WP => FULLY_TESTED,
            Federation::WPA => false,
            Federation::WPAGEO => false,
            Federation::WPARUS => false,
            Federation::WPAU => false,
            Federation::WBC => false,
            Federation::WPC => false,
            Federation::WPCFinland => false,
            Federation::WPCFrance => false,
            Federation::WPCGermany => false,
            Federation::WPCIceland => false,
            Federation::WPCIsrael => false,
            Federation::WPCItaly => false,
            Federation::WPCKAZ => false,
            Federation::WPCKGZ => false,
            Federation::WPCLatvia => false,
            Federation::WPCMoldova => false,
            Federation::WPCPortugal => false,
            Federation::WPCRUS => false,
            Federation::WPCSA => false,
            Federation::WPCUKR => false,
            Federation::WPF => false,
            Federation::WPFG => false,
            Federation::WPLeague => false,
            Federation::WPNZ => FULLY_TESTED,
            Federation::WPPO => FULLY_TESTED,
            Federation::WPRO => false,
            Federation::WPSF => false,
            Federation::WPSFBelarus => false,
            Federation::WPU => false,
            Federation::WPUF => false,
            Federation::WPURUS => false,
            Federation::WRPF => false,
            Federation::WRPFAUS => false,
            Federation::WRPFBelarus => false,
            Federation::WRPFCAN => false,
            Federation::WRPFIceland => false,
            Federation::WRPFIreland => false,
            Federation::WRPFItaly => false,
            Federation::WRPFKAZ => false,
            Federation::WRPFLithuania => false,
            Federation::WRPFPOL => false,
            Federation::WRPFPortugal => false,
            Federation::WRPFSlovakia => false,
            Federation::WRPFSlovenia => false,
            Federation::WRPFSpain => false,
            Federation::WRPFSRB => false,
            Federation::WRPFSweden => false,
            Federation::WRPFUK => false,
            Federation::WUAP => false,
            Federation::WUAPAUT => false,
            Federation::WUAPSVK => false,
            Federation::XPC => false,
            Federation::XPCPoland => false,
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
            Federation::ACPA => None,
            Federation::ADAU => Some(Country::USA),
            Federation::ADFPA => Some(Country::USA),
            Federation::ADFPF => Some(Country::USA),
            Federation::AEP => Some(Country::Spain),
            Federation::AFPF => Some(Country::USA),
            Federation::AfricanPF => None,
            Federation::AIWBPA => Some(Country::Indonesia),
            Federation::AmericanSA => Some(Country::USA),
            Federation::ANPPC => Some(Country::USA),
            Federation::APA => Some(Country::USA),
            Federation::APC => Some(Country::USA),
            Federation::APF => Some(Country::USA),
            Federation::APU => Some(Country::Australia),
            Federation::AsianPF => None,
            Federation::AusDFPF => Some(Country::Australia),
            Federation::AusPF => Some(Country::Australia),
            Federation::AusPL => Some(Country::Australia),
            Federation::AWF => Some(Country::Australia),
            Federation::BAWLA => Some(Country::UK),
            Federation::BB => Some(Country::Russia),
            Federation::BDFPA => Some(Country::UK),
            Federation::BelPF => Some(Country::Belarus),
            Federation::BP => Some(Country::UK),
            Federation::BPC => Some(Country::UK),
            Federation::BPF => Some(Country::UK),
            Federation::BPO => Some(Country::UK),
            Federation::BPU => Some(Country::UK),
            Federation::BVDG => Some(Country::Germany),
            Federation::BVDK => Some(Country::Germany),
            Federation::CanadaUA => Some(Country::Canada),
            Federation::CAPO => Some(Country::Australia),
            Federation::CAPONZ => Some(Country::NewZealand),
            Federation::CAST => Some(Country::Czechia),
            Federation::CBLB => Some(Country::Brazil),
            Federation::ChinaPA => Some(Country::China),
            Federation::ColPF => Some(Country::Colombia),
            Federation::CommonwealthPF => None,
            Federation::CONBRAP => Some(Country::Brazil),
            Federation::CPA => Some(Country::Canada),
            Federation::CPC => Some(Country::Canada),
            Federation::CPF => Some(Country::Canada),
            Federation::CPL => Some(Country::Canada),
            Federation::CPO => Some(Country::Canada),
            Federation::CPU => Some(Country::Canada),
            //Initially Ukraine,until annexation
            Federation::CRPEBA => Some(Country::Russia),
            Federation::CSST => Some(Country::Czechia),
            Federation::DBKV => Some(Country::Germany),
            Federation::CzechiaUA => Some(Country::Czechia),
            Federation::DSF => Some(Country::Denmark),
            Federation::EPC => Some(Country::Canada),
            Federation::EnglandUA => Some(Country::England),
            Federation::EPA => Some(Country::England),
            Federation::EPF => None,
            Federation::ESDT => Some(Country::Greece),
            Federation::FALPO => Some(Country::Argentina),
            Federation::FBPD => Some(Country::Russia),
            Federation::FCA => Some(Country::USA),
            Federation::FCST => Some(Country::Czechia),
            Federation::FECAPOLIF => Some(Country::Cameroon),
            Federation::FEMEPO => Some(Country::Mexico),
            Federation::FEPOA => Some(Country::Argentina),
            Federation::FESUPO => None,
            Federation::FFForce => Some(Country::France),
            Federation::FHSAA => Some(Country::USA),
            Federation::FIAP => Some(Country::Italy),
            Federation::FIPL => Some(Country::Italy),
            Federation::FPO => Some(Country::Finland),
            Federation::FPR => Some(Country::Russia),
            Federation::FRPL => Some(Country::Romania),
            Federation::GDFPF => Some(Country::Germany),
            Federation::GermanyUA => Some(Country::Germany),
            Federation::GlobalPU => Some(Country::Ukraine),
            Federation::GPA => None,
            Federation::GPACRO => Some(Country::Croatia),
            Federation::GPC => None,
            Federation::GPCAUS => Some(Country::Australia),
            Federation::GPCCAN => Some(Country::Canada),
            Federation::GPCGB => Some(Country::UK),
            Federation::GPCIRL => Some(Country::Ireland),
            Federation::GPCISR => Some(Country::Israel),
            Federation::GPCLAT => Some(Country::Latvia),
            Federation::GPCNZ => Some(Country::NewZealand),
            Federation::GPCUKR => Some(Country::Ukraine),
            Federation::GPCUSA => Some(Country::USA),
            Federation::GPCRUS => Some(Country::Russia),
            Federation::GPCWUAPCRO => Some(Country::Croatia),
            Federation::GPF => None,
            Federation::GPU => Some(Country::Germany),
            Federation::GRAWA => Some(Country::Germany),
            Federation::GSFBelarus => Some(Country::Belarus),
            Federation::Hardcore => Some(Country::USA),
            Federation::HERC => Some(Country::USA),
            Federation::CroatiaUA => Some(Country::Croatia),
            Federation::HKWPA => Some(Country::HongKong),
            Federation::HPC => Some(Country::Hungary),
            Federation::HPLS => Some(Country::Croatia),
            Federation::HPLSUA => Some(Country::Croatia),
            Federation::HPO => Some(Country::Croatia),
            Federation::HTPL => Some(Country::China),
            Federation::Hunpower => Some(Country::Hungary),
            Federation::IBSA => None,
            Federation::IDFPA => Some(Country::Ireland),
            Federation::IDFPF => Some(Country::Ireland),
            Federation::IKF => Some(Country::Iceland),
            Federation::ILPA => Some(Country::Israel),
            Federation::IPA => Some(Country::USA),
            Federation::IPC => Some(Country::Israel),
            Federation::IPF => None,
            Federation::IPL => None,
            Federation::IPLNZ => Some(Country::NewZealand),
            Federation::IrelandUA => Some(Country::Ireland),
            Federation::IrishPF => Some(Country::Ireland),
            Federation::IrishPO => Some(Country::Ireland),
            Federation::IronBoy => Some(Country::USA),
            Federation::IRP => None,
            Federation::JPA => Some(Country::Japan),
            Federation::KPF => Some(Country::Kazakhstan),
            Federation::KRAFT => Some(Country::Iceland),
            Federation::KuwaitPL => Some(Country::Kuwait),
            Federation::LGBT => None,
            Federation::LHSPLA => Some(Country::USA),
            Federation::LJTF => Some(Country::Lithuania),
            Federation::LMP => Some(Country::Mexico),
            Federation::LPF => Some(Country::Latvia),
            Federation::MHSAA => Some(Country::USA),
            Federation::MHSPLA => Some(Country::USA),
            Federation::MM => Some(Country::USA),
            Federation::MPA => Some(Country::Malaysia),
            Federation::NAP => Some(Country::Russia),
            Federation::NAPF => None,
            Federation::NASA => Some(Country::USA),
            Federation::NaturalPA => Some(Country::USA),
            Federation::NauruPF => Some(Country::Nauru),
            Federation::NextGenPF => Some(Country::USA),
            Federation::NORCAL => Some(Country::USA),
            Federation::NIPF => Some(Country::NorthernIreland),
            Federation::NordicPF => None,
            Federation::NPA => Some(Country::Israel),
            Federation::NPB => Some(Country::Netherlands),
            Federation::NSF => Some(Country::Norway),
            Federation::NZPF => Some(Country::NewZealand),
            Federation::NZAWLA => Some(Country::NewZealand),
            Federation::NZUA => Some(Country::NewZealand),
            Federation::OceaniaPF => None,
            Federation::ORPF => None,
            Federation::OEVK => Some(Country::Austria),
            Federation::PA => Some(Country::Australia),
            Federation::PoliceAL => Some(Country::USA),
            Federation::PAP => Some(Country::Philippines),
            Federation::PHPL => Some(Country::Philippines),
            Federation::PLSS => Some(Country::Serbia),
            Federation::PLZS => Some(Country::Slovenia),
            Federation::PNGPF => Some(Country::PapuaNewGuinea),
            Federation::PolandUA => Some(Country::Poland),
            Federation::PRIDE => Some(Country::USA),
            Federation::ProRaw => Some(Country::Australia),
            Federation::PRPA => Some(Country::USA),
            Federation::PZKFiTS => Some(Country::Poland),
            Federation::RAW => Some(Country::USA),
            Federation::RAWCAN => Some(Country::Canada),
            Federation::RAWIceland => Some(Country::Iceland),
            Federation::RawIronPL => Some(Country::USA),
            Federation::RAWUKR => Some(Country::Ukraine),
            Federation::RAWU => Some(Country::USA),
            Federation::RhinoPC => Some(Country::SouthAfrica),
            Federation::RPS => Some(Country::USA),
            Federation::RPU => Some(Country::Russia),
            Federation::RUPC => Some(Country::USA),
            Federation::RussiaUA => Some(Country::Russia),
            Federation::SAPF => Some(Country::SouthAfrica),
            Federation::SAST => Some(Country::Slovakia),
            Federation::ScottishPL => Some(Country::Scotland),
            Federation::SCI => Some(Country::USA),
            Federation::SCT => Some(Country::Russia),
            Federation::SDFPF => Some(Country::Switzerland),
            Federation::SLP => Some(Country::USA),
            Federation::SPA => Some(Country::Singapore),
            Federation::SPF => Some(Country::USA),
            Federation::SPFIRL => Some(Country::Ireland),
            Federation::SPSS => Some(Country::Russia),
            Federation::SSA => Some(Country::USA),
            Federation::SSF => Some(Country::Sweden),
            Federation::SVNL => Some(Country::Finland),
            Federation::SwissPL => Some(Country::Switzerland),
            Federation::ThaiPF => Some(Country::Thailand),
            Federation::THSPA => Some(Country::USA),
            Federation::THSWPA => Some(Country::USA),
            Federation::UDFPF => Some(Country::Ukraine),
            Federation::UkrainePA => Some(Country::Ukraine),
            Federation::UkrainePO => Some(Country::Ukraine),
            Federation::UPA => Some(Country::USA),
            Federation::UPC => Some(Country::Ukraine),
            Federation::UPCGermany => Some(Country::Germany),
            Federation::UkrainePF => Some(Country::Ukraine),
            Federation::UPL => Some(Country::Ukraine),
            Federation::USABA => Some(Country::USA),
            Federation::USABPA => Some(Country::USA),
            Federation::USAUA => Some(Country::USA),
            Federation::USAPL => Some(Country::USA),
            Federation::USARawBP => Some(Country::USA),
            Federation::USMilAbroad => Some(Country::USA),
            Federation::USPS => Some(Country::Serbia),
            Federation::USPF => Some(Country::USA),
            Federation::USPA => Some(Country::USA),
            Federation::USSF => Some(Country::USA),
            Federation::USSports => Some(Country::USA),
            Federation::USVIPF => Some(Country::USVirginIslands),
            Federation::VietnamPA => Some(Country::Vietnam),
            Federation::Vityaz => Some(Country::Russia),
            Federation::VPF => Some(Country::Vietnam),
            Federation::WABDL => Some(Country::USA),
            Federation::WarriorPLF => Some(Country::USA),
            Federation::WBC => Some(Country::USA),
            Federation::WDFPF => None,
            Federation::WelshPA => Some(Country::Wales),
            Federation::WP => None,
            Federation::WPA => None,
            Federation::WPAGEO => Some(Country::Georgia),
            Federation::WPARUS => Some(Country::Russia),
            Federation::WPAU => Some(Country::Ukraine),
            Federation::WPC => None,
            Federation::WPCFinland => Some(Country::Finland),
            Federation::WPCFrance => Some(Country::France),
            Federation::WPCGermany => Some(Country::Germany),
            Federation::WPCIceland => Some(Country::Iceland),
            Federation::WPCIsrael => Some(Country::Israel),
            Federation::WPCItaly => Some(Country::Italy),
            Federation::WPCKAZ => Some(Country::Kazakhstan),
            Federation::WPCKGZ => Some(Country::Kyrgyzstan),
            Federation::WPCLatvia => Some(Country::Latvia),
            Federation::WPCMoldova => Some(Country::Moldova),
            Federation::WPCPortugal => Some(Country::Portugal),
            Federation::WPCRUS => Some(Country::Russia),
            Federation::WPCSA => Some(Country::SouthAfrica),
            Federation::WPCUKR => Some(Country::Ukraine),
            Federation::WPF => None,
            Federation::WPFG => None,
            Federation::WPLeague => Some(Country::Ukraine),
            Federation::WPNZ => Some(Country::NewZealand),
            Federation::WPPO => None,
            Federation::WPRO => Some(Country::Ukraine),
            Federation::WPSF => None,
            Federation::WPSFBelarus => Some(Country::Belarus),
            Federation::WPU => None,
            Federation::WPUF => Some(Country::Ukraine),
            Federation::WPURUS => Some(Country::Russia),
            Federation::WNPF => Some(Country::USA),
            Federation::WRPF => Some(Country::Russia),
            Federation::WRPFAUS => Some(Country::Australia),
            Federation::WRPFBelarus => Some(Country::Belarus),
            Federation::WRPFCAN => Some(Country::Canada),
            Federation::WRPFIceland => Some(Country::Iceland),
            Federation::WRPFIreland => Some(Country::Ireland),
            Federation::WRPFItaly => Some(Country::Italy),
            Federation::WRPFKAZ => Some(Country::Kazakhstan),
            Federation::WRPFLithuania => Some(Country::Lithuania),
            Federation::WRPFPOL => Some(Country::Poland),
            Federation::WRPFPortugal => Some(Country::Portugal),
            Federation::WRPFSlovakia => Some(Country::Slovakia),
            Federation::WRPFSlovenia => Some(Country::Slovenia),
            Federation::WRPFSpain => Some(Country::Spain),
            Federation::WRPFSRB => Some(Country::Serbia),
            Federation::WRPFSweden => Some(Country::Sweden),
            Federation::WRPFUK => Some(Country::UK),
            Federation::WUAP => None,
            Federation::WUAPAUT => Some(Country::Austria),
            Federation::WUAPSVK => Some(Country::Slovakia),
            Federation::XPC => Some(Country::USA),
            Federation::XPCPoland => Some(Country::Poland),
        }
    }

    /// The parent federation that provides sanction, if any.
    pub fn sanctioning_body(self, date: Date) -> Option<Federation> {
        match self {
            Federation::_365Strong => None,
            Federation::AAP => Some(Federation::GPA),
            Federation::AAPLF => Some(Federation::IPF),
            Federation::AAU => None,
            Federation::ACHIPO => Some(Federation::GPA),
            Federation::ACPA => Some(Federation::WPA),
            Federation::ADAU => None,
            Federation::ADFPA => None,
            Federation::ADFPF => Some(Federation::WDFPF),
            Federation::AEP => Some(Federation::IPF),
            Federation::AFPF => None,
            Federation::AfricanPF => Some(Federation::IPF),
            Federation::AIWBPA => Some(Federation::IPF),
            Federation::AmericanSA => None,
            Federation::ANPPC => None,
            Federation::APA => Some(Federation::WPA),
            Federation::APC => Some(Federation::WUAP),
            Federation::APF => Some(Federation::WPC),
            Federation::APU => Some(Federation::IPF),
            Federation::AsianPF => Some(Federation::IPF),
            Federation::AusDFPF => Some(Federation::WDFPF),
            Federation::AusPF => Some(Federation::IPF),
            Federation::AusPL => Some(Federation::IPL),
            Federation::AWF => None,
            Federation::BAWLA => Some(Federation::IPF),
            Federation::BB => None,
            Federation::BDFPA => Some(Federation::WDFPF),
            Federation::BelPF => Some(Federation::IPF),
            Federation::BP => Some(Federation::IPF),
            Federation::BPC => {
                // The BPC was WPC-affiliated until 2012.
                if date.year() >= 2012 {
                    None
                } else {
                    Some(Federation::WPC)
                }
            }
            Federation::BPF => Some(Federation::WRPF),
            Federation::BPO => Some(Federation::WPF),
            Federation::BPU => {
                // The BPU has been WPC-affiliated since 2013.
                if date.year() >= 2013 {
                    Some(Federation::WPC)
                } else {
                    None
                }
            }
            Federation::BVDG => Some(Federation::IPF),
            Federation::BVDK => Some(Federation::IPF),
            Federation::CanadaUA => None,
            Federation::CAPO => Some(Federation::GPA),
            Federation::CAPONZ => Some(Federation::GPA),
            Federation::CAST => Some(Federation::GPC),
            Federation::CBLB => Some(Federation::IPF),
            Federation::ChinaPA => Some(Federation::GPA),
            Federation::ColPF => Some(Federation::IPF),
            Federation::CommonwealthPF => Some(Federation::IPF),
            Federation::CONBRAP => Some(Federation::GPC),
            Federation::CPA => Some(Federation::WPA),
            Federation::CPC => Some(Federation::WPC),
            Federation::CPF => Some(Federation::WPF),
            Federation::CPL => Some(Federation::IPL),
            Federation::CPO => Some(Federation::WPC),
            Federation::CPU => Some(Federation::IPF),
            Federation::CRPEBA => None,
            Federation::CSST => Some(Federation::IPF),
            Federation::CzechiaUA => None,
            Federation::DBKV => None,
            Federation::DSF => Some(Federation::IPF),
            Federation::EPC => {
                // The EPC was IPL-affiliated until 2018.
                if date.year() >= 2018 {
                    None
                } else {
                    Some(Federation::IPL)
                }
            }
            Federation::EnglandUA => None,
            Federation::EPA => Some(Federation::IPF),
            Federation::EPF => Some(Federation::IPF),
            Federation::ESDT => None,
            Federation::FALPO => Some(Federation::IPF),
            Federation::FBPD => None,
            Federation::FCA => None,
            Federation::FCST => Some(Federation::GPC),
            Federation::FECAPOLIF => Some(Federation::IPF),
            Federation::FEMEPO => Some(Federation::IPF),
            Federation::FEPOA => Some(Federation::GPC),
            Federation::FESUPO => Some(Federation::IPF),
            Federation::FFForce => Some(Federation::IPF),
            Federation::FHSAA => None,
            Federation::FIAP => None,
            Federation::FIPL => Some(Federation::IPF),
            Federation::FPO => Some(Federation::IPA),
            Federation::FPR => Some(Federation::IPF),
            Federation::FRPL => Some(Federation::IPF),
            Federation::GDFPF => Some(Federation::WDFPF),
            Federation::GermanyUA => None,
            Federation::GlobalPU => Some(Federation::GPC),
            Federation::GPA => Some(Federation::GPA),
            Federation::GPACRO => Some(Federation::GPA),
            Federation::GPC => Some(Federation::GPC),
            Federation::GPCAUS => Some(Federation::GPC),
            Federation::GPCCAN => Some(Federation::GPC),
            Federation::GPCGB => Some(Federation::GPC),
            Federation::GPCIRL => Some(Federation::GPC),
            Federation::GPCISR => Some(Federation::GPC),
            Federation::GPCLAT => Some(Federation::GPC),
            Federation::GPCNZ => Some(Federation::GPC),
            Federation::GPCUKR => Some(Federation::GPC),
            Federation::GPCUSA => Some(Federation::GPC),
            Federation::GPCRUS => Some(Federation::GPC),
            Federation::GPCWUAPCRO => Some(Federation::GPC),
            Federation::GPF => None,
            Federation::GPU => Some(Federation::WPU),
            Federation::GRAWA => Some(Federation::IRP),
            Federation::GSFBelarus => None,
            Federation::Hardcore => None,
            Federation::HERC => None,
            Federation::CroatiaUA => None,
            Federation::HKWPA => Some(Federation::IPF),
            Federation::HPC => Some(Federation::WPC),
            Federation::HPLS => Some(Federation::IPF),
            Federation::HPLSUA => None,
            Federation::HPO => None,
            Federation::HTPL => None,
            Federation::Hunpower => Some(Federation::IPF),
            Federation::IBSA => None,
            Federation::IDFPA => None,
            Federation::IDFPF => Some(Federation::WDFPF),
            Federation::IKF => Some(Federation::GPC),
            Federation::ILPA => Some(Federation::GPA),
            Federation::IPA => None,
            Federation::IPC => None,
            Federation::IPF => Some(Federation::IPF),
            Federation::IPL => Some(Federation::IPL),
            Federation::IPLNZ => Some(Federation::IPL),
            Federation::IrelandUA => None,
            Federation::IrishPF => Some(Federation::IPF),
            Federation::IrishPO => Some(Federation::IPL),
            Federation::IronBoy => None,
            Federation::IRP => None,
            Federation::JPA => Some(Federation::IPF),
            Federation::KPF => Some(Federation::IPF),
            Federation::KRAFT => Some(Federation::IPF),
            Federation::KuwaitPL => Some(Federation::IPL),
            Federation::LGBT => None,
            Federation::LHSPLA => None,
            Federation::LJTF => Some(Federation::IPF),
            Federation::LMP => Some(Federation::IPL),
            Federation::LPF => Some(Federation::IPF),
            Federation::MHSAA => None,
            Federation::MHSPLA => None,
            Federation::MM => None,
            Federation::MPA => None,
            Federation::NAP => Some(Federation::IPA),
            Federation::NAPF => Some(Federation::IPF),
            Federation::NASA => None,
            Federation::NaturalPA => None,
            Federation::NauruPF => Some(Federation::IPF),
            Federation::NextGenPF => None,
            Federation::NIPF => Some(Federation::IPF),
            Federation::NORCAL => None,
            Federation::NordicPF => Some(Federation::IPF),
            Federation::NPA => None,
            Federation::NPB => Some(Federation::IPF),
            Federation::NSF => Some(Federation::IPF),
            Federation::NZPF => Some(Federation::IPF),
            Federation::NZAWLA => Some(Federation::IPF),
            Federation::NZUA => None,
            Federation::OceaniaPF => {
                // PA lost IPF affiliation in 2018, replaced by the ORPF.
                if date.year() >= 2018 {
                    Some(Federation::WP)
                } else {
                    Some(Federation::IPF)
                }
            }
            Federation::ORPF => Some(Federation::IPF),
            Federation::OEVK => Some(Federation::IPF),
            Federation::PA => {
                // PA lost IPF affiliation in 2018, replaced by the APU.
                if date.year() >= 2018 {
                    Some(Federation::WP)
                } else {
                    Some(Federation::IPF)
                }
            }
            Federation::PAP => Some(Federation::IPF),
            Federation::PHPL => Some(Federation::GPA),
            Federation::PLSS => Some(Federation::IPF),
            Federation::PLZS => Some(Federation::IPF),
            Federation::PNGPF => Some(Federation::IPF),
            Federation::PolandUA => None,
            Federation::PoliceAL => None,
            Federation::PRIDE => None,
            Federation::ProRaw => None,
            Federation::PRPA => None,
            Federation::PZKFiTS => Some(Federation::IPF),
            Federation::RAW => None,
            Federation::RAWCAN => None,
            Federation::RAWIceland => None,
            Federation::RawIronPL => None,
            Federation::RAWUKR => None,
            Federation::RAWU => None,
            Federation::RhinoPC => Some(Federation::GPC),
            Federation::RPS => None,
            Federation::RPU => None,
            Federation::RUPC => None,
            Federation::RussiaUA => None,
            Federation::SAPF => Some(Federation::IPF),
            Federation::SAST => Some(Federation::GPC),
            Federation::ScottishPL => Some(Federation::IPF),
            Federation::SCI => None,
            Federation::SCT => None,
            Federation::SDFPF => Some(Federation::WDFPF),
            Federation::SLP => None,
            Federation::SPA => None,
            Federation::SPF => None,
            Federation::SPFIRL => Some(Federation::SPF),
            Federation::SPSS => None,
            Federation::SSA => None,
            Federation::SSF => Some(Federation::IPF),
            Federation::SVNL => Some(Federation::IPF),
            Federation::SwissPL => {
                // Not sure about the exact date of the switch to IPF.
                if date.year() >= 2018 {
                    Some(Federation::IPF)
                } else {
                    None
                }
            }
            Federation::ThaiPF => Some(Federation::IPF),
            Federation::THSPA => None,
            Federation::THSWPA => None,
            Federation::UDFPF => Some(Federation::WDFPF),
            Federation::UkrainePA => None,
            Federation::UkrainePO => None,
            Federation::UPA => None,
            Federation::UPC => Some(Federation::UPC),
            Federation::UPCGermany => Some(Federation::UPC),
            Federation::UkrainePF => Some(Federation::IPF),
            Federation::UPL => Some(Federation::IPL),
            Federation::USABA => Some(Federation::IBSA),
            Federation::USABPA => None,
            Federation::USAUA => None,
            Federation::USAPL => Some(Federation::IPF),
            Federation::USARawBP => None,
            Federation::USMilAbroad => None,
            Federation::USPS => None,
            Federation::USPF => None,
            Federation::USPA => Some(Federation::IPL),
            Federation::USSF => None,
            Federation::USSports => None,
            Federation::USVIPF => Some(Federation::IPF),
            Federation::VietnamPA => Some(Federation::GPA),
            Federation::Vityaz => None,
            Federation::VPF => Some(Federation::IPF),
            Federation::WABDL => None,
            Federation::WarriorPLF => None,
            Federation::WBC => None,
            Federation::WDFPF => Some(Federation::WDFPF),
            Federation::WelshPA => Some(Federation::IPF),
            Federation::WP => Some(Federation::WP),
            Federation::WPA => None,
            Federation::WPAGEO => Some(Federation::WPA),
            Federation::WPARUS => Some(Federation::WPA),
            Federation::WPAU => None,
            Federation::WPC => Some(Federation::WPC),
            Federation::WPCFinland => Some(Federation::WPC),
            Federation::WPCFrance => Some(Federation::WPC),
            Federation::WPCGermany => Some(Federation::WPC),
            Federation::WPCIceland => Some(Federation::WPC),
            Federation::WPCIsrael => Some(Federation::WPC),
            Federation::WPCItaly => Some(Federation::WPC),
            Federation::WPCKAZ => Some(Federation::WPC),
            Federation::WPCKGZ => Some(Federation::WPC),
            Federation::WPCLatvia => Some(Federation::WPC),
            Federation::WPCMoldova => Some(Federation::WPC),
            Federation::WPCPortugal => Some(Federation::WPC),
            Federation::WPCRUS => Some(Federation::WPC),
            Federation::WPCSA => Some(Federation::WPC),
            Federation::WPCUKR => Some(Federation::WPC),
            Federation::WPF => None,
            Federation::WPFG => None,
            Federation::WPLeague => None,
            Federation::WPNZ => Some(Federation::WP),
            Federation::WPPO => None,
            Federation::WPRO => None,
            Federation::WPSF => Some(Federation::WPSF),
            Federation::WPSFBelarus => Some(Federation::WPSF),
            Federation::WPU => None,
            Federation::WPUF => None,
            Federation::WPURUS => None,
            Federation::WNPF => None,
            Federation::WRPF => Some(Federation::WRPF),
            Federation::WRPFAUS => Some(Federation::WRPF),
            Federation::WRPFBelarus => Some(Federation::WRPF),
            Federation::WRPFCAN => Some(Federation::WRPF),
            Federation::WRPFIceland => Some(Federation::WRPF),
            Federation::WRPFIreland => Some(Federation::WRPF),
            Federation::WRPFItaly => Some(Federation::WRPF),
            Federation::WRPFKAZ => Some(Federation::WRPF),
            Federation::WRPFLithuania => Some(Federation::WRPF),
            Federation::WRPFPOL => Some(Federation::WRPF),
            Federation::WRPFPortugal => Some(Federation::WRPF),
            Federation::WRPFSlovakia => Some(Federation::WRPF),
            Federation::WRPFSlovenia => Some(Federation::WRPF),
            Federation::WRPFSpain => Some(Federation::WRPF),
            Federation::WRPFSRB => Some(Federation::WRPF),
            Federation::WRPFSweden => Some(Federation::WRPF),
            Federation::WRPFUK => Some(Federation::WRPF),
            Federation::WUAP => Some(Federation::WUAP),
            Federation::WUAPAUT => Some(Federation::WUAP),
            Federation::WUAPSVK => Some(Federation::WUAP),
            Federation::XPC => Some(Federation::XPC),
            Federation::XPCPoland => Some(Federation::XPC),
        }
    }

    /// Helper function for specifying the PointsSystem of federations under IPF rules.
    #[inline]
    fn ipf_rules_on(date: Date) -> PointsSystem {
        // The IPF and their affiliates developed a new federation-specific
        // formula beginning in 2019.
        if date.year() >= 2019 {
            PointsSystem::IPFPoints
        } else {
            PointsSystem::Wilks
        }
    }

    /// Which points system is default for a federation's meet.
    pub fn default_points(self, date: Date) -> PointsSystem {
        match self {
            Federation::_365Strong => PointsSystem::Wilks,
            Federation::AAP => PointsSystem::Wilks,
            Federation::AAPLF => PointsSystem::Wilks,
            Federation::AAU => PointsSystem::Wilks,
            Federation::ACHIPO => PointsSystem::Wilks,
            Federation::ACPA => PointsSystem::Wilks,
            Federation::ADAU => PointsSystem::Wilks,
            Federation::ADFPA => PointsSystem::Wilks,
            Federation::ADFPF => PointsSystem::Wilks,
            Federation::AEP => Federation::ipf_rules_on(date),
            Federation::AFPF => PointsSystem::Wilks,
            Federation::AfricanPF => Federation::ipf_rules_on(date),
            Federation::AIWBPA => Federation::ipf_rules_on(date),
            Federation::AmericanSA => PointsSystem::Wilks,
            Federation::ANPPC => PointsSystem::Wilks,
            Federation::APA => PointsSystem::Wilks,
            Federation::APC => PointsSystem::Wilks,
            Federation::APF => PointsSystem::Glossbrenner,
            Federation::APU => Federation::ipf_rules_on(date),
            Federation::AsianPF => Federation::ipf_rules_on(date),
            Federation::AusDFPF => PointsSystem::Wilks,
            Federation::AusPF => PointsSystem::Wilks,
            Federation::AusPL => PointsSystem::Wilks,
            Federation::AWF => PointsSystem::Wilks,
            Federation::BAWLA => PointsSystem::Wilks,
            Federation::BB => PointsSystem::Wilks,
            Federation::BDFPA => PointsSystem::Wilks,
            Federation::BelPF => Federation::ipf_rules_on(date),
            Federation::BP => Federation::ipf_rules_on(date),
            Federation::BPC => PointsSystem::Wilks,
            Federation::BPF => PointsSystem::Wilks,
            Federation::BPO => PointsSystem::Wilks,
            Federation::BPU => PointsSystem::Glossbrenner,
            Federation::BVDG => PointsSystem::Wilks,
            Federation::BVDK => {
                // Federation voted in Nov 2019 to switch to Dots in 2020.
                if date.year() >= 2020 {
                    PointsSystem::Dots
                } else {
                    Federation::ipf_rules_on(date)
                }
            }
            Federation::CanadaUA => PointsSystem::Wilks,
            Federation::CAPO => PointsSystem::Glossbrenner,
            Federation::CAPONZ => PointsSystem::Glossbrenner,
            Federation::CAST => PointsSystem::Wilks,
            Federation::CBLB => Federation::ipf_rules_on(date),
            Federation::ChinaPA => PointsSystem::Wilks,
            Federation::ColPF => Federation::ipf_rules_on(date),
            Federation::CommonwealthPF => Federation::ipf_rules_on(date),
            Federation::CONBRAP => PointsSystem::Glossbrenner,
            Federation::CPA => PointsSystem::Wilks,
            Federation::CPC => PointsSystem::Wilks,
            Federation::CPF => PointsSystem::Wilks,
            Federation::CPL => PointsSystem::Wilks,
            Federation::CPO => PointsSystem::Wilks,
            Federation::CPU => Federation::ipf_rules_on(date),
            Federation::CRPEBA => PointsSystem::Wilks,
            Federation::CSST => PointsSystem::Wilks,
            Federation::DBKV => PointsSystem::Wilks,
            Federation::CzechiaUA => PointsSystem::Wilks,
            Federation::DSF => Federation::ipf_rules_on(date),
            Federation::EPC => PointsSystem::Wilks,
            Federation::EnglandUA => PointsSystem::Wilks,
            Federation::EPA => Federation::ipf_rules_on(date),
            Federation::EPF => Federation::ipf_rules_on(date),
            Federation::ESDT => PointsSystem::Wilks,
            Federation::FALPO => Federation::ipf_rules_on(date),
            Federation::FBPD => PointsSystem::Wilks,
            Federation::FCA => PointsSystem::Wilks,
            Federation::FCST => PointsSystem::Wilks,
            Federation::FECAPOLIF => Federation::ipf_rules_on(date),
            Federation::FEMEPO => Federation::ipf_rules_on(date),
            Federation::FEPOA => PointsSystem::Wilks,
            Federation::FESUPO => Federation::ipf_rules_on(date),
            Federation::FFForce => Federation::ipf_rules_on(date),
            Federation::FHSAA => PointsSystem::Wilks,
            Federation::FIAP => Federation::ipf_rules_on(date),
            Federation::FIPL => Federation::ipf_rules_on(date),
            Federation::FPO => PointsSystem::Wilks,
            Federation::FPR => Federation::ipf_rules_on(date),
            Federation::FRPL => Federation::ipf_rules_on(date),
            Federation::GDFPF => PointsSystem::Wilks,
            Federation::GermanyUA => PointsSystem::Wilks,
            Federation::GlobalPU => PointsSystem::Glossbrenner,
            Federation::GPA => PointsSystem::Wilks,
            Federation::GPACRO => PointsSystem::Wilks,
            Federation::GPC => PointsSystem::Glossbrenner,
            Federation::GPCAUS => PointsSystem::Glossbrenner,
            Federation::GPCCAN => PointsSystem::Glossbrenner,
            Federation::GPCGB => PointsSystem::Glossbrenner,
            Federation::GPCIRL => PointsSystem::Glossbrenner,
            Federation::GPCISR => PointsSystem::Glossbrenner,
            Federation::GPCLAT => PointsSystem::Glossbrenner,
            Federation::GPCNZ => PointsSystem::Glossbrenner,
            Federation::GPCUKR => PointsSystem::Glossbrenner,
            Federation::GPCUSA => PointsSystem::Glossbrenner,
            Federation::GPCRUS => PointsSystem::Glossbrenner,
            Federation::GPCWUAPCRO => PointsSystem::Wilks,
            Federation::GPF => PointsSystem::Wilks,
            Federation::GPU => PointsSystem::Wilks,
            Federation::GRAWA => PointsSystem::Wilks,
            Federation::GSFBelarus => PointsSystem::Wilks,
            Federation::Hardcore => PointsSystem::Wilks,
            Federation::HERC => PointsSystem::Wilks,
            Federation::CroatiaUA => PointsSystem::Wilks,
            Federation::HKWPA => Federation::ipf_rules_on(date),
            Federation::HPC => PointsSystem::Wilks,
            Federation::HPLS => Federation::ipf_rules_on(date),
            Federation::HPLSUA => PointsSystem::Wilks,
            Federation::HPO => PointsSystem::Wilks,
            Federation::HTPL => PointsSystem::Wilks,
            Federation::Hunpower => Federation::ipf_rules_on(date),
            Federation::IBSA => PointsSystem::Wilks,
            Federation::IDFPA => PointsSystem::Wilks,
            Federation::IDFPF => PointsSystem::Wilks,
            Federation::IKF => PointsSystem::Wilks,
            Federation::ILPA => PointsSystem::Wilks,
            Federation::IPA => PointsSystem::Wilks,
            Federation::IPC => PointsSystem::Wilks,
            Federation::IPF => Federation::ipf_rules_on(date),
            Federation::IPL => PointsSystem::Wilks,
            Federation::IPLNZ => PointsSystem::Wilks,
            Federation::IrelandUA => PointsSystem::Wilks,
            Federation::IrishPF => {
                // On 2020-02-16, IrishPF voted to immediately switch to Dots.
                if date > Date::from_parts(2020, 02, 16) {
                    PointsSystem::Dots
                } else {
                    Federation::ipf_rules_on(date)
                }
            }
            Federation::IrishPO => PointsSystem::Wilks,
            Federation::IronBoy => PointsSystem::Wilks,
            Federation::IRP => PointsSystem::Wilks,
            Federation::JPA => Federation::ipf_rules_on(date),
            Federation::KPF => Federation::ipf_rules_on(date),
            Federation::KRAFT => {
                // On 2020-03-04, KRAFT announced that they voted for Dots since 02-29.
                if date >= Date::from_parts(2020, 02, 29) {
                    PointsSystem::Dots
                } else {
                    Federation::ipf_rules_on(date)
                }
            }
            Federation::KuwaitPL => PointsSystem::Wilks,
            Federation::LGBT => PointsSystem::Wilks,
            Federation::LHSPLA => PointsSystem::Wilks,
            Federation::LJTF => Federation::ipf_rules_on(date),
            Federation::LMP => PointsSystem::Wilks,
            Federation::LPF => Federation::ipf_rules_on(date),
            Federation::MHSAA => PointsSystem::Wilks,
            Federation::MHSPLA => PointsSystem::Wilks,
            Federation::MM => PointsSystem::Wilks,
            Federation::MPA => PointsSystem::Wilks,
            Federation::NAP => PointsSystem::Wilks,
            Federation::NAPF => Federation::ipf_rules_on(date),
            Federation::NASA => PointsSystem::NASA,
            Federation::NaturalPA => PointsSystem::Wilks,
            Federation::NauruPF => Federation::ipf_rules_on(date),
            Federation::NextGenPF => PointsSystem::Wilks,
            Federation::NORCAL => PointsSystem::Wilks,
            Federation::NIPF => Federation::ipf_rules_on(date),
            Federation::NordicPF => Federation::ipf_rules_on(date),
            Federation::NPA => PointsSystem::Wilks,
            Federation::NPB => Federation::ipf_rules_on(date),
            Federation::NSF => Federation::ipf_rules_on(date),
            Federation::NZPF => Federation::ipf_rules_on(date),
            Federation::NZAWLA => Federation::ipf_rules_on(date),
            Federation::NZUA => PointsSystem::Wilks,
            Federation::OceaniaPF => PointsSystem::Wilks,
            Federation::ORPF => Federation::ipf_rules_on(date),
            Federation::OEVK => Federation::ipf_rules_on(date),
            Federation::PA => PointsSystem::Wilks,
            Federation::PoliceAL => PointsSystem::Wilks,
            Federation::PAP => Federation::ipf_rules_on(date),
            Federation::PHPL => PointsSystem::Reshel,
            Federation::PLSS => Federation::ipf_rules_on(date),
            Federation::PLZS => Federation::ipf_rules_on(date),
            Federation::PNGPF => Federation::ipf_rules_on(date),
            Federation::PolandUA => PointsSystem::Wilks,
            Federation::PRIDE => PointsSystem::Wilks,
            Federation::ProRaw => PointsSystem::Glossbrenner,
            Federation::PRPA => PointsSystem::Wilks,
            Federation::PZKFiTS => Federation::ipf_rules_on(date),
            Federation::RAW => PointsSystem::Wilks,
            Federation::RAWCAN => PointsSystem::Wilks,
            Federation::RAWIceland => PointsSystem::Wilks,
            Federation::RawIronPL => PointsSystem::Wilks,
            Federation::RAWUKR => PointsSystem::Wilks,
            Federation::RAWU => PointsSystem::Wilks,
            Federation::RhinoPC => PointsSystem::Glossbrenner,
            Federation::RPS => PointsSystem::Wilks,
            Federation::RPU => PointsSystem::Wilks,
            Federation::RUPC => PointsSystem::Wilks,
            Federation::RussiaUA => PointsSystem::Wilks,
            Federation::SAPF => Federation::ipf_rules_on(date),
            Federation::SAST => PointsSystem::Glossbrenner,
            Federation::ScottishPL => Federation::ipf_rules_on(date),
            Federation::SCI => PointsSystem::Wilks,
            Federation::SCT => PointsSystem::Wilks,
            Federation::SDFPF => PointsSystem::Wilks,
            Federation::SLP => PointsSystem::Wilks,
            Federation::SPA => PointsSystem::Wilks,
            Federation::SPF => PointsSystem::SchwartzMalone,
            Federation::SPFIRL => PointsSystem::SchwartzMalone,
            Federation::SPSS => PointsSystem::Wilks,
            Federation::SSA => PointsSystem::Wilks,
            Federation::SSF => Federation::ipf_rules_on(date),
            Federation::SVNL => Federation::ipf_rules_on(date),
            Federation::SwissPL => {
                // Federation voted in Nov 2019 to switch to Dots in 2020.
                if date.year() >= 2020 {
                    PointsSystem::Dots
                } else {
                    Federation::ipf_rules_on(date)
                }
            }
            Federation::ThaiPF => Federation::ipf_rules_on(date),
            Federation::THSPA => PointsSystem::Wilks,
            Federation::THSWPA => PointsSystem::Wilks,
            Federation::UDFPF => PointsSystem::Wilks,
            Federation::UkrainePA => PointsSystem::Wilks,
            Federation::UkrainePO => PointsSystem::Wilks,
            Federation::UPA => PointsSystem::Wilks,
            Federation::UPC => PointsSystem::Wilks,
            Federation::UPCGermany => PointsSystem::Glossbrenner,
            Federation::UkrainePF => Federation::ipf_rules_on(date),
            Federation::UPL => PointsSystem::Wilks,
            Federation::USABA => PointsSystem::Wilks,
            Federation::USABPA => PointsSystem::Wilks,
            Federation::USAUA => PointsSystem::Wilks,
            Federation::USAPL => Federation::ipf_rules_on(date),
            Federation::USARawBP => PointsSystem::Wilks,
            Federation::USMilAbroad => PointsSystem::Wilks,
            Federation::USPS => PointsSystem::Wilks,
            Federation::USPF => PointsSystem::Wilks,
            Federation::USPA => PointsSystem::Wilks,
            Federation::USSF => PointsSystem::Wilks,
            Federation::USSports => PointsSystem::Wilks,
            Federation::USVIPF => Federation::ipf_rules_on(date),
            Federation::VietnamPA => PointsSystem::Wilks,
            Federation::Vityaz => PointsSystem::Wilks,
            Federation::VPF => Federation::ipf_rules_on(date),
            Federation::WABDL => PointsSystem::Wilks,
            Federation::WarriorPLF => PointsSystem::Wilks,
            Federation::WBC => PointsSystem::Wilks,
            Federation::WDFPF => PointsSystem::Wilks,
            Federation::WelshPA => Federation::ipf_rules_on(date),
            Federation::WP => PointsSystem::Wilks,
            Federation::WPA => PointsSystem::Wilks,
            Federation::WPAGEO => PointsSystem::Wilks,
            Federation::WPARUS => PointsSystem::Wilks,
            Federation::WPAU => PointsSystem::Wilks,
            Federation::WPC => PointsSystem::Glossbrenner,
            Federation::WPCFinland => PointsSystem::Glossbrenner,
            Federation::WPCFrance => PointsSystem::Glossbrenner,
            Federation::WPCGermany => PointsSystem::Glossbrenner,
            Federation::WPCIceland => PointsSystem::Glossbrenner,
            Federation::WPCIsrael => PointsSystem::Glossbrenner,
            Federation::WPCItaly => PointsSystem::Glossbrenner,
            Federation::WPCKAZ => PointsSystem::Glossbrenner,
            Federation::WPCKGZ => PointsSystem::Glossbrenner,
            Federation::WPCLatvia => PointsSystem::Glossbrenner,
            Federation::WPCMoldova => PointsSystem::Glossbrenner,
            Federation::WPCPortugal => PointsSystem::Glossbrenner,
            Federation::WPCRUS => PointsSystem::Glossbrenner,
            Federation::WPCSA => PointsSystem::Glossbrenner,
            Federation::WPCUKR => PointsSystem::Glossbrenner,
            Federation::WPF => PointsSystem::Wilks,
            Federation::WPFG => PointsSystem::Wilks,
            Federation::WPLeague => PointsSystem::Wilks,
            Federation::WPNZ => PointsSystem::Wilks,
            Federation::WPPO => PointsSystem::AH,
            Federation::WPRO => PointsSystem::Wilks,
            Federation::WPSF => PointsSystem::Wilks,
            Federation::WPSFBelarus => PointsSystem::Wilks,
            Federation::WPU => PointsSystem::Wilks,
            Federation::WPUF => PointsSystem::Wilks,
            Federation::WPURUS => PointsSystem::Wilks,
            Federation::WNPF => PointsSystem::Wilks,
            Federation::WRPF => PointsSystem::Wilks,
            Federation::WRPFAUS => PointsSystem::Wilks,
            Federation::WRPFBelarus => PointsSystem::Wilks,
            Federation::WRPFCAN => PointsSystem::Wilks,
            Federation::WRPFIceland => PointsSystem::Wilks,
            Federation::WRPFIreland => PointsSystem::Wilks,
            Federation::WRPFItaly => PointsSystem::Wilks,
            Federation::WRPFKAZ => PointsSystem::Wilks,
            Federation::WRPFLithuania => PointsSystem::Wilks,
            Federation::WRPFPOL => PointsSystem::Wilks,
            Federation::WRPFPortugal => PointsSystem::Wilks,
            Federation::WRPFSlovakia => PointsSystem::Wilks,
            Federation::WRPFSlovenia => PointsSystem::Wilks,
            Federation::WRPFSpain => PointsSystem::Wilks,
            Federation::WRPFSRB => PointsSystem::Wilks,
            Federation::WRPFSweden => PointsSystem::Wilks,
            Federation::WRPFUK => PointsSystem::Wilks,
            Federation::WUAP => PointsSystem::Wilks,
            Federation::WUAPAUT => PointsSystem::Wilks,
            Federation::WUAPSVK => PointsSystem::Wilks,
            Federation::XPC => PointsSystem::Wilks,
            Federation::XPCPoland => PointsSystem::Wilks,
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
