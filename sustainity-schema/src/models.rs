#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

use serde::{Deserialize, Serialize};

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a TryFrom or FromStr implementation."]
    pub struct ConversionError(std::borrow::Cow<'static, str>);
    impl std::error::Error for ConversionError {}
    impl std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "AboutCataloger"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"name\","]
#[doc = "    \"variant\","]
#[doc = "    \"website\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"variant\": {"]
#[doc = "      \"$ref\": \"#/$defs/catalogVariant\""]
#[doc = "    },"]
#[doc = "    \"website\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AboutCataloger {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub id: String,
    pub name: String,
    pub variant: CatalogVariant,
    pub website: String,
}
impl From<&AboutCataloger> for AboutCataloger {
    fn from(value: &AboutCataloger) -> Self {
        value.clone()
    }
}
impl AboutCataloger {
    pub fn builder() -> builder::AboutCataloger {
        Default::default()
    }
}
#[doc = "AboutCertification"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AboutCertification(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for AboutCertification {
    type Target = serde_json::Map<String, serde_json::Value>;
    fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
        &self.0
    }
}
impl From<AboutCertification> for serde_json::Map<String, serde_json::Value> {
    fn from(value: AboutCertification) -> Self {
        value.0
    }
}
impl From<&AboutCertification> for AboutCertification {
    fn from(value: &AboutCertification) -> Self {
        value.clone()
    }
}
impl From<serde_json::Map<String, serde_json::Value>> for AboutCertification {
    fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "AboutProducer"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"ids\","]
