//! # nord-proxy
//!
//! `nord-proxy` is a Rust crate for retrieving NordVPN proxy endpoints (SOCKS5 and HTTP/HTTPS)
//! and integrating them with `reqwest`.
//!
//! ## Usage
//!
//! ```rust
//! use nord_proxy::{Proxy, Socks5, ProxyTrait};
//!
//! // SOCKS5 proxies
//! let socks5 = Socks5::new().await;
//! let socks5_proxies = socks5.proxies("username", "password");
//!
//! // HTTP / HTTPS proxies
//! let proxy = Proxy::new().await;
//! let http_proxies = proxy.proxies("username", "password");
//!
//! // Example: use with reqwest
//! let proxy_info = &http_proxies[0];
//! let client = reqwest::Client::builder()
//!     .proxy(reqwest::Proxy::all(proxy_info.proxy.clone())?)
//!     .build()?;
//! ```
use serde::{Deserialize, Serialize};

use crate::structure::{Root, Technologies};

mod structure;

async fn get_info(s: &str) -> Vec<Root> {
    let client = reqwest::Client::new();
    let response = client.get(s).send().await.unwrap();
    let json: Vec<Root> = response.json().await.unwrap();
    json
}

pub struct Socks5 {
    data: Vec<Root>,
}

pub struct Proxy {
    data: Vec<(u32, Country, City, Technologies)>,
}

impl Proxy {
    pub async fn new() -> Self {
        let url = "https://api.nordvpn.com/v1/servers?filters[servers_services][identifier]=proxy&limit=0";
        Proxy {
            data: get_info(url)
                .await
                .into_iter()
                .filter(|v| {
                    v.status.to_lowercase() == "online"
                        && v.services.iter().any(|v| v.identifier == "proxy")
                })
                .flat_map(|v| {
                    v.technologies
                        .into_iter()
                        .filter(|v| v.identifier == "proxy_ssl")
                        .map(|vv| {
                            (
                                v.load,
                                v.locations.first().unwrap().country.code.clone(),
                                v.locations.first().unwrap().country.city.name.clone(),
                                vv,
                            )
                        })
                        .collect::<Vec<_>>()
                })
                .collect(),
        }
    }
}

impl ProxyTrait for Proxy {
    fn proxies(&self, username: &str, password: &str) -> Vec<ProxyInfo> {
        self.data
            .iter()
            .map(|v| ProxyInfo {
                load: v.0,
                country: v.1,
                city: v.2,
                proxy: reqwest::Proxy::https(format!(
                    "https://{}:89",
                    v.3.metadata
                        .iter()
                        .find(|v| v.name == "proxy_hostname")
                        .unwrap()
                        .value
                ))
                .unwrap()
                .basic_auth(username, password),
            })
            .collect()
    }
}
impl Socks5 {
    pub async fn new() -> Self {
        let url = "https://api.nordvpn.com/v1/servers?filters[servers_technologies][identifier]=socks&limit=0";
        Socks5 {
            data: get_info(url)
                .await
                .into_iter()
                .filter(|v| {
                    v.status == "online"
                        && v.technologies
                            .iter()
                            .any(|v| v.pivot.status == "online" && v.identifier == "socks")
                })
                .collect(),
        }
    }
}

impl ProxyTrait for Socks5 {
    fn proxies(&self, username: &str, password: &str) -> Vec<ProxyInfo> {
        self.data
            .iter()
            .map(|v| {
                let c = v.locations.first().unwrap();
                ProxyInfo {
                    load: v.load,
                    city: c.country.city.name,
                    country: c.country.code,
                    proxy: reqwest::Proxy::all(format!(
                        "socks5h://{username}:{password}@{}:1080",
                        v.hostname
                    ))
                    .unwrap(),
                }
            })
            .collect()
    }
}

pub trait ProxyTrait {
    fn proxies(&self, username: &str, password: &str) -> Vec<ProxyInfo>;
}

pub struct ProxyInfo {
    pub load: u32,
    pub country: Country,
    pub city: City,
    pub proxy: reqwest::Proxy,
}

#[cfg(test)]
mod tests {
    use crate::{Proxy, ProxyTrait, Socks5};

    #[tokio::test]
    async fn proxy() {
        let proxy = Proxy::new().await.proxies("user", "pass");
        assert!(proxy.len() > 0)
    }

