//! Internationalization facilities.

use serde;
use serde::ser::Serialize;
use serde_json;
use strum::IntoEnumIterator;
use opltypes::*;

use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use opldb;

/// List of languages accepted by the project, in ISO 639-1 code.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, EnumIter, EnumString, Serialize)]
pub enum Language {
    /// German, without regional variance.
    de,
    /// English, without regional variance (US).
    en,
    /// Esperanto.
    eo,
    /// Spanish.
    es,
    /// Finnish.
    fi,
    /// French.
    fr,
    /// Croatian.
    hr,
    /// Hungarian.
    hu,
    /// Italian.
    it,
    /// Japanese.
    ja,
    /// Polish.
    pl,
    /// Portuguese.
    pt,
    /// Slovenian.
    sl,
    /// Swedish.
    sv,
    /// Russian.
    ru,
    /// Turkish.
    tr,
    /// Ukrainian.
    uk,
    /// Vietnamese.
    vi,
    /// Chinese, written in Traditional Chinese script.
    #[serde(rename = "zh-Hant")]
    #[strum(to_string = "zh-Hant")]
    zh_hant,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Language::de => "de",
                Language::en => "en",
                Language::eo => "eo",
                Language::es => "es",
                Language::fi => "fi",
                Language::fr => "fr",
                Language::hr => "hr",
                Language::hu => "hu",
                Language::it => "it",
                Language::ja => "ja",
                Language::pl => "pl",
                Language::pt => "pt",
                Language::sl => "sl",
                Language::sv => "sv",
                Language::ru => "ru",
                Language::tr => "tr",
                Language::uk => "uk",
                Language::vi => "vi",
                Language::zh_hant => "zh-Hant",
            }
        )
    }
}

impl Language {
    /// Returns the units associated with the language.
    pub fn default_units(self) -> WeightUnits {
        match self {
            Language::en => WeightUnits::Lbs,
            _ => WeightUnits::Kg,
        }
    }

    /// Returns a list of available languages as strings.
    pub fn string_list() -> Vec<String> {
        Language::iter().map(|lang| lang.to_string()).collect()
    }
}

/// Helper struct to pass around language information.
pub struct Locale<'a> {
    pub langinfo: &'a LangInfo,
    pub language: Language,
    pub strings: &'a Translations,
    pub number_format: NumberFormat,
    pub units: WeightUnits,
}

