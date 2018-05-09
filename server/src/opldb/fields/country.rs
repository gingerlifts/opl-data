//! Defines the `Country` field for the `meets` table.

#[derive(Copy, Clone, Deserialize, Serialize, PartialEq)]
pub enum Country {
    Algeria,
    Argentina,
    Aruba,
    Australia,
    Austria,
    Belarus,
    Belgium,
    Brazil,
    Britain,
    #[serde(rename = "British Virgin Islands")]
    BritishVirginIslands,
    Bulgaria,
    Canada,
    #[serde(rename = "Cayman Islands")]
    CaymanIslands,
    Colombia,
    #[serde(rename = "Costa Rica")]
    CostaRica,
    #[serde(rename = "Côte d’Ivoire")]
    CoteDIvoire,
    // FIXME: Standardize on one of these.
    Czechia,
    Denmark,
    Ecuador,
    Egypt,
    #[serde(rename = "El Salvador")]
    ElSalvador,
    England,
    Estonia,
    Fiji,
    Finland,
    France,
    Germany,
    Greece,
    Guatemala,
    Guyana,
    #[serde(rename = "Hong Kong")]
    HongKong,
    Hungary,
    Iceland,
    India,
    Indonesia,
    Ireland,
    Israel,
    Italy,
    Iran,
    Japan,
    Kazakhstan,
    Latvia,
    Lithuania,
    Luxembourg,
    Malaysia,
    Mexico,
    Mongolia,
    Morocco,
    Netherlands,
    #[serde(rename = "New Caledonia")]
    NewCaledonia,
    #[serde(rename = "New Zealand")]
    NewZealand,
    Nicaragua,
    Norway,
    #[serde(rename = "N.Ireland")]
    NorthernIreland,
    Oman,
    #[serde(rename = "Papua New Guinea")]
    PapuaNewGuinea,
    Peru,
    Philippines,
    Poland,
    #[serde(rename = "Puerto Rico")]
    PuertoRico,
    Russia,
    Samoa,
    Scotland,
    Serbia,
    Singapore,
    Slovakia,
    Slovenia,
    #[serde(rename = "South Africa")]
    SouthAfrica,
    #[serde(rename = "South Korea")]
    SouthKorea,
    Spain,
    Sweden,
    Switzerland,
    Tahiti,
    Taiwan,
    Turkey,
    UK,
    Ukraine,
    Uruguay,
    USA,
    #[serde(rename = "US Virgin Islands")]
    USVirginIslands,
    Uzbekistan,
    Venezuela,
    Wales,
}