    #[tokio::test]
    async fn socks() {
        let proxy = Socks5::new().await.proxies("user", "pass");
        assert!(proxy.len() > 0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Country {
    #[serde(rename = "EE")]
    EE,
    #[serde(rename = "NP")]
    NP,
    #[serde(rename = "ZA")]
    ZA,
    #[serde(rename = "QA")]
    QA,
    #[serde(rename = "GB")]
    GB,
    #[serde(rename = "JO")]
    JO,
    #[serde(rename = "SV")]
    SV,
    #[serde(rename = "LU")]
    LU,
    #[serde(rename = "TT")]
    TT,
    #[serde(rename = "LB")]
    LB,
    #[serde(rename = "AZ")]
    AZ,
    #[serde(rename = "BH")]
    BH,
    #[serde(rename = "LT")]
    LT,
    #[serde(rename = "PL")]
    PL,
    #[serde(rename = "PG")]
    PG,
    #[serde(rename = "LY")]
    LY,
    #[serde(rename = "HK")]
    HK,
    #[serde(rename = "IN")]
    IN,
    #[serde(rename = "TH")]
    TH,
    #[serde(rename = "CY")]
    CY,
    #[serde(rename = "UA")]
    UA,
    #[serde(rename = "KY")]
    KY,
    #[serde(rename = "FR")]
    FR,
    #[serde(rename = "KM")]
    KM,
    #[serde(rename = "MC")]
    MC,
    #[serde(rename = "NL")]
    NL,
    #[serde(rename = "AD")]
    AD,
    #[serde(rename = "MM")]
    MM,
    #[serde(rename = "PK")]
    PK,
    #[serde(rename = "ME")]
    ME,
    #[serde(rename = "DZ")]
    DZ,
    #[serde(rename = "TW")]
    TW,
    #[serde(rename = "BT")]
    BT,
    #[serde(rename = "UZ")]
    UZ,
    #[serde(rename = "MN")]
    MN,
    #[serde(rename = "LI")]
    LI,
    #[serde(rename = "MX")]
    MX,
    #[serde(rename = "JM")]
    JM,
    #[serde(rename = "GH")]
    GH,
    #[serde(rename = "BG")]
    BG,
    #[serde(rename = "RS")]
    RS,
    #[serde(rename = "BO")]
    BO,
    #[serde(rename = "BE")]
    BE,
    #[serde(rename = "GR")]
    GR,
    #[serde(rename = "LV")]
    LV,
    #[serde(rename = "SN")]
    SN,
    #[serde(rename = "PA")]
    PA,
    #[serde(rename = "TJ")]
    TJ,
    #[serde(rename = "BR")]
    BR,
    #[serde(rename = "KH")]
    KH,
    #[serde(rename = "AF")]
    AF,
    #[serde(rename = "KR")]
    KR,
    #[serde(rename = "BZ")]
    BZ,
    #[serde(rename = "ES")]
    ES,
    #[serde(rename = "HU")]
    HU,
    #[serde(rename = "AR")]
    AR,
    #[serde(rename = "LA")]
    LA,
    #[serde(rename = "SK")]
    SK,
    #[serde(rename = "MK")]
    MK,
    #[serde(rename = "TR")]
    TR,
    #[serde(rename = "CO")]
    CO,
    #[serde(rename = "SE")]
    SE,
    #[serde(rename = "CA")]
    CA,
    #[serde(rename = "AO")]
    AO,
    #[serde(rename = "TN")]
    TN,
    #[serde(rename = "KW")]
    KW,
    #[serde(rename = "KE")]
    KE,
    #[serde(rename = "DE")]
    DE,
    #[serde(rename = "CR")]
    CR,
    #[serde(rename = "NO")]
    NO,
    #[serde(rename = "DO")]
    DO,
    #[serde(rename = "HR")]
    HR,
    #[serde(rename = "MR")]
    MR,
    #[serde(rename = "GT")]
    GT,
    #[serde(rename = "AM")]
    AM,
    #[serde(rename = "MA")]
    MA,
    #[serde(rename = "IL")]
    IL,
    #[serde(rename = "BN")]
    BN,
    #[serde(rename = "BA")]
    BA,
    #[serde(rename = "BS")]
    BS,
    #[serde(rename = "EC")]
    EC,
    #[serde(rename = "NZ")]
    NZ,
    #[serde(rename = "IT")]
    IT,
    #[serde(rename = "MD")]
    MD,
    #[serde(rename = "CH")]
    CH,
    #[serde(rename = "SO")]
    SO,
    #[serde(rename = "EG")]
    EG,
    #[serde(rename = "JP")]
    JP,
    #[serde(rename = "VN")]
    VN,
    #[serde(rename = "AE")]
    AE,
    #[serde(rename = "JE")]
    JE,
    #[serde(rename = "RO")]
    RO,
    #[serde(rename = "KZ")]
    KZ,
    #[serde(rename = "DK")]
    DK,
    #[serde(rename = "US")]
    US,
    #[serde(rename = "IQ")]
    IQ,
    #[serde(rename = "LK")]
    LK,
    #[serde(rename = "AU")]
    AU,
    #[serde(rename = "PH")]
    PH,
    #[serde(rename = "MZ")]
    MZ,
    #[serde(rename = "PY")]
    PY,
    #[serde(rename = "GE")]
    GE,
    #[serde(rename = "PR")]
    PR,
    #[serde(rename = "NG")]
    NG,
    #[serde(rename = "ID")]
    ID,
    #[serde(rename = "VE")]
    VE,
    #[serde(rename = "MU")]
    MU,
    #[serde(rename = "PT")]
    PT,
    #[serde(rename = "UY")]
    UY,
    #[serde(rename = "IM")]
    IM,
    #[serde(rename = "CZ")]
    CZ,
    #[serde(rename = "CL")]
    CL,
    #[serde(rename = "MT")]
    MT,
    #[serde(rename = "MY")]
    MY,
    #[serde(rename = "RW")]
    RW,
    #[serde(rename = "AT")]
    AT,
    #[serde(rename = "FI")]
    FI,
    #[serde(rename = "ET")]
    ET,
    #[serde(rename = "AL")]
    AL,
    #[serde(rename = "BD")]
    BD,
    #[serde(rename = "IE")]
    IE,
    #[serde(rename = "GU")]
    GU,
    #[serde(rename = "SI")]
    SI,
    #[serde(rename = "SG")]
    SG,
    #[serde(rename = "HN")]
    HN,
    #[serde(rename = "BM")]
    BM,
    #[serde(rename = "PE")]
    PE,
    #[serde(rename = "GL")]
    GL,
    #[serde(rename = "IS")]
    IS,
    #[serde(rename = "SR")]
    SR,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum City {
    #[serde(rename = "Sofia")]
    Sofia,
    #[serde(rename = "Jakarta")]
    Jakarta,
    #[serde(rename = "Guatemala City")]
    GuatemalaCity,
    #[serde(rename = "Kuala Lumpur")]
    KualaLumpur,
    #[serde(rename = "Wilmington")]
    Wilmington,
    #[serde(rename = "Dublin")]
    Dublin,
    #[serde(rename = "Warsaw")]
    Warsaw,
    #[serde(rename = "Dushanbe")]
    Dushanbe,
    #[serde(rename = "Chicago")]
    Chicago,
    #[serde(rename = "Brisbane")]
    Brisbane,
    #[serde(rename = "Thimphu")]
    Thimphu,
    #[serde(rename = "Luxembourg")]
    Luxembourg,
    #[serde(rename = "Houston")]
    Houston,
    #[serde(rename = "Buenos Aires")]
    BuenosAires,
    #[serde(rename = "Baltimore")]
    Baltimore,
    #[serde(rename = "San Salvador")]
    SanSalvador,
    #[serde(rename = "Istanbul")]
    Istanbul,
    #[serde(rename = "Huntington")]
    Huntington,
    #[serde(rename = "Nuuk")]
    Nuuk,
    #[serde(rename = "Quito")]
    Quito,
    #[serde(rename = "Vancouver")]
    Vancouver,
    #[serde(rename = "Dakar")]
    Dakar,
    #[serde(rename = "Montevideo")]
    Montevideo,
    #[serde(rename = "Sarajevo")]
    Sarajevo,
    #[serde(rename = "Algiers")]
    Algiers,
    #[serde(rename = "Mexico")]
    Mexico,
    #[serde(rename = "Doha")]
    Doha,
    #[serde(rename = "Los Angeles")]
    LosAngeles,
    #[serde(rename = "Riga")]
    Riga,
    #[serde(rename = "Kigali")]
    Kigali,
    #[serde(rename = "Oslo")]
    Oslo,
    #[serde(rename = "Taipei")]
    Taipei,
    #[serde(rename = "Port Louis")]
    PortLouis,
    #[serde(rename = "Auckland")]
    Auckland,
    #[serde(rename = "Port of Spain")]
    PortOfSpain,
    #[serde(rename = "Panama City")]
    PanamaCity,
    #[serde(rename = "La Paz")]
    LaPaz,
    #[serde(rename = "George Town")]
    GeorgeTown,
    #[serde(rename = "London")]
    London,
    #[serde(rename = "Tokyo")]
    Tokyo,
    #[serde(rename = "Phoenix")]
    Phoenix,
    #[serde(rename = "San Jose")]
    SanJose,
    #[serde(rename = "Zagreb")]
    Zagreb,
    #[serde(rename = "Buffalo")]
    Buffalo,
    #[serde(rename = "Marseille")]
    Marseille,
    #[serde(rename = "Santiago")]
    Santiago,
    #[serde(rename = "Yerevan")]
    Yerevan,
    #[serde(rename = "Kingston")]
    Kingston,
    #[serde(rename = "Ashburn")]
    Ashburn,
    #[serde(rename = "Lima")]
    Lima,
    #[serde(rename = "Milan")]
    Milan,
    #[serde(rename = "Tripoli")]
    Tripoli,
    #[serde(rename = "Dhaka")]
    Dhaka,
    #[serde(rename = "Stockholm")]
    Stockholm,
    #[serde(rename = "Addis Ababa")]
    AddisAbaba,
    #[serde(rename = "Omaha")]
    Omaha,
    #[serde(rename = "Toronto")]
    Toronto,
    #[serde(rename = "Berlin")]
    Berlin,
    #[serde(rename = "Burlington")]
    Burlington,
    #[serde(rename = "Charlotte")]
    Charlotte,
    #[serde(rename = "Hagatna")]
    Hagatna,
    #[serde(rename = "Belgrade")]
    Belgrade,
    #[serde(rename = "Paris")]
    Paris,
    #[serde(rename = "Athens")]
    Athens,
    #[serde(rename = "Luanda")]
    Luanda,
    #[serde(rename = "Providence")]
    Providence,
    #[serde(rename = "Lewiston")]
    Lewiston,
    #[serde(rename = "Tegucigalpa")]
    Tegucigalpa,
    #[serde(rename = "Denver")]
    Denver,
    #[serde(rename = "Sao Paulo")]
    SaoPaulo,
    #[serde(rename = "Osaka")]
    Osaka,
    #[serde(rename = "Maputo")]
    Maputo,
    #[serde(rename = "Kyiv")]
    Kyiv,
    #[serde(rename = "Ho Chi Minh City")]
    HoChiMinhCity,
    #[serde(rename = "Phnom Penh")]
    PhnomPenh,
    #[serde(rename = "Karachi")]
    Karachi,
    #[serde(rename = "Accra")]
    Accra,
    #[serde(rename = "Glasgow")]
    Glasgow,
    #[serde(rename = "Dubai")]
    Dubai,
    #[serde(rename = "Chisinau")]
    Chisinau,
    #[serde(rename = "Baku")]
    Baku,
    #[serde(rename = "Perth")]
    Perth,
    #[serde(rename = "Palermo")]
    Palermo,
    #[serde(rename = "McAllen")]
    McAllen,
    #[serde(rename = "Madrid")]
    Madrid,
    #[serde(rename = "Douglas")]
    Douglas,
    #[serde(rename = "Pittsburgh")]
    Pittsburgh,
    #[serde(rename = "Edinburgh")]
    Edinburgh,
    #[serde(rename = "Lagos")]
    Lagos,
    #[serde(rename = "Ljubljana")]
    Ljubljana,
    #[serde(rename = "Lisbon")]
    Lisbon,
    #[serde(rename = "Caracas")]
    Caracas,
    #[serde(rename = "Prague")]
    Prague,
    #[serde(rename = "Beirut")]
    Beirut,
    #[serde(rename = "Vientiane")]
    Vientiane,
    #[serde(rename = "Copenhagen")]
    Copenhagen,
    #[serde(rename = "Cairo")]
    Cairo,
    #[serde(rename = "Sydney")]
    Sydney,
    #[serde(rename = "Nouakchott")]
    Nouakchott,
    #[serde(rename = "Rome")]
    Rome,
    #[serde(rename = "Boston")]
    Boston,
    #[serde(rename = "Bangkok")]
    Bangkok,
    #[serde(rename = "New Haven")]
    NewHaven,
    #[serde(rename = "Astana")]
    Astana,
    #[serde(rename = "Valletta")]
    Valletta,
    #[serde(rename = "Mumbai")]
    Mumbai,
    #[serde(rename = "Bandar Seri Begawan")]
    BandarSeriBegawan,
    #[serde(rename = "Skopje")]
    Skopje,
    #[serde(rename = "Kathmandu")]
    Kathmandu,
    #[serde(rename = "Tbilisi")]
    Tbilisi,
    #[serde(rename = "Zurich")]
    Zurich,
    #[serde(rename = "Hong Kong")]
    HongKong,
    #[serde(rename = "Belmopan")]
    Belmopan,
    #[serde(rename = "Mogadishu")]
    Mogadishu,
    #[serde(rename = "Barcelona")]
    Barcelona,
    #[serde(rename = "Moroni")]
    Moroni,
    #[serde(rename = "New York")]
    NewYork,
    #[serde(rename = "Nashua")]
    Nashua,
    #[serde(rename = "Reykjavik")]
    Reykjavik,
    #[serde(rename = "San Juan")]
    SanJuan,
    #[serde(rename = "Johannesburg")]
    Johannesburg,
    #[serde(rename = "Amman")]
    Amman,
    #[serde(rename = "Nicosia")]
    Nicosia,
    #[serde(rename = "Nairobi")]
    Nairobi,
    #[serde(rename = "Colombo")]
    Colombo,
    #[serde(rename = "Tashkent")]
    Tashkent,
    #[serde(rename = "Miami")]
    Miami,
    #[serde(rename = "Rabat")]
    Rabat,
    #[serde(rename = "Hamilton")]
    Hamilton,
    #[serde(rename = "Saint Louis")]
    SaintLouis,
    #[serde(rename = "Atlanta")]
    Atlanta,
    #[serde(rename = "Montreal")]
    Montreal,
    #[serde(rename = "Tel Aviv")]
    TelAviv,
    #[serde(rename = "Ulaanbaatar")]
    Ulaanbaatar,
    #[serde(rename = "Port Moresby")]
    PortMoresby,
    #[serde(rename = "Seoul")]
    Seoul,
    #[serde(rename = "Nassau")]
    Nassau,
    #[serde(rename = "Salt Lake City")]
    SaltLakeCity,
    #[serde(rename = "Hamburg")]
    Hamburg,
    #[serde(rename = "San Francisco")]
    SanFrancisco,
    #[serde(rename = "Bucharest")]
    Bucharest,
    #[serde(rename = "Andorra la Vella")]
    AndorraLaVella,
    #[serde(rename = "Melbourne")]
    Melbourne,
    #[serde(rename = "Kuwait City")]
    KuwaitCity,
    #[serde(rename = "Helsinki")]
    Helsinki,
    #[serde(rename = "Nashville")]
    Nashville,
    #[serde(rename = "Saint Helier")]
    SaintHelier,
    #[serde(rename = "Brussels")]
    Brussels,
    #[serde(rename = "Dallas")]
    Dallas,
    #[serde(rename = "Budapest")]
    Budapest,
    #[serde(rename = "Monte Carlo")]
    MonteCarlo,
    #[serde(rename = "Bogota")]
    Bogota,
    #[serde(rename = "Vilnius")]
    Vilnius,
    #[serde(rename = "Tunis")]
    Tunis,
    #[serde(rename = "Amsterdam")]
    Amsterdam,
    #[serde(rename = "Tirana")]
    Tirana,
    #[serde(rename = "Podgorica")]
    Podgorica,
    #[serde(rename = "Fujairah")]
    Fujairah,
    #[serde(rename = "Vienna")]
    Vienna,
    #[serde(rename = "Bratislava")]
    Bratislava,
    #[serde(rename = "Seattle")]
    Seattle,
    #[serde(rename = "Baghdad")]
    Baghdad,
    #[serde(rename = "Frankfurt")]
    Frankfurt,
    #[serde(rename = "Manchester")]
    Manchester,
    #[serde(rename = "Trenton")]
    Trenton,
    #[serde(rename = "Asuncion")]
    Asuncion,
    #[serde(rename = "Singapore")]
    Singapore,
    #[serde(rename = "Vaduz")]
    Vaduz,
    #[serde(rename = "Adelaide")]
    Adelaide,
    #[serde(rename = "Hanoi")]
    Hanoi,
    #[serde(rename = "Tallinn")]
    Tallinn,
    #[serde(rename = "Santo Domingo")]
    SantoDomingo,
    #[serde(rename = "Manama")]
    Manama,
    #[serde(rename = "Kansas City")]
    KansasCity,
    #[serde(rename = "Kabul")]
    Kabul,
    #[serde(rename = "Naypyidaw")]
    Naypyidaw,
    #[serde(rename = "Manila")]
    Manila,
    #[serde(rename = "Strasbourg")]
    Strasbourg,
    Paramaribo,
    Bordeaux,
    Charleston,
}