#[doc = "    \"name\","]
#[doc = "    \"website\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ids\": {"]
#[doc = "      \"$ref\": \"#/$defs/producerIds\""]
#[doc = "    },"]
#[doc = "    \"images\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"origins\": {"]
#[doc = "      \"$ref\": \"#/$defs/producerOrigins\""]
#[doc = "    },"]
#[doc = "    \"websites\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AboutProducer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub id: String,
    pub ids: ProducerIds,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<String>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origins: Option<ProducerOrigins>,
    pub website: serde_json::Value,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub websites: Vec<String>,
}
impl From<&AboutProducer> for AboutProducer {
    fn from(value: &AboutProducer) -> Self {
        value.clone()
    }
}
impl AboutProducer {
    pub fn builder() -> builder::AboutProducer {
        Default::default()
    }
}
#[doc = "AboutReview"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/aboutScoreReview\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/aboutCertification\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum AboutReview {
    ScoreReview(AboutScoreReview),
    Certification(AboutCertification),
}
impl From<&AboutReview> for AboutReview {
    fn from(value: &AboutReview) -> Self {
        value.clone()
    }
}
impl From<AboutScoreReview> for AboutReview {
    fn from(value: AboutScoreReview) -> Self {
        Self::ScoreReview(value)
    }
}
impl From<AboutCertification> for AboutReview {
    fn from(value: AboutCertification) -> Self {
        Self::Certification(value)
    }
}
#[doc = "AboutReviewer"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"description\","]
#[doc = "    \"id\","]
#[doc = "    \"name\","]
#[doc = "    \"website\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"reviews\": {"]
#[doc = "      \"$ref\": \"#/$defs/aboutReview\""]
#[doc = "    },"]
#[doc = "    \"website\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AboutReviewer {
    pub description: String,
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviews: Option<AboutReview>,
    pub website: String,
}
impl From<&AboutReviewer> for AboutReviewer {
    fn from(value: &AboutReviewer) -> Self {
        value.clone()
    }
}
impl AboutReviewer {
    pub fn builder() -> builder::AboutReviewer {
        Default::default()
    }
}
#[doc = "AboutScoreReview"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"div\","]
#[doc = "    \"max\","]
#[doc = "    \"min\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"div\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"max\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"min\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AboutScoreReview {
    pub div: i64,
    pub max: i64,
    pub min: i64,
}
impl From<&AboutScoreReview> for AboutScoreReview {
    fn from(value: &AboutScoreReview) -> Self {
        value.clone()
    }
}
impl AboutScoreReview {
    pub fn builder() -> builder::AboutScoreReview {
        Default::default()
    }
}
#[doc = "CatalogProducer"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"ids\","]
#[doc = "    \"names\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ids\": {"]
#[doc = "      \"$ref\": \"#/$defs/producerIds\""]
#[doc = "    },"]
#[doc = "    \"images\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"names\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"origins\": {"]
#[doc = "      \"$ref\": \"#/$defs/producerOrigins\""]
#[doc = "    },"]
#[doc = "    \"websites\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogProducer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub id: String,
    pub ids: ProducerIds,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<String>,
    pub names: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origins: Option<ProducerOrigins>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub websites: Vec<String>,
}
impl From<&CatalogProducer> for CatalogProducer {
    fn from(value: &CatalogProducer) -> Self {
        value.clone()
    }
}
impl CatalogProducer {
    pub fn builder() -> builder::CatalogProducer {
        Default::default()
    }
}
#[doc = "CatalogProduct"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"ids\","]
#[doc = "    \"names\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"availability\": {"]
#[doc = "      \"$ref\": \"#/$defs/productAvailability\""]
#[doc = "    },"]
#[doc = "    \"categorisation\": {"]
#[doc = "      \"$ref\": \"#/$defs/productCategorisation\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ids\": {"]
#[doc = "      \"$ref\": \"#/$defs/productIds\""]
#[doc = "    },"]
#[doc = "    \"images\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"names\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"origins\": {"]
#[doc = "      \"$ref\": \"#/$defs/productOrigins\""]
#[doc = "    },"]
#[doc = "    \"related\": {"]
#[doc = "      \"$ref\": \"#/$defs/relatedProducts\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogProduct {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub availability: Option<ProductAvailability>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub categorisation: Option<ProductCategorisation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub id: String,
    pub ids: ProductIds,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<String>,
    pub names: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origins: Option<ProductOrigins>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<RelatedProducts>,
}
impl From<&CatalogProduct> for CatalogProduct {
    fn from(value: &CatalogProduct) -> Self {
        value.clone()
    }
}
impl CatalogProduct {
    pub fn builder() -> builder::CatalogProduct {
        Default::default()
    }
}
#[doc = "CatalogVariant"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"store\","]
#[doc = "    \"priceComparator\","]
#[doc = "    \"database\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum CatalogVariant {
    #[serde(rename = "store")]
    Store,
    #[serde(rename = "priceComparator")]
    PriceComparator,
    #[serde(rename = "database")]
    Database,
}
impl From<&CatalogVariant> for CatalogVariant {
    fn from(value: &CatalogVariant) -> Self {
        value.clone()
    }
}
impl ToString for CatalogVariant {
    fn to_string(&self) -> String {
        match *self {
            Self::Store => "store".to_string(),
            Self::PriceComparator => "priceComparator".to_string(),
            Self::Database => "database".to_string(),
        }
    }
}
impl std::str::FromStr for CatalogVariant {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "store" => Ok(Self::Store),
            "priceComparator" => Ok(Self::PriceComparator),
            "database" => Ok(Self::Database),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for CatalogVariant {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CatalogVariant {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CatalogVariant {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "CatalogerRoot"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"cataloger\","]
#[doc = "    \"meta\","]
#[doc = "    \"producers\","]
#[doc = "    \"products\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"cataloger\": {"]
#[doc = "      \"$ref\": \"#/$defs/aboutCataloger\""]
#[doc = "    },"]
#[doc = "    \"meta\": {"]
#[doc = "      \"$ref\": \"#/$defs/meta\""]
#[doc = "    },"]
#[doc = "    \"producers\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/catalogProducer\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"products\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/catalogProduct\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogerRoot {
    pub cataloger: AboutCataloger,
    pub meta: Meta,
    pub producers: Vec<CatalogProducer>,
    pub products: Vec<CatalogProduct>,
}
impl From<&CatalogerRoot> for CatalogerRoot {
    fn from(value: &CatalogerRoot) -> Self {
        value.clone()
    }
}
impl CatalogerRoot {
    pub fn builder() -> builder::CatalogerRoot {
        Default::default()
    }
}
#[doc = "Certification"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"is_certified\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Certification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_certified: Option<bool>,
}
impl From<&Certification> for Certification {
    fn from(value: &Certification) -> Self {
        value.clone()
    }
}
impl Certification {
    pub fn builder() -> builder::Certification {
        Default::default()
    }
}
#[doc = "EntryVariant"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"about\","]
#[doc = "    \"product\","]
#[doc = "    \"producer\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum EntryVariant {
    #[serde(rename = "about")]
    About,
    #[serde(rename = "product")]
    Product,
    #[serde(rename = "producer")]
    Producer,
}
impl From<&EntryVariant> for EntryVariant {
    fn from(value: &EntryVariant) -> Self {
        value.clone()
    }
}
impl ToString for EntryVariant {
    fn to_string(&self) -> String {
        match *self {
            Self::About => "about".to_string(),
            Self::Product => "product".to_string(),
            Self::Producer => "producer".to_string(),
        }
    }
}
impl std::str::FromStr for EntryVariant {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "about" => Ok(Self::About),
            "product" => Ok(Self::Product),
            "producer" => Ok(Self::Producer),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for EntryVariant {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for EntryVariant {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for EntryVariant {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Meta"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"title\","]
#[doc = "    \"variant\","]
#[doc = "    \"version\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"authors\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"creation_timestamp\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"valid_from\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"valid_to\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"variant\": {"]
#[doc = "      \"$ref\": \"#/$defs/providerVariant\""]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Meta {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authors: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<chrono::DateTime<chrono::offset::Utc>>,
    pub variant: ProviderVariant,
    pub version: String,
}
impl From<&Meta> for Meta {
    fn from(value: &Meta) -> Self {
        value.clone()
    }
}
impl Meta {
    pub fn builder() -> builder::Meta {
        Default::default()
    }
}
#[doc = "ProducerIds"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"domains\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"uniqueItems\": true"]
#[doc = "    },"]
#[doc = "    \"vat\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"uniqueItems\": true"]
#[doc = "    },"]
#[doc = "    \"wiki\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"uniqueItems\": true"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProducerIds {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vat: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wiki: Option<Vec<String>>,
}
impl From<&ProducerIds> for ProducerIds {
    fn from(value: &ProducerIds) -> Self {
        value.clone()
    }
}
impl ProducerIds {
    pub fn builder() -> builder::ProducerIds {
        Default::default()
    }
}
#[doc = "ProducerOrigins"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"regions\": {"]
#[doc = "      \"$ref\": \"#/$defs/regionList\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProducerOrigins {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regions: Option<RegionList>,
}
impl From<&ProducerOrigins> for ProducerOrigins {
    fn from(value: &ProducerOrigins) -> Self {
        value.clone()
    }
}
impl ProducerOrigins {
    pub fn builder() -> builder::ProducerOrigins {
        Default::default()
    }
}
#[doc = "ProducerProduct"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"categorisation\","]
#[doc = "    \"description\","]
#[doc = "    \"id\","]
#[doc = "    \"ids\","]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"availability\": {"]
#[doc = "      \"$ref\": \"#/$defs/productAvailability\""]
#[doc = "    },"]
#[doc = "    \"categorisation\": {"]
#[doc = "      \"$ref\": \"#/$defs/productCategorisation\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ids\": {"]
#[doc = "      \"$ref\": \"#/$defs/productIds\""]
#[doc = "    },"]
#[doc = "    \"images\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"names\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"origins\": {"]
#[doc = "      \"$ref\": \"#/$defs/productOrigins\""]
#[doc = "    },"]
#[doc = "    \"related\": {"]
#[doc = "      \"$ref\": \"#/$defs/relatedProducts\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProducerProduct {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub availability: Option<ProductAvailability>,
    pub categorisation: ProductCategorisation,
    pub description: String,
    pub id: String,
    pub ids: ProductIds,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<String>,
    pub name: serde_json::Value,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub names: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origins: Option<ProductOrigins>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<RelatedProducts>,
}
impl From<&ProducerProduct> for ProducerProduct {
    fn from(value: &ProducerProduct) -> Self {
        value.clone()
    }
}
impl ProducerProduct {
    pub fn builder() -> builder::ProducerProduct {
        Default::default()
    }
}
#[doc = "ProducerReviewer"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"names\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProducerReviewer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub id: String,
    pub name: serde_json::Value,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub names: Vec<String>,
}
impl From<&ProducerReviewer> for ProducerReviewer {
    fn from(value: &ProducerReviewer) -> Self {
        value.clone()
    }
}
impl ProducerReviewer {
    pub fn builder() -> builder::ProducerReviewer {
        Default::default()
    }
}
#[doc = "ProducerRoot"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"meta\","]
#[doc = "    \"producer\","]
#[doc = "    \"producers\","]
#[doc = "    \"reviewers\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"meta\": {"]
#[doc = "      \"$ref\": \"#/$defs/meta\""]
#[doc = "    },"]
#[doc = "    \"producer\": {"]
#[doc = "      \"$ref\": \"#/$defs/aboutProducer\""]
#[doc = "    },"]
#[doc = "    \"products\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/producerProduct\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"reviewers\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/producerReviewer\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProducerRoot {
    pub meta: Meta,
    pub producer: AboutProducer,
    pub producers: serde_json::Value,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub products: Vec<ProducerProduct>,
    pub reviewers: Vec<ProducerReviewer>,
}
impl From<&ProducerRoot> for ProducerRoot {
    fn from(value: &ProducerRoot) -> Self {
        value.clone()
    }
}
impl ProducerRoot {
    pub fn builder() -> builder::ProducerRoot {
        Default::default()
    }
}
#[doc = "ProductAvailability"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"regions\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"regions\": {"]
#[doc = "      \"$ref\": \"#/$defs/regions\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProductAvailability {
    pub regions: Regions,
}
impl From<&ProductAvailability> for ProductAvailability {
    fn from(value: &ProductAvailability) -> Self {
        value.clone()
    }
}
impl ProductAvailability {
    pub fn builder() -> builder::ProductAvailability {
        Default::default()
    }
}
#[doc = "ProductCategorisation"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"categories\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"categories\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/productCategory\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProductCategorisation {
    pub categories: Vec<ProductCategory>,
}
impl From<&ProductCategorisation> for ProductCategorisation {
    fn from(value: &ProductCategorisation) -> Self {
        value.clone()
    }
}
impl ProductCategorisation {
    pub fn builder() -> builder::ProductCategorisation {
        Default::default()
    }
}
#[doc = "ProductCategory"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"type\": \"string\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProductCategory(pub Vec<String>);
impl std::ops::Deref for ProductCategory {
    type Target = Vec<String>;
    fn deref(&self) -> &Vec<String> {
        &self.0
    }
}
impl From<ProductCategory> for Vec<String> {
    fn from(value: ProductCategory) -> Self {
        value.0
    }
}
impl From<&ProductCategory> for ProductCategory {
    fn from(value: &ProductCategory) -> Self {
        value.clone()
    }
}
impl From<Vec<String>> for ProductCategory {
    fn from(value: Vec<String>) -> Self {
        Self(value)
    }
}
#[doc = "ProductIds"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"ean\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"uniqueItems\": true"]
#[doc = "    },"]
#[doc = "    \"gtin\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"uniqueItems\": true"]
#[doc = "    },"]
#[doc = "    \"wiki\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"uniqueItems\": true"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProductIds {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ean: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gtin: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wiki: Option<Vec<String>>,
}
impl From<&ProductIds> for ProductIds {
    fn from(value: &ProductIds) -> Self {
        value.clone()
    }
}
impl ProductIds {
    pub fn builder() -> builder::ProductIds {
        Default::default()
    }
}
#[doc = "ProductOrigins"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"producer_ids\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"regions\": {"]
#[doc = "      \"$ref\": \"#/$defs/regionList\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProductOrigins {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub producer_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regions: Option<RegionList>,
}
impl From<&ProductOrigins> for ProductOrigins {
    fn from(value: &ProductOrigins) -> Self {
        value.clone()
    }
}
impl ProductOrigins {
    pub fn builder() -> builder::ProductOrigins {
        Default::default()
    }
}
#[doc = "ProviderVariant"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"cataloger\","]
#[doc = "    \"producer\","]
#[doc = "    \"reviewer\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ProviderVariant {
    #[serde(rename = "cataloger")]
    Cataloger,
    #[serde(rename = "producer")]
    Producer,
    #[serde(rename = "reviewer")]
    Reviewer,
}
impl From<&ProviderVariant> for ProviderVariant {
    fn from(value: &ProviderVariant) -> Self {
        value.clone()
    }
}
impl ToString for ProviderVariant {
    fn to_string(&self) -> String {
        match *self {
            Self::Cataloger => "cataloger".to_string(),
            Self::Producer => "producer".to_string(),
            Self::Reviewer => "reviewer".to_string(),
        }
    }
}
impl std::str::FromStr for ProviderVariant {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "cataloger" => Ok(Self::Cataloger),
            "producer" => Ok(Self::Producer),
            "reviewer" => Ok(Self::Reviewer),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for ProviderVariant {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ProviderVariant {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ProviderVariant {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "RegionList"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"type\": \"string\""]
#[doc = "  },"]
#[doc = "  \"uniqueItems\": true"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RegionList(pub Vec<String>);
impl std::ops::Deref for RegionList {
    type Target = Vec<String>;
    fn deref(&self) -> &Vec<String> {
        &self.0
    }
}
impl From<RegionList> for Vec<String> {
    fn from(value: RegionList) -> Self {
        value.0
    }
}
impl From<&RegionList> for RegionList {
    fn from(value: &RegionList) -> Self {
        value.clone()
    }
}
impl From<Vec<String>> for RegionList {
    fn from(value: Vec<String>) -> Self {
        Self(value)
    }
}
#[doc = "RegionVariant"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"all\","]
#[doc = "    \"unknown\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum RegionVariant {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "unknown")]
    Unknown,
}
impl From<&RegionVariant> for RegionVariant {
    fn from(value: &RegionVariant) -> Self {
        value.clone()
    }
}
impl ToString for RegionVariant {
    fn to_string(&self) -> String {
        match *self {
            Self::All => "all".to_string(),
            Self::Unknown => "unknown".to_string(),
        }
    }
}
impl std::str::FromStr for RegionVariant {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "all" => Ok(Self::All),
            "unknown" => Ok(Self::Unknown),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for RegionVariant {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for RegionVariant {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for RegionVariant {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Regions"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/regionVariant\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/regionList\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Regions {
    Variant(RegionVariant),
    List(RegionList),
}
impl From<&Regions> for Regions {
    fn from(value: &Regions) -> Self {
        value.clone()
    }
}
impl From<RegionVariant> for Regions {
    fn from(value: RegionVariant) -> Self {
        Self::Variant(value)
    }
}
impl From<RegionList> for Regions {
    fn from(value: RegionList) -> Self {
        Self::List(value)
    }
}
#[doc = "RelatedProducts"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"followed_by\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"uniqueItems\": true"]
#[doc = "    },"]
#[doc = "    \"preceded_by\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"uniqueItems\": true"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RelatedProducts {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub followed_by: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preceded_by: Option<Vec<String>>,
}
impl From<&RelatedProducts> for RelatedProducts {
    fn from(value: &RelatedProducts) -> Self {
        value.clone()
    }
}
impl RelatedProducts {
    pub fn builder() -> builder::RelatedProducts {
        Default::default()
    }
}
#[doc = "Report"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"url\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Report {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl From<&Report> for Report {
    fn from(value: &Report) -> Self {
        value.clone()
    }
}
impl Report {
    pub fn builder() -> builder::Report {
        Default::default()
    }
}
#[doc = "Review"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/scoreReview\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/certification\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Review {
    ScoreReview(ScoreReview),
    Certification(Certification),
}
impl From<&Review> for Review {
    fn from(value: &Review) -> Self {
        value.clone()
    }
}
impl From<ScoreReview> for Review {
    fn from(value: ScoreReview) -> Self {
        Self::ScoreReview(value)
    }
}
impl From<Certification> for Review {
    fn from(value: Certification) -> Self {
        Self::Certification(value)
    }
}
#[doc = "ReviewProducer"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"ids\","]
#[doc = "    \"names\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ids\": {"]
#[doc = "      \"$ref\": \"#/$defs/producerIds\""]
#[doc = "    },"]
#[doc = "    \"images\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"names\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"origins\": {"]
#[doc = "      \"$ref\": \"#/$defs/producerOrigins\""]
#[doc = "    },"]
#[doc = "    \"report\": {"]
#[doc = "      \"$ref\": \"#/$defs/report\""]
#[doc = "    },"]
#[doc = "    \"review\": {"]
#[doc = "      \"$ref\": \"#/$defs/review\""]
#[doc = "    },"]
#[doc = "    \"websites\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReviewProducer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub id: String,
    pub ids: ProducerIds,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<String>,
    pub names: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origins: Option<ProducerOrigins>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report: Option<Report>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review: Option<Review>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub websites: Vec<String>,
}
impl From<&ReviewProducer> for ReviewProducer {
    fn from(value: &ReviewProducer) -> Self {
        value.clone()
    }
}
impl ReviewProducer {
    pub fn builder() -> builder::ReviewProducer {
        Default::default()
    }
}
#[doc = "ReviewProduct"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"ids\","]
#[doc = "    \"names\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"availability\": {"]
#[doc = "      \"$ref\": \"#/$defs/productAvailability\""]
#[doc = "    },"]
#[doc = "    \"categorisation:\": {"]
#[doc = "      \"$ref\": \"#/$defs/productCategorisation\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ids\": {"]
#[doc = "      \"$ref\": \"#/$defs/productIds\""]
#[doc = "    },"]
#[doc = "    \"images\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"names\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"origins\": {"]
#[doc = "      \"$ref\": \"#/$defs/productOrigins\""]
#[doc = "    },"]
#[doc = "    \"related\": {"]
#[doc = "      \"$ref\": \"#/$defs/relatedProducts\""]
#[doc = "    },"]
#[doc = "    \"report\": {"]
#[doc = "      \"$ref\": \"#/$defs/report\""]
#[doc = "    },"]
#[doc = "    \"review\": {"]
#[doc = "      \"$ref\": \"#/$defs/review\""]
#[doc = "    },"]
#[doc = "    \"summary\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReviewProduct {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub availability: Option<ProductAvailability>,
    #[serde(
        rename = "categorisation:",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub categorisation: Option<ProductCategorisation>,
    pub id: String,
    pub ids: ProductIds,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<String>,
    pub names: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origins: Option<ProductOrigins>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<RelatedProducts>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report: Option<Report>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review: Option<Review>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}
