//! Defines valid entries in the MeetState column.

use strum::ParseError;

use crate::Country;

/// The State column.
#[derive(Debug, PartialEq)]
pub enum State {
    InAustralia(AustraliaState),
    InBrazil(BrazilState),
    InCanada(CanadaState),
    InGermany(GermanyState),
    InIndia(IndiaState),
    InMexico(MexicoState),
    InNetherlands(NetherlandsState),
    InNewZealand(NewZealandState),
    InUSA(USAState),
}

impl State {
    /// Constructs a State for a specific Country.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::{Country, State, USAState};
    /// let state = State::from_str_and_country("NY", Country::USA).unwrap();
    /// assert_eq!(state, State::InUSA(USAState::NY));
    /// ```
    pub fn from_str_and_country(s: &str, country: Country) -> Result<State, ParseError> {
        match country {
            Country::Australia => Ok(State::InAustralia(s.parse::<AustraliaState>()?)),
            Country::Brazil => Ok(State::InBrazil(s.parse::<BrazilState>()?)),
            Country::Canada => Ok(State::InCanada(s.parse::<CanadaState>()?)),
            Country::Germany => Ok(State::InGermany(s.parse::<GermanyState>()?)),
            Country::India => Ok(State::InIndia(s.parse::<IndiaState>()?)),
            Country::Mexico => Ok(State::InMexico(s.parse::<MexicoState>()?)),
            Country::Netherlands => Ok(State::InNetherlands(s.parse::<NetherlandsState>()?)),
            Country::NewZealand => Ok(State::InNewZealand(s.parse::<NewZealandState>()?)),
            Country::USA => Ok(State::InUSA(s.parse::<USAState>()?)),
            _ => Err(ParseError::VariantNotFound),
        }
    }
}

/// A state in Australia.
#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Debug, EnumString, PartialEq)]
pub enum AustraliaState {
    ACT, NSW, NT, QLD, SA, TAS, VIC, WA
}

/// A state in Brazil.
#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Debug, EnumString, PartialEq)]
pub enum BrazilState {
    AC, AL, AP, AM, BA, CE, DF, ES, GO, MA, MT, MS, MG, PA,
    PB, PR, PE, PI, RJ, RN, RS, RO, RR, SC, SP, SE, TO
}

/// A state in Canada.
#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Debug, EnumString, PartialEq)]
pub enum CanadaState {
    AB, BC, MB, NB, NL, NT, NS, NU, ON, PE, QC, SK, YT
}

/// A state in Germany.
#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Debug, EnumString, PartialEq)]
pub enum GermanyState {
    BW, BY, BE, BB, HB, HE, HH, MV, NI, NW, RP, SH, SL, SN, ST, TH
}

/// A state in India.
#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Debug, EnumString, PartialEq)]
pub enum IndiaState {
    AP, AR, AS, BR, CG, GA, GJ, HR, HP, JK, JH, KA,
    KL, MP, MH, MN, ML, MZ, NL, OR, PB, RJ, SK, TN,
    TR, UK, UP, WB, AN, CH, DH, DD, DL, LD, PY
}

/// A state in Mexico.
#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Debug, EnumString, PartialEq)]
pub enum MexicoState {
    AG, BC, BS, CM, CS, CH, CO, CL, DF, DG, GT,
    GR, HG, JA, EM, MI, MO, NA, NL, OA, PU, QT,
    QR, SL, SI, SO, TB, TM, TL, VE, YU, ZA
}

/// A state in the Netherlands
#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Debug, EnumString, PartialEq)]
pub enum NetherlandsState {
    DR, FL, FR, GE, GR, LI, NB, NH, OV, UT, ZE,
    ZH
}

/// A state in New Zealand.
#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Debug, EnumString, PartialEq)]
pub enum NewZealandState {
    NTL, AKL, WKO, BOP, GIS, HKB, TKI, MWT, WGN,
    TAS, NSN, MBH, WTC, CAN, OTA, STL
}

/// A state in the USA.
#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Debug, EnumString, PartialEq)]
pub enum USAState {
    AL, AK, AZ, AR, CA, CO, CT, DE, DC, FL, GA, HI, ID, IL, IN, IA, KS,
    KY, LA, ME, MD, MA, MI, MN, MS, MO, MT, NE, NV, NH, NJ, NM, NY, NC,
    ND, OH, OK, OR, PA, RI, SC, SD, TN, TX, UT, VT, VA, WA, WV, WI, WY,

    /// Guam is an unincorporated territory of the USA.
    Guam,
}