impl<'a> Locale<'a> {
    pub fn new(
        langinfo: &'a LangInfo,
        language: Language,
        units: WeightUnits,
    ) -> Locale<'a> {
        Locale {
            langinfo,
            language,
            strings: langinfo.get_translations(language),
            number_format: language.number_format(),
            units,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UnitsTranslations {
    pub lbs: String,
    pub kg: String,
}

#[derive(Serialize, Deserialize)]
pub struct EquipmentTranslations {
    pub raw: String,
    pub wraps: String,
    pub single: String,
    pub multi: String,
    pub straps: String,
}

#[derive(Serialize, Deserialize)]
pub struct SexTranslations {
    pub m: String,
    pub f: String,
}

#[derive(Serialize, Deserialize)]
pub struct CountryTranslations {
    pub algeria: String,
    pub argentina: String,
    pub aruba: String,
    pub australia: String,
    pub austria: String,
    pub azerbaijan: String,
    pub belarus: String,
    pub belgium: String,
    pub brazil: String,
    pub britain: String,
    pub britishvirginislands: String,
    pub bulgaria: String,
    pub canada: String,
    pub caymanislands: String,
    pub china: String,
    pub colombia: String,
    pub costarica: String,
    pub cotedivoire: String,
    pub czechia: String,
    pub denmark: String,
    pub ecuador: String,
    pub egypt: String,
    pub elsalvador: String,
    pub england: String,
    pub estonia: String,
    pub fiji: String,
    pub finland: String,
    pub france: String,
    pub germany: String,
    pub greece: String,
    pub guatemala: String,
    pub guyana: String,
    pub hongkong: String,
    pub hungary: String,
    pub iceland: String,
    pub india: String,
    pub indonesia: String,
    pub ireland: String,
    pub israel: String,
    pub italy: String,
    pub iran: String,
    pub japan: String,
    pub kazakhstan: String,
    pub latvia: String,
    pub lithuania: String,
    pub luxembourg: String,
    pub malaysia: String,
    pub mexico: String,
    pub mongolia: String,
    pub morocco: String,
    pub netherlands: String,
    pub newcaledonia: String,
    pub newzealand: String,
    pub nicaragua: String,
    pub norway: String,
    pub northernireland: String,
    pub oman: String,
    pub papuanewguinea: String,
    pub peru: String,
    pub philippines: String,
    pub poland: String,
    pub portugal: String,
    pub puertorico: String,
    pub russia: String,
    pub samoa: String,
    pub scotland: String,
    pub serbia: String,
    pub singapore: String,
    pub slovakia: String,
    pub slovenia: String,
    pub southafrica: String,
    pub southkorea: String,
    pub spain: String,
    pub sweden: String,
    pub switzerland: String,
    pub tahiti: String,
    pub taiwan: String,
    pub turkey: String,
    pub uae: String,
    pub uk: String,
    pub ukraine: String,
    pub uruguay: String,
    pub usa: String,
    pub usvirginislands: String,
    pub uzbekistan: String,
    pub venezuela: String,
    pub vietnam: String,
    pub wales: String,
}

#[derive(Serialize, Deserialize)]
pub struct HeaderTranslations {
    pub rankings: String,
    pub meets: String,
    pub data: String,
    pub status: String,
    pub faq: String,
    pub contact: String,
    pub shop: String,
    pub supportus: String,
}

#[derive(Serialize, Deserialize)]
pub struct ColumnTranslations {
    pub place: String,
    pub formulaplace: String,
    pub liftername: String,
    pub federation: String,
    pub date: String,
    pub location: String,
    pub meetname: String,
    pub division: String,
    pub sex: String,
    pub age: String,
    pub equipment: String,
    pub weightclass: String,
    pub bodyweight: String,
    pub squat: String,
    pub bench: String,
    pub deadlift: String,
    pub total: String,
    pub wilks: String,
    pub mcculloch: String,
    pub glossbrenner: String,
    pub num_lifters: String,
}

#[derive(Serialize, Deserialize)]
pub struct ButtonTranslations {
    pub search: String,
}

#[derive(Serialize, Deserialize)]
pub struct SelectorTranslations {
    pub equipment: EquipmentSelectorTranslations,
    pub weightclass: WeightClassSelectorTranslations,
    pub sort: SortSelectorTranslations,
    pub year: YearSelectorTranslations,
    pub sex: SexSelectorTranslations,
    pub fed: FedSelectorTranslations,
}

#[derive(Serialize, Deserialize)]
pub struct EquipmentSelectorTranslations {
    pub raw: String,
    pub wraps: String,
    pub raw_wraps: String,
    pub single: String,
    pub multi: String,
}

#[derive(Serialize, Deserialize)]
pub struct WeightClassSelectorTranslations {
    pub all: String,
    pub traditional: String,
    pub ipfmen: String,
    pub ipfwomen: String,
}

#[derive(Serialize, Deserialize)]
pub struct SortSelectorTranslations {
    pub by_squat: String,
    pub by_bench: String,
    pub by_deadlift: String,
    pub by_total: String,
    pub by_allometric: String,
    pub by_glossbrenner: String,
    pub by_mcculloch: String,
    pub by_wilks: String,
    pub by_division: String,
    pub weight: String,
    pub points: String,
}

#[derive(Serialize, Deserialize)]
pub struct YearSelectorTranslations {
    pub all: String,
}

#[derive(Serialize, Deserialize)]
pub struct SexSelectorTranslations {
    pub all: String,
    pub m: String,
    pub f: String,
}

#[derive(Serialize, Deserialize)]
pub struct FedSelectorTranslations {
    pub all: String,
    pub all_tested: String,
    pub all_amateur: String,
    pub international: String,
    pub regional: String,
    pub all_usa: String,
    pub all_argentina: String,
    pub all_australia: String,
    pub all_canada: String,
    pub all_czechia: String,
    pub all_finland: String,
    pub all_germany: String,
    pub all_ireland: String,
    pub all_israel: String,
    pub all_russia: String,
    pub all_uk: String,
    pub all_ukraine: String,
}

#[derive(Serialize, Deserialize)]
pub struct LifterPageTranslations {
    pub personal_bests: String,
    pub competition_results: String,
}

#[derive(Serialize, Deserialize)]
pub struct Translations {
    pub units: UnitsTranslations,
    pub equipment: EquipmentTranslations,
    pub sex: SexTranslations,
    pub header: HeaderTranslations,
    pub columns: ColumnTranslations,
    pub country: CountryTranslations,
    pub buttons: ButtonTranslations,
    pub selectors: SelectorTranslations,
    pub lifter_page: LifterPageTranslations,
}

/// Owner struct of all translation state.
#[derive(Default)]
pub struct LangInfo {
    de: Option<Translations>,
    en: Option<Translations>,
    eo: Option<Translations>,
    es: Option<Translations>,
    fi: Option<Translations>,
    fr: Option<Translations>,
    hr: Option<Translations>,
    hu: Option<Translations>,
    it: Option<Translations>,
    ja: Option<Translations>,
    pl: Option<Translations>,
    pt: Option<Translations>,
    sl: Option<Translations>,
    sv: Option<Translations>,
    ru: Option<Translations>,
    tr: Option<Translations>,
    uk: Option<Translations>,
    vi: Option<Translations>,
    zh_hant: Option<Translations>,
}

impl LangInfo {
    pub fn load_translations(
        &mut self,
        language: Language,
        filename: &str,
    ) -> Result<(), Box<Error>> {
        let file = File::open(filename)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;

        let trans = serde_json::from_str(&contents)?;

        match language {
            Language::de => self.de = trans,
            Language::en => self.en = trans,
            Language::eo => self.eo = trans,
            Language::es => self.es = trans,
            Language::fi => self.fi = trans,
            Language::fr => self.fr = trans,
            Language::hr => self.hr = trans,
            Language::hu => self.hu = trans,
            Language::it => self.it = trans,
            Language::ja => self.ja = trans,
            Language::pl => self.pl = trans,
            Language::pt => self.pt = trans,
            Language::sl => self.sl = trans,
            Language::sv => self.sv = trans,
            Language::ru => self.ru = trans,
            Language::tr => self.tr = trans,
            Language::uk => self.uk = trans,
            Language::vi => self.vi = trans,
            Language::zh_hant => self.zh_hant = trans,
        };

        Ok(())
    }

    pub fn get_translations(&self, language: Language) -> &Translations {
        match language {
            Language::de => self.de.as_ref().unwrap(),
            Language::en => self.en.as_ref().unwrap(),
            Language::eo => self.eo.as_ref().unwrap(),
            Language::es => self.es.as_ref().unwrap(),
            Language::fi => self.fi.as_ref().unwrap(),
            Language::fr => self.fr.as_ref().unwrap(),
            Language::hr => self.hr.as_ref().unwrap(),
            Language::hu => self.hu.as_ref().unwrap(),
            Language::it => self.it.as_ref().unwrap(),
            Language::ja => self.ja.as_ref().unwrap(),
            Language::pl => self.pl.as_ref().unwrap(),
            Language::pt => self.pt.as_ref().unwrap(),
            Language::sl => self.sl.as_ref().unwrap(),
            Language::sv => self.sv.as_ref().unwrap(),
            Language::ru => self.ru.as_ref().unwrap(),
            Language::tr => self.tr.as_ref().unwrap(),
            Language::uk => self.uk.as_ref().unwrap(),
            Language::vi => self.vi.as_ref().unwrap(),
            Language::zh_hant => self.zh_hant.as_ref().unwrap(),
        }
    }
}

impl Translations {
    pub fn translate_equipment(&self, equip: Equipment) -> &str {
        match equip {
            Equipment::Raw => &self.equipment.raw,
            Equipment::Wraps => &self.equipment.wraps,
            Equipment::Single => &self.equipment.single,
            Equipment::Multi => &self.equipment.multi,
            Equipment::Straps => &self.equipment.straps,
        }
    }

    pub fn translate_sex(&self, sex: Sex) -> &str {
        match sex {
            Sex::M => &self.sex.m,
            Sex::F => &self.sex.f,
        }
    }

    pub fn translate_country(&self, country: Country) -> &str {
        match country {
            Country::Algeria => &self.country.algeria,
            Country::Argentina => &self.country.argentina,
            Country::Aruba => &self.country.aruba,
            Country::Austria => &self.country.austria,
            Country::Australia => &self.country.australia,
            Country::Azerbaijan => &self.country.azerbaijan,
            Country::Belarus => &self.country.belarus,
            Country::Belgium => &self.country.belgium,
            Country::Brazil => &self.country.brazil,
            Country::Britain => &self.country.britain,
            Country::BritishVirginIslands => &self.country.britishvirginislands,
            Country::Bulgaria => &self.country.bulgaria,
            Country::Canada => &self.country.canada,
            Country::CaymanIslands => &self.country.caymanislands,
            Country::China => &self.country.china,
            Country::Colombia => &self.country.colombia,
            Country::CostaRica => &self.country.costarica,
            Country::CoteDIvoire => &self.country.cotedivoire,
            Country::Czechia => &self.country.czechia,
            Country::Denmark => &self.country.denmark,
            Country::Ecuador => &self.country.ecuador,
            Country::Egypt => &self.country.egypt,
            Country::ElSalvador => &self.country.elsalvador,
            Country::England => &self.country.england,
            Country::Estonia => &self.country.estonia,
            Country::Fiji => &self.country.fiji,
            Country::Finland => &self.country.finland,
            Country::France => &self.country.france,
            Country::Germany => &self.country.germany,
            Country::Greece => &self.country.greece,
            Country::Guatemala => &self.country.guatemala,
            Country::Guyana => &self.country.guyana,
            Country::HongKong => &self.country.hongkong,
            Country::Hungary => &self.country.hungary,
            Country::Iceland => &self.country.iceland,
            Country::India => &self.country.india,
            Country::Indonesia => &self.country.indonesia,
            Country::Ireland => &self.country.ireland,
            Country::Israel => &self.country.israel,
            Country::Italy => &self.country.italy,
            Country::Iran => &self.country.iran,
            Country::Japan => &self.country.japan,
            Country::Kazakhstan => &self.country.kazakhstan,
            Country::Latvia => &self.country.latvia,
            Country::Lithuania => &self.country.lithuania,
            Country::Luxembourg => &self.country.luxembourg,
            Country::Malaysia => &self.country.malaysia,
            Country::Mexico => &self.country.mexico,
            Country::Mongolia => &self.country.mongolia,
            Country::Morocco => &self.country.morocco,
            Country::Netherlands => &self.country.netherlands,
            Country::NewCaledonia => &self.country.newcaledonia,
            Country::NewZealand => &self.country.newzealand,
            Country::Nicaragua => &self.country.nicaragua,
            Country::Norway => &self.country.norway,
            Country::NorthernIreland => &self.country.northernireland,
            Country::Oman => &self.country.oman,
            Country::PapuaNewGuinea => &self.country.papuanewguinea,
            Country::Peru => &self.country.peru,
            Country::Philippines => &self.country.philippines,
            Country::Poland => &self.country.poland,
            Country::Portugal => &self.country.portugal,
            Country::PuertoRico => &self.country.puertorico,
            Country::Russia => &self.country.russia,
            Country::Samoa => &self.country.samoa,
            Country::Scotland => &self.country.scotland,
            Country::Serbia => &self.country.serbia,
            Country::Singapore => &self.country.singapore,
            Country::Slovakia => &self.country.slovakia,
            Country::Slovenia => &self.country.slovenia,
            Country::SouthAfrica => &self.country.southafrica,
            Country::SouthKorea => &self.country.southkorea,
            Country::Spain => &self.country.spain,
            Country::Sweden => &self.country.sweden,
            Country::Switzerland => &self.country.switzerland,
            Country::Tahiti => &self.country.tahiti,
            Country::Taiwan => &self.country.taiwan,
            Country::Turkey => &self.country.turkey,
            Country::UAE => &self.country.uae,
            Country::UK => &self.country.uk,
            Country::Ukraine => &self.country.ukraine,
            Country::Uruguay => &self.country.uruguay,
            Country::USA => &self.country.usa,
            Country::USVirginIslands => &self.country.usvirginislands,
            Country::Uzbekistan => &self.country.uzbekistan,
            Country::Venezuela => &self.country.venezuela,
            Country::Vietnam => &self.country.vietnam,
            Country::Wales => &self.country.wales,
        }
    }
}

/// Selects the localized format of displayed numbers.
#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum NumberFormat {
    /// Arabic numerals with a period as decimal separator, like "1234.5".
    ArabicPeriod,
    /// Arabic numerals with a comma as decimal separator, like "1234,5".
    ArabicComma,
}

impl Language {
    /// Gets the number format for the given language.
    pub fn number_format(self) -> NumberFormat {
        // Taken from the following list:
        // https://en.wikipedia.org/wiki/Decimal_separator
        match self {
            Language::de => NumberFormat::ArabicComma,
            Language::en => NumberFormat::ArabicPeriod,
            Language::eo => NumberFormat::ArabicComma,
            Language::es => NumberFormat::ArabicPeriod, // TODO: Only Central America.
            Language::fi => NumberFormat::ArabicComma,
            Language::fr => NumberFormat::ArabicComma,
            Language::hr => NumberFormat::ArabicComma,
            Language::hu => NumberFormat::ArabicComma,
            Language::it => NumberFormat::ArabicComma,
            Language::ja => NumberFormat::ArabicPeriod,
            Language::pl => NumberFormat::ArabicComma,
            Language::pt => NumberFormat::ArabicComma,
            Language::sl => NumberFormat::ArabicComma,
            Language::sv => NumberFormat::ArabicComma,
            Language::ru => NumberFormat::ArabicComma,
            Language::tr => NumberFormat::ArabicComma,
            Language::uk => NumberFormat::ArabicComma,
            Language::vi => NumberFormat::ArabicComma,
            Language::zh_hant => NumberFormat::ArabicPeriod,
        }
    }
}

/// Type that gets serialized into a localized `WeightAny`.
///
/// This is the final weight type that should be stored in the `Context`
/// and passed to the `Template`.
#[derive(Copy, Clone)]
pub struct LocalizedWeightAny {
    pub format: NumberFormat,
    pub weight: WeightAny,
}

impl Serialize for LocalizedWeightAny {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s: String = match self.format {
            NumberFormat::ArabicPeriod => format!("{}", self.weight),
            NumberFormat::ArabicComma => self.weight.format_comma(),
        };
        serializer.serialize_str(&s)
    }
}