impl From<&ReviewProduct> for ReviewProduct {
    fn from(value: &ReviewProduct) -> Self {
        value.clone()
    }
}
impl ReviewProduct {
    pub fn builder() -> builder::ReviewProduct {
        Default::default()
    }
}
#[doc = "ReviewerRoot"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"meta\","]
#[doc = "    \"producers\","]
#[doc = "    \"products\","]
#[doc = "    \"reviewer\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"meta\": {"]
#[doc = "      \"$ref\": \"#/$defs/meta\""]
#[doc = "    },"]
#[doc = "    \"producers\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/reviewProducer\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"products\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/reviewProduct\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"reviewer\": {"]
#[doc = "      \"$ref\": \"#/$defs/aboutReviewer\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReviewerRoot {
    pub meta: Meta,
    pub producers: Vec<ReviewProducer>,
    pub products: Vec<ReviewProduct>,
    pub reviewer: AboutReviewer,
}
impl From<&ReviewerRoot> for ReviewerRoot {
    fn from(value: &ReviewerRoot) -> Self {
        value.clone()
    }
}
impl ReviewerRoot {
    pub fn builder() -> builder::ReviewerRoot {
        Default::default()
    }
}
#[doc = "Root"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/catalogerRoot\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/producerRoot\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/reviewerRoot\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Root {
    CatalogerRoot(CatalogerRoot),
    ProducerRoot(ProducerRoot),
    ReviewerRoot(ReviewerRoot),
}
impl From<&Root> for Root {
    fn from(value: &Root) -> Self {
        value.clone()
    }
}
impl From<CatalogerRoot> for Root {
    fn from(value: CatalogerRoot) -> Self {
        Self::CatalogerRoot(value)
    }
}
impl From<ProducerRoot> for Root {
    fn from(value: ProducerRoot) -> Self {
        Self::ProducerRoot(value)
    }
}
impl From<ReviewerRoot> for Root {
    fn from(value: ReviewerRoot) -> Self {
        Self::ReviewerRoot(value)
    }
}
#[doc = "ScoreReview"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"value\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"maximum\": 100.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ScoreReview {
    pub value: i64,
}
impl From<&ScoreReview> for ScoreReview {
    fn from(value: &ScoreReview) -> Self {
        value.clone()
    }
}
impl ScoreReview {
    pub fn builder() -> builder::ScoreReview {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct AboutCataloger {
        description: Result<Option<String>, String>,
        id: Result<String, String>,
        name: Result<String, String>,
        variant: Result<super::CatalogVariant, String>,
        website: Result<String, String>,
    }
    impl Default for AboutCataloger {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
                variant: Err("no value supplied for variant".to_string()),
                website: Err("no value supplied for website".to_string()),
            }
        }
    }
    impl AboutCataloger {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn variant<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::CatalogVariant>,
            T::Error: std::fmt::Display,
        {
            self.variant = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for variant: {}", e));
            self
        }
        pub fn website<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.website = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for website: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<AboutCataloger> for super::AboutCataloger {
        type Error = super::error::ConversionError;
        fn try_from(value: AboutCataloger) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                id: value.id?,
                name: value.name?,
                variant: value.variant?,
                website: value.website?,
            })
        }
    }
    impl From<super::AboutCataloger> for AboutCataloger {
        fn from(value: super::AboutCataloger) -> Self {
            Self {
                description: Ok(value.description),
                id: Ok(value.id),
                name: Ok(value.name),
                variant: Ok(value.variant),
                website: Ok(value.website),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AboutProducer {
        description: Result<Option<String>, String>,
        id: Result<String, String>,
        ids: Result<super::ProducerIds, String>,
        images: Result<Vec<String>, String>,
        name: Result<String, String>,
        origins: Result<Option<super::ProducerOrigins>, String>,
        website: Result<serde_json::Value, String>,
        websites: Result<Vec<String>, String>,
    }
    impl Default for AboutProducer {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                ids: Err("no value supplied for ids".to_string()),
                images: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                origins: Ok(Default::default()),
                website: Err("no value supplied for website".to_string()),
                websites: Ok(Default::default()),
            }
        }
    }
    impl AboutProducer {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn ids<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ProducerIds>,
            T::Error: std::fmt::Display,
        {
            self.ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ids: {}", e));
            self
        }
        pub fn images<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.images = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for images: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn origins<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ProducerOrigins>>,
            T::Error: std::fmt::Display,
        {
            self.origins = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for origins: {}", e));
            self
        }
        pub fn website<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self.website = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for website: {}", e));
            self
        }
        pub fn websites<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.websites = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for websites: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<AboutProducer> for super::AboutProducer {
        type Error = super::error::ConversionError;
        fn try_from(value: AboutProducer) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                id: value.id?,
                ids: value.ids?,
                images: value.images?,
                name: value.name?,
                origins: value.origins?,
                website: value.website?,
                websites: value.websites?,
            })
        }
    }
    impl From<super::AboutProducer> for AboutProducer {
        fn from(value: super::AboutProducer) -> Self {
            Self {
                description: Ok(value.description),
                id: Ok(value.id),
                ids: Ok(value.ids),
                images: Ok(value.images),
                name: Ok(value.name),
                origins: Ok(value.origins),
                website: Ok(value.website),
                websites: Ok(value.websites),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AboutReviewer {
        description: Result<String, String>,
        id: Result<String, String>,
        name: Result<String, String>,
        reviews: Result<Option<super::AboutReview>, String>,
        website: Result<String, String>,
    }
    impl Default for AboutReviewer {
        fn default() -> Self {
            Self {
                description: Err("no value supplied for description".to_string()),
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
                reviews: Ok(Default::default()),
                website: Err("no value supplied for website".to_string()),
            }
        }
    }
    impl AboutReviewer {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn reviews<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::AboutReview>>,
            T::Error: std::fmt::Display,
        {
            self.reviews = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reviews: {}", e));
            self
        }
        pub fn website<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.website = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for website: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<AboutReviewer> for super::AboutReviewer {
        type Error = super::error::ConversionError;
        fn try_from(value: AboutReviewer) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                id: value.id?,
                name: value.name?,
                reviews: value.reviews?,
                website: value.website?,
            })
        }
    }
    impl From<super::AboutReviewer> for AboutReviewer {
        fn from(value: super::AboutReviewer) -> Self {
            Self {
                description: Ok(value.description),
                id: Ok(value.id),
                name: Ok(value.name),
                reviews: Ok(value.reviews),
                website: Ok(value.website),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AboutScoreReview {
        div: Result<i64, String>,
        max: Result<i64, String>,
        min: Result<i64, String>,
    }
    impl Default for AboutScoreReview {
        fn default() -> Self {
            Self {
                div: Err("no value supplied for div".to_string()),
                max: Err("no value supplied for max".to_string()),
                min: Err("no value supplied for min".to_string()),
            }
        }
    }
    impl AboutScoreReview {
        pub fn div<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.div = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for div: {}", e));
            self
        }
        pub fn max<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.max = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max: {}", e));
            self
        }
        pub fn min<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.min = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for min: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<AboutScoreReview> for super::AboutScoreReview {
        type Error = super::error::ConversionError;
        fn try_from(value: AboutScoreReview) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                div: value.div?,
                max: value.max?,
                min: value.min?,
            })
        }
    }
    impl From<super::AboutScoreReview> for AboutScoreReview {
        fn from(value: super::AboutScoreReview) -> Self {
            Self {
                div: Ok(value.div),
                max: Ok(value.max),
                min: Ok(value.min),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CatalogProducer {
        description: Result<Option<String>, String>,
        id: Result<String, String>,
        ids: Result<super::ProducerIds, String>,
        images: Result<Vec<String>, String>,
        names: Result<Vec<String>, String>,
        origins: Result<Option<super::ProducerOrigins>, String>,
        websites: Result<Vec<String>, String>,
    }
    impl Default for CatalogProducer {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                ids: Err("no value supplied for ids".to_string()),
                images: Ok(Default::default()),
                names: Err("no value supplied for names".to_string()),
                origins: Ok(Default::default()),
                websites: Ok(Default::default()),
            }
        }
    }
    impl CatalogProducer {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn ids<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ProducerIds>,
            T::Error: std::fmt::Display,
        {
            self.ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ids: {}", e));
            self
        }
        pub fn images<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.images = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for images: {}", e));
            self
        }
        pub fn names<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.names = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for names: {}", e));
            self
        }
        pub fn origins<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ProducerOrigins>>,
            T::Error: std::fmt::Display,
        {
            self.origins = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for origins: {}", e));
            self
        }
        pub fn websites<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.websites = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for websites: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<CatalogProducer> for super::CatalogProducer {
        type Error = super::error::ConversionError;
        fn try_from(value: CatalogProducer) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                id: value.id?,
                ids: value.ids?,
                images: value.images?,
                names: value.names?,
                origins: value.origins?,
                websites: value.websites?,
            })
        }
    }
    impl From<super::CatalogProducer> for CatalogProducer {
        fn from(value: super::CatalogProducer) -> Self {
            Self {
                description: Ok(value.description),
                id: Ok(value.id),
                ids: Ok(value.ids),
                images: Ok(value.images),
                names: Ok(value.names),
                origins: Ok(value.origins),
                websites: Ok(value.websites),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CatalogProduct {
        availability: Result<Option<super::ProductAvailability>, String>,
        categorisation: Result<Option<super::ProductCategorisation>, String>,
        description: Result<Option<String>, String>,
        id: Result<String, String>,
        ids: Result<super::ProductIds, String>,
        images: Result<Vec<String>, String>,
        names: Result<Vec<String>, String>,
        origins: Result<Option<super::ProductOrigins>, String>,
        related: Result<Option<super::RelatedProducts>, String>,
    }
    impl Default for CatalogProduct {
        fn default() -> Self {
            Self {
                availability: Ok(Default::default()),
                categorisation: Ok(Default::default()),
                description: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                ids: Err("no value supplied for ids".to_string()),
                images: Ok(Default::default()),
                names: Err("no value supplied for names".to_string()),
                origins: Ok(Default::default()),
                related: Ok(Default::default()),
            }
        }
    }
    impl CatalogProduct {
        pub fn availability<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ProductAvailability>>,
            T::Error: std::fmt::Display,
        {
            self.availability = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for availability: {}", e));
            self
        }
        pub fn categorisation<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ProductCategorisation>>,
            T::Error: std::fmt::Display,
        {
            self.categorisation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for categorisation: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn ids<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ProductIds>,
            T::Error: std::fmt::Display,
        {
            self.ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ids: {}", e));
            self
        }
        pub fn images<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.images = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for images: {}", e));
            self
        }
        pub fn names<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.names = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for names: {}", e));
            self
        }
        pub fn origins<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ProductOrigins>>,
            T::Error: std::fmt::Display,
        {
            self.origins = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for origins: {}", e));
            self
        }
        pub fn related<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::RelatedProducts>>,
            T::Error: std::fmt::Display,
        {
            self.related = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for related: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<CatalogProduct> for super::CatalogProduct {
        type Error = super::error::ConversionError;
        fn try_from(value: CatalogProduct) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                availability: value.availability?,
                categorisation: value.categorisation?,
                description: value.description?,
                id: value.id?,
                ids: value.ids?,
                images: value.images?,
                names: value.names?,
                origins: value.origins?,
                related: value.related?,
            })
        }
    }
    impl From<super::CatalogProduct> for CatalogProduct {
        fn from(value: super::CatalogProduct) -> Self {
            Self {
                availability: Ok(value.availability),
                categorisation: Ok(value.categorisation),
                description: Ok(value.description),
                id: Ok(value.id),
                ids: Ok(value.ids),
                images: Ok(value.images),
                names: Ok(value.names),
                origins: Ok(value.origins),
                related: Ok(value.related),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CatalogerRoot {
        cataloger: Result<super::AboutCataloger, String>,
        meta: Result<super::Meta, String>,
        producers: Result<Vec<super::CatalogProducer>, String>,
        products: Result<Vec<super::CatalogProduct>, String>,
    }
    impl Default for CatalogerRoot {
        fn default() -> Self {
            Self {
                cataloger: Err("no value supplied for cataloger".to_string()),
                meta: Err("no value supplied for meta".to_string()),
                producers: Err("no value supplied for producers".to_string()),
                products: Err("no value supplied for products".to_string()),
            }
        }
    }
    impl CatalogerRoot {
        pub fn cataloger<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::AboutCataloger>,
            T::Error: std::fmt::Display,
        {
            self.cataloger = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cataloger: {}", e));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Meta>,
            T::Error: std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {}", e));
            self
        }
        pub fn producers<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::CatalogProducer>>,
            T::Error: std::fmt::Display,
        {
            self.producers = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for producers: {}", e));
            self
        }
        pub fn products<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::CatalogProduct>>,
            T::Error: std::fmt::Display,
        {
            self.products = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for products: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<CatalogerRoot> for super::CatalogerRoot {
        type Error = super::error::ConversionError;
        fn try_from(value: CatalogerRoot) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                cataloger: value.cataloger?,
                meta: value.meta?,
                producers: value.producers?,
                products: value.products?,
            })
        }
    }
    impl From<super::CatalogerRoot> for CatalogerRoot {
        fn from(value: super::CatalogerRoot) -> Self {
            Self {
                cataloger: Ok(value.cataloger),
                meta: Ok(value.meta),
                producers: Ok(value.producers),
                products: Ok(value.products),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Certification {
        is_certified: Result<Option<bool>, String>,
    }
    impl Default for Certification {
        fn default() -> Self {
            Self {
                is_certified: Ok(Default::default()),
            }
        }
    }
    impl Certification {
        pub fn is_certified<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.is_certified = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_certified: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Certification> for super::Certification {
        type Error = super::error::ConversionError;
        fn try_from(value: Certification) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                is_certified: value.is_certified?,
            })
        }
    }
    impl From<super::Certification> for Certification {
        fn from(value: super::Certification) -> Self {
            Self {
                is_certified: Ok(value.is_certified),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Meta {
        authors: Result<Vec<String>, String>,
        creation_timestamp: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
        description: Result<Option<String>, String>,
        title: Result<String, String>,
        valid_from: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
        valid_to: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
        variant: Result<super::ProviderVariant, String>,
        version: Result<String, String>,
    }
    impl Default for Meta {
        fn default() -> Self {
            Self {
                authors: Ok(Default::default()),
                creation_timestamp: Ok(Default::default()),
                description: Ok(Default::default()),
                title: Err("no value supplied for title".to_string()),
                valid_from: Ok(Default::default()),
                valid_to: Ok(Default::default()),
                variant: Err("no value supplied for variant".to_string()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl Meta {
        pub fn authors<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.authors = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for authors: {}", e));
            self
        }
        pub fn creation_timestamp<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<chrono::DateTime<chrono::offset::Utc>>>,
            T::Error: std::fmt::Display,
        {
            self.creation_timestamp = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for creation_timestamp: {}",
                    e
                )
            });
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
        pub fn valid_from<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<chrono::DateTime<chrono::offset::Utc>>>,
            T::Error: std::fmt::Display,
        {
            self.valid_from = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for valid_from: {}", e));
            self
        }
        pub fn valid_to<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<chrono::DateTime<chrono::offset::Utc>>>,
            T::Error: std::fmt::Display,
        {
            self.valid_to = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for valid_to: {}", e));
            self
        }
        pub fn variant<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ProviderVariant>,
            T::Error: std::fmt::Display,
        {
            self.variant = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for variant: {}", e));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Meta> for super::Meta {
        type Error = super::error::ConversionError;
        fn try_from(value: Meta) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                authors: value.authors?,
                creation_timestamp: value.creation_timestamp?,
                description: value.description?,
                title: value.title?,
                valid_from: value.valid_from?,
                valid_to: value.valid_to?,
                variant: value.variant?,
                version: value.version?,
            })
        }
    }
    impl From<super::Meta> for Meta {
        fn from(value: super::Meta) -> Self {
            Self {
                authors: Ok(value.authors),
                creation_timestamp: Ok(value.creation_timestamp),
                description: Ok(value.description),
                title: Ok(value.title),
                valid_from: Ok(value.valid_from),
                valid_to: Ok(value.valid_to),
                variant: Ok(value.variant),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProducerIds {
        domains: Result<Option<Vec<String>>, String>,
        vat: Result<Option<Vec<String>>, String>,
        wiki: Result<Option<Vec<String>>, String>,
    }
    impl Default for ProducerIds {
        fn default() -> Self {
            Self {
                domains: Ok(Default::default()),
                vat: Ok(Default::default()),
                wiki: Ok(Default::default()),
            }
        }
    }
    impl ProducerIds {
        pub fn domains<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<Vec<String>>>,
            T::Error: std::fmt::Display,
        {
            self.domains = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for domains: {}", e));
            self
        }
        pub fn vat<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<Vec<String>>>,
            T::Error: std::fmt::Display,
        {
            self.vat = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vat: {}", e));
            self
        }
        pub fn wiki<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<Vec<String>>>,
            T::Error: std::fmt::Display,
        {
            self.wiki = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for wiki: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ProducerIds> for super::ProducerIds {
        type Error = super::error::ConversionError;
        fn try_from(value: ProducerIds) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                domains: value.domains?,
                vat: value.vat?,
                wiki: value.wiki?,
            })
        }
    }
    impl From<super::ProducerIds> for ProducerIds {
        fn from(value: super::ProducerIds) -> Self {
            Self {
                domains: Ok(value.domains),
                vat: Ok(value.vat),
                wiki: Ok(value.wiki),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProducerOrigins {
        regions: Result<Option<super::RegionList>, String>,
    }
    impl Default for ProducerOrigins {
        fn default() -> Self {
            Self {
                regions: Ok(Default::default()),
            }
        }
    }
    impl ProducerOrigins {
        pub fn regions<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::RegionList>>,
            T::Error: std::fmt::Display,
        {
            self.regions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for regions: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ProducerOrigins> for super::ProducerOrigins {
        type Error = super::error::ConversionError;
        fn try_from(value: ProducerOrigins) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                regions: value.regions?,
            })
        }
    }
    impl From<super::ProducerOrigins> for ProducerOrigins {
        fn from(value: super::ProducerOrigins) -> Self {
            Self {
                regions: Ok(value.regions),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProducerProduct {
        availability: Result<Option<super::ProductAvailability>, String>,
        categorisation: Result<super::ProductCategorisation, String>,
        description: Result<String, String>,
        id: Result<String, String>,
        ids: Result<super::ProductIds, String>,
        images: Result<Vec<String>, String>,
        name: Result<serde_json::Value, String>,
        names: Result<Vec<String>, String>,
        origins: Result<Option<super::ProductOrigins>, String>,
        related: Result<Option<super::RelatedProducts>, String>,
    }
    impl Default for ProducerProduct {
        fn default() -> Self {
            Self {
                availability: Ok(Default::default()),
                categorisation: Err("no value supplied for categorisation".to_string()),
                description: Err("no value supplied for description".to_string()),
                id: Err("no value supplied for id".to_string()),
                ids: Err("no value supplied for ids".to_string()),
                images: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                names: Ok(Default::default()),
                origins: Ok(Default::default()),
                related: Ok(Default::default()),
            }
        }
    }
    impl ProducerProduct {
        pub fn availability<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ProductAvailability>>,
            T::Error: std::fmt::Display,
        {
            self.availability = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for availability: {}", e));
            self
        }
        pub fn categorisation<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ProductCategorisation>,
            T::Error: std::fmt::Display,
        {
            self.categorisation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for categorisation: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn ids<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ProductIds>,
            T::Error: std::fmt::Display,
        {
            self.ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ids: {}", e));
            self
        }
        pub fn images<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.images = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for images: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn names<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.names = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for names: {}", e));
            self
        }
        pub fn origins<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ProductOrigins>>,
            T::Error: std::fmt::Display,
        {
            self.origins = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for origins: {}", e));
            self
        }
        pub fn related<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::RelatedProducts>>,
            T::Error: std::fmt::Display,
        {
            self.related = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for related: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ProducerProduct> for super::ProducerProduct {
        type Error = super::error::ConversionError;
        fn try_from(value: ProducerProduct) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                availability: value.availability?,
                categorisation: value.categorisation?,
                description: value.description?,
                id: value.id?,
                ids: value.ids?,
                images: value.images?,
                name: value.name?,
                names: value.names?,
                origins: value.origins?,
                related: value.related?,
            })
        }
    }
    impl From<super::ProducerProduct> for ProducerProduct {
        fn from(value: super::ProducerProduct) -> Self {
            Self {
                availability: Ok(value.availability),
                categorisation: Ok(value.categorisation),
                description: Ok(value.description),
                id: Ok(value.id),
                ids: Ok(value.ids),
                images: Ok(value.images),
                name: Ok(value.name),
                names: Ok(value.names),
                origins: Ok(value.origins),
                related: Ok(value.related),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProducerReviewer {
        description: Result<Option<String>, String>,
        id: Result<String, String>,
        name: Result<serde_json::Value, String>,
        names: Result<Vec<String>, String>,
    }
    impl Default for ProducerReviewer {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
                names: Ok(Default::default()),
            }
        }
    }
    impl ProducerReviewer {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn names<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.names = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for names: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ProducerReviewer> for super::ProducerReviewer {
        type Error = super::error::ConversionError;
        fn try_from(value: ProducerReviewer) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                id: value.id?,
                name: value.name?,
                names: value.names?,
            })
        }
    }
    impl From<super::ProducerReviewer> for ProducerReviewer {
        fn from(value: super::ProducerReviewer) -> Self {
            Self {
                description: Ok(value.description),
                id: Ok(value.id),
                name: Ok(value.name),
                names: Ok(value.names),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProducerRoot {
        meta: Result<super::Meta, String>,
        producer: Result<super::AboutProducer, String>,
        producers: Result<serde_json::Value, String>,
        products: Result<Vec<super::ProducerProduct>, String>,
        reviewers: Result<Vec<super::ProducerReviewer>, String>,
    }
    impl Default for ProducerRoot {
        fn default() -> Self {
            Self {
                meta: Err("no value supplied for meta".to_string()),
                producer: Err("no value supplied for producer".to_string()),
                producers: Err("no value supplied for producers".to_string()),
                products: Ok(Default::default()),
                reviewers: Err("no value supplied for reviewers".to_string()),
            }
        }
    }
    impl ProducerRoot {
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Meta>,
            T::Error: std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {}", e));
            self
        }
        pub fn producer<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::AboutProducer>,
            T::Error: std::fmt::Display,
        {
            self.producer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for producer: {}", e));
            self
        }
        pub fn producers<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self.producers = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for producers: {}", e));
            self
        }
        pub fn products<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::ProducerProduct>>,
            T::Error: std::fmt::Display,
        {
            self.products = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for products: {}", e));
            self
        }
        pub fn reviewers<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::ProducerReviewer>>,
            T::Error: std::fmt::Display,
        {
            self.reviewers = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reviewers: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ProducerRoot> for super::ProducerRoot {
        type Error = super::error::ConversionError;
        fn try_from(value: ProducerRoot) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                meta: value.meta?,
                producer: value.producer?,
                producers: value.producers?,
                products: value.products?,
                reviewers: value.reviewers?,
            })
        }
    }
    impl From<super::ProducerRoot> for ProducerRoot {
        fn from(value: super::ProducerRoot) -> Self {
            Self {
                meta: Ok(value.meta),
                producer: Ok(value.producer),
                producers: Ok(value.producers),
                products: Ok(value.products),
                reviewers: Ok(value.reviewers),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProductAvailability {
        regions: Result<super::Regions, String>,
    }
    impl Default for ProductAvailability {
        fn default() -> Self {
            Self {
                regions: Err("no value supplied for regions".to_string()),
            }
        }
    }
    impl ProductAvailability {
        pub fn regions<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Regions>,
            T::Error: std::fmt::Display,
        {
            self.regions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for regions: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ProductAvailability> for super::ProductAvailability {
        type Error = super::error::ConversionError;
        fn try_from(value: ProductAvailability) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                regions: value.regions?,
            })
        }
    }
    impl From<super::ProductAvailability> for ProductAvailability {
        fn from(value: super::ProductAvailability) -> Self {
            Self {
                regions: Ok(value.regions),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProductCategorisation {
        categories: Result<Vec<super::ProductCategory>, String>,
    }
    impl Default for ProductCategorisation {
        fn default() -> Self {
            Self {
                categories: Err("no value supplied for categories".to_string()),
            }
        }
    }
    impl ProductCategorisation {
        pub fn categories<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::ProductCategory>>,
            T::Error: std::fmt::Display,
        {
            self.categories = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for categories: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ProductCategorisation> for super::ProductCategorisation {
        type Error = super::error::ConversionError;
        fn try_from(value: ProductCategorisation) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                categories: value.categories?,
            })
        }
    }
    impl From<super::ProductCategorisation> for ProductCategorisation {
        fn from(value: super::ProductCategorisation) -> Self {
            Self {
                categories: Ok(value.categories),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProductIds {
        ean: Result<Option<Vec<String>>, String>,
        gtin: Result<Option<Vec<String>>, String>,
        wiki: Result<Option<Vec<String>>, String>,
    }
    impl Default for ProductIds {
        fn default() -> Self {
            Self {
                ean: Ok(Default::default()),
                gtin: Ok(Default::default()),
                wiki: Ok(Default::default()),
            }
        }
    }
    impl ProductIds {
        pub fn ean<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<Vec<String>>>,
            T::Error: std::fmt::Display,
        {
            self.ean = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ean: {}", e));
            self
        }
        pub fn gtin<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<Vec<String>>>,
            T::Error: std::fmt::Display,
        {
            self.gtin = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for gtin: {}", e));
            self
        }
        pub fn wiki<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<Vec<String>>>,
            T::Error: std::fmt::Display,
        {
            self.wiki = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for wiki: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ProductIds> for super::ProductIds {
        type Error = super::error::ConversionError;
        fn try_from(value: ProductIds) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                ean: value.ean?,
                gtin: value.gtin?,
                wiki: value.wiki?,
            })
        }
    }
    impl From<super::ProductIds> for ProductIds {
        fn from(value: super::ProductIds) -> Self {
            Self {
                ean: Ok(value.ean),
                gtin: Ok(value.gtin),
                wiki: Ok(value.wiki),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProductOrigins {
        producer_ids: Result<Vec<String>, String>,
        regions: Result<Option<super::RegionList>, String>,
    }
    impl Default for ProductOrigins {
        fn default() -> Self {
            Self {
                producer_ids: Ok(Default::default()),
                regions: Ok(Default::default()),
            }
        }
    }
    impl ProductOrigins {
        pub fn producer_ids<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.producer_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for producer_ids: {}", e));
            self
        }
        pub fn regions<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::RegionList>>,
            T::Error: std::fmt::Display,
        {
            self.regions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for regions: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ProductOrigins> for super::ProductOrigins {
        type Error = super::error::ConversionError;
        fn try_from(value: ProductOrigins) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                producer_ids: value.producer_ids?,
                regions: value.regions?,
            })
        }
    }
    impl From<super::ProductOrigins> for ProductOrigins {
        fn from(value: super::ProductOrigins) -> Self {
            Self {
                producer_ids: Ok(value.producer_ids),
                regions: Ok(value.regions),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RelatedProducts {
        followed_by: Result<Option<Vec<String>>, String>,
        preceded_by: Result<Option<Vec<String>>, String>,
    }
    impl Default for RelatedProducts {
        fn default() -> Self {
            Self {
                followed_by: Ok(Default::default()),
                preceded_by: Ok(Default::default()),
            }
        }
    }
    impl RelatedProducts {
        pub fn followed_by<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<Vec<String>>>,
            T::Error: std::fmt::Display,
        {
            self.followed_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for followed_by: {}", e));
            self
        }
        pub fn preceded_by<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<Vec<String>>>,
            T::Error: std::fmt::Display,
        {
            self.preceded_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for preceded_by: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<RelatedProducts> for super::RelatedProducts {
        type Error = super::error::ConversionError;
        fn try_from(value: RelatedProducts) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                followed_by: value.followed_by?,
                preceded_by: value.preceded_by?,
            })
        }
    }
    impl From<super::RelatedProducts> for RelatedProducts {
        fn from(value: super::RelatedProducts) -> Self {
            Self {
                followed_by: Ok(value.followed_by),
                preceded_by: Ok(value.preceded_by),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Report {
        url: Result<Option<String>, String>,
    }
    impl Default for Report {
        fn default() -> Self {
            Self {
                url: Ok(Default::default()),
            }
        }
    }
    impl Report {
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Report> for super::Report {
        type Error = super::error::ConversionError;
        fn try_from(value: Report) -> Result<Self, super::error::ConversionError> {
            Ok(Self { url: value.url? })
        }
    }
    impl From<super::Report> for Report {
        fn from(value: super::Report) -> Self {
            Self { url: Ok(value.url) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ReviewProducer {
        description: Result<Option<String>, String>,
        id: Result<String, String>,
        ids: Result<super::ProducerIds, String>,
        images: Result<Vec<String>, String>,
        names: Result<Vec<String>, String>,
        origins: Result<Option<super::ProducerOrigins>, String>,
        report: Result<Option<super::Report>, String>,
        review: Result<Option<super::Review>, String>,
        websites: Result<Vec<String>, String>,
    }
    impl Default for ReviewProducer {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                ids: Err("no value supplied for ids".to_string()),
                images: Ok(Default::default()),
                names: Err("no value supplied for names".to_string()),
                origins: Ok(Default::default()),
                report: Ok(Default::default()),
                review: Ok(Default::default()),
                websites: Ok(Default::default()),
            }
        }
    }
    impl ReviewProducer {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn ids<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ProducerIds>,
            T::Error: std::fmt::Display,
        {
            self.ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ids: {}", e));
            self
        }
        pub fn images<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.images = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for images: {}", e));
            self
        }
        pub fn names<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.names = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for names: {}", e));
            self
        }
        pub fn origins<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ProducerOrigins>>,
            T::Error: std::fmt::Display,
        {
            self.origins = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for origins: {}", e));
            self
        }
        pub fn report<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Report>>,
            T::Error: std::fmt::Display,
        {
            self.report = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for report: {}", e));
            self
        }
        pub fn review<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Review>>,
            T::Error: std::fmt::Display,
        {
            self.review = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for review: {}", e));
            self
        }
        pub fn websites<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.websites = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for websites: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ReviewProducer> for super::ReviewProducer {
        type Error = super::error::ConversionError;
        fn try_from(value: ReviewProducer) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                id: value.id?,
                ids: value.ids?,
                images: value.images?,
                names: value.names?,
                origins: value.origins?,
                report: value.report?,
                review: value.review?,
                websites: value.websites?,
            })
        }
    }
    impl From<super::ReviewProducer> for ReviewProducer {
        fn from(value: super::ReviewProducer) -> Self {
            Self {
                description: Ok(value.description),
                id: Ok(value.id),
                ids: Ok(value.ids),
                images: Ok(value.images),
                names: Ok(value.names),
                origins: Ok(value.origins),
                report: Ok(value.report),
                review: Ok(value.review),
                websites: Ok(value.websites),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ReviewProduct {
        availability: Result<Option<super::ProductAvailability>, String>,
        categorisation: Result<Option<super::ProductCategorisation>, String>,
        id: Result<String, String>,
        ids: Result<super::ProductIds, String>,
        images: Result<Vec<String>, String>,
        names: Result<Vec<String>, String>,
        origins: Result<Option<super::ProductOrigins>, String>,
        related: Result<Option<super::RelatedProducts>, String>,
        report: Result<Option<super::Report>, String>,
        review: Result<Option<super::Review>, String>,
        summary: Result<Option<String>, String>,
    }
    impl Default for ReviewProduct {
        fn default() -> Self {
            Self {
                availability: Ok(Default::default()),
                categorisation: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                ids: Err("no value supplied for ids".to_string()),
                images: Ok(Default::default()),
                names: Err("no value supplied for names".to_string()),
                origins: Ok(Default::default()),
                related: Ok(Default::default()),
                report: Ok(Default::default()),
                review: Ok(Default::default()),
                summary: Ok(Default::default()),
            }
        }
    }
    impl ReviewProduct {
        pub fn availability<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ProductAvailability>>,
            T::Error: std::fmt::Display,
        {
            self.availability = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for availability: {}", e));
            self
        }
        pub fn categorisation<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ProductCategorisation>>,
            T::Error: std::fmt::Display,
        {
            self.categorisation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for categorisation: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn ids<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ProductIds>,
            T::Error: std::fmt::Display,
        {
            self.ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ids: {}", e));
            self
        }
        pub fn images<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.images = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for images: {}", e));
            self
        }
        pub fn names<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.names = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for names: {}", e));
            self
        }
        pub fn origins<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ProductOrigins>>,
            T::Error: std::fmt::Display,
        {
            self.origins = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for origins: {}", e));
            self
        }
        pub fn related<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::RelatedProducts>>,
            T::Error: std::fmt::Display,
        {
            self.related = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for related: {}", e));
            self
        }
        pub fn report<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Report>>,
            T::Error: std::fmt::Display,
        {
            self.report = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for report: {}", e));
            self
        }
        pub fn review<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Review>>,
            T::Error: std::fmt::Display,
        {
            self.review = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for review: {}", e));
            self
        }
        pub fn summary<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.summary = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for summary: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ReviewProduct> for super::ReviewProduct {
        type Error = super::error::ConversionError;
        fn try_from(value: ReviewProduct) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                availability: value.availability?,
                categorisation: value.categorisation?,
                id: value.id?,
                ids: value.ids?,
                images: value.images?,
                names: value.names?,
                origins: value.origins?,
                related: value.related?,
                report: value.report?,
                review: value.review?,
                summary: value.summary?,
            })
        }
    }
    impl From<super::ReviewProduct> for ReviewProduct {
        fn from(value: super::ReviewProduct) -> Self {
            Self {
                availability: Ok(value.availability),
                categorisation: Ok(value.categorisation),
                id: Ok(value.id),
                ids: Ok(value.ids),
                images: Ok(value.images),
                names: Ok(value.names),
                origins: Ok(value.origins),
                related: Ok(value.related),
                report: Ok(value.report),
                review: Ok(value.review),
                summary: Ok(value.summary),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ReviewerRoot {
        meta: Result<super::Meta, String>,
        producers: Result<Vec<super::ReviewProducer>, String>,
        products: Result<Vec<super::ReviewProduct>, String>,
        reviewer: Result<super::AboutReviewer, String>,
    }
    impl Default for ReviewerRoot {
        fn default() -> Self {
            Self {
                meta: Err("no value supplied for meta".to_string()),
                producers: Err("no value supplied for producers".to_string()),
                products: Err("no value supplied for products".to_string()),
                reviewer: Err("no value supplied for reviewer".to_string()),
            }
        }
    }
    impl ReviewerRoot {
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Meta>,
            T::Error: std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {}", e));
            self
        }
        pub fn producers<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::ReviewProducer>>,
            T::Error: std::fmt::Display,
        {
            self.producers = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for producers: {}", e));
            self
        }
        pub fn products<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::ReviewProduct>>,
            T::Error: std::fmt::Display,
        {
            self.products = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for products: {}", e));
            self
        }
        pub fn reviewer<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::AboutReviewer>,
            T::Error: std::fmt::Display,
        {
            self.reviewer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reviewer: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ReviewerRoot> for super::ReviewerRoot {
        type Error = super::error::ConversionError;
        fn try_from(value: ReviewerRoot) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                meta: value.meta?,
                producers: value.producers?,
                products: value.products?,
                reviewer: value.reviewer?,
            })
        }
    }
    impl From<super::ReviewerRoot> for ReviewerRoot {
        fn from(value: super::ReviewerRoot) -> Self {
            Self {
                meta: Ok(value.meta),
                producers: Ok(value.producers),
                products: Ok(value.products),
                reviewer: Ok(value.reviewer),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ScoreReview {
        value: Result<i64, String>,
    }
    impl Default for ScoreReview {
        fn default() -> Self {
            Self {
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl ScoreReview {
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ScoreReview> for super::ScoreReview {
        type Error = super::error::ConversionError;
        fn try_from(value: ScoreReview) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                value: value.value?,
            })
        }
    }
    impl From<super::ScoreReview> for ScoreReview {
        fn from(value: super::ScoreReview) -> Self {
            Self {
                value: Ok(value.value),
            }
        }
    }
}