/// Type that gets serialized into a localized `Points`.
#[derive(Copy, Clone)]
pub struct LocalizedPoints {
    pub format: NumberFormat,
    pub points: Points,
}

impl Serialize for LocalizedPoints {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s: String = match self.format {
            NumberFormat::ArabicPeriod => format!("{}", self.points),
            NumberFormat::ArabicComma => self.points.format_comma(),
        };
        serializer.serialize_str(&s)
    }
}

/// Type that gets serialized into a localized `WeightClassAny`.
#[derive(Copy, Clone)]
pub struct LocalizedWeightClassAny {
    pub format: NumberFormat,
    pub class: WeightClassAny,
}

impl Serialize for LocalizedWeightClassAny {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s: String = match self.format {
            NumberFormat::ArabicPeriod => format!("{}", self.class),
            NumberFormat::ArabicComma => self.class.format_comma(),
        };
        serializer.serialize_str(&s)
    }
}

/// Gets the lifter's name localized into the target language.
pub fn get_localized_name(lifter: &opldb::Lifter, language: Language) -> &str {
    match language {
        Language::ru | Language::uk => {
            lifter.cyrillic_name.as_ref().unwrap_or(&lifter.name)
        }
        _ => &lifter.name,
    }
}

/// Localizes the separator between integer and fraction based on `NumberFormat`.
pub trait LocalizeNumber {
    type LocalizedType;

    fn in_format(self, format: NumberFormat) -> Self::LocalizedType;
}

impl LocalizeNumber for WeightAny {
    type LocalizedType = LocalizedWeightAny;

    fn in_format(self, format: NumberFormat) -> LocalizedWeightAny {
        LocalizedWeightAny { format, weight: self }
    }
}

impl LocalizeNumber for WeightClassAny {
    type LocalizedType = LocalizedWeightClassAny;

    fn in_format(self, format: NumberFormat) -> LocalizedWeightClassAny {
        LocalizedWeightClassAny { format, class: self }
    }
}

impl LocalizeNumber for Points {
    type LocalizedType = LocalizedPoints;

    fn in_format(self, format: NumberFormat) -> LocalizedPoints {
        LocalizedPoints { format, points: self }
    }
}
