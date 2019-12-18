// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [CommunityFeatures Lookups](https://ddwiki.reso.org/display/DDW17/CommunityFeatures+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CommunityFeatures {
    /// "[Airport/Runway](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244536)": The community has an airport or runway.
    AirportRunway,

    /// "[Clubhouse](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244537)": The community has a clubhouse.
    Clubhouse,

    /// "[Curbs](https://ddwiki.reso.org/display/DDW17/Curbs)": The community streets have curbs.
    Curbs,

    /// "[Fishing](https://ddwiki.reso.org/display/DDW17/Fishing)": The community has places to go fishing.
    Fishing,

    /// "[Fitness Center](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244540)": The community has a fitness center.
    FitnessCenter,

    /// "[Gated](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244541)": The community is gated.
    Gated,

    /// "[Golf](https://ddwiki.reso.org/display/DDW17/Golf)": The community has golfing.
    Golf,

    /// "[Lake](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244543)": The community has a lake.
    Lake,

    /// "[None](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244544)": The community includes no additional features.
    None,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244545)": The community has features beyond those listed in this field.
    Other,

    /// "[Park](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244546)": The community has a park.
    Park,

    /// "[Playground](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244547)": The community has a playground.
    Playground,

    /// "[Pool](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244548)": The community has a pool.
    Pool,

    /// "[Racquetball](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244549)": The community has racquetball facilities.
    Racquetball,

    /// "[Restaurant](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244550)": The community has a restaurant.
    Restaurant,

    /// "[Sidewalks](https://ddwiki.reso.org/display/DDW17/Sidewalks)": The community streets have sidewalks.
    Sidewalks,

    /// "[Stable(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244552)": The community has horse stables.
    Stables,

    /// "[Street Lights](https://ddwiki.reso.org/display/DDW17/Street+Lights)": The community streets have lighting.
    StreetLights,

    /// "[Suburban](https://ddwiki.reso.org/display/DDW17/Suburban)": The community is a suburban setting.
    Suburban,

    /// "[Tennis Court(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244555)": The community has tennis court(s).
    TennisCourts,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for CommunityFeatures {
    fn from(s: String) -> CommunityFeatures {
        match s.as_ref() {
            "Airport/Runway" => CommunityFeatures::AirportRunway,

            "Clubhouse" => CommunityFeatures::Clubhouse,

            "Curbs" => CommunityFeatures::Curbs,

            "Fishing" => CommunityFeatures::Fishing,

            "Fitness Center" => CommunityFeatures::FitnessCenter,

            "Gated" => CommunityFeatures::Gated,

            "Golf" => CommunityFeatures::Golf,

            "Lake" => CommunityFeatures::Lake,

            "None" => CommunityFeatures::None,

            "Other" => CommunityFeatures::Other,

            "Park" => CommunityFeatures::Park,

            "Playground" => CommunityFeatures::Playground,

            "Pool" => CommunityFeatures::Pool,

            "Racquetball" => CommunityFeatures::Racquetball,

            "Restaurant" => CommunityFeatures::Restaurant,

            "Sidewalks" => CommunityFeatures::Sidewalks,

            "Stable(s)" => CommunityFeatures::Stables,

            "Street Lights" => CommunityFeatures::StreetLights,

            "Suburban" => CommunityFeatures::Suburban,

            "Tennis Court(s)" => CommunityFeatures::TennisCourts,

            _ => CommunityFeatures::OpenEnumeration(s),
        }
    }
}

impl From<&str> for CommunityFeatures {
    fn from(s: &str) -> CommunityFeatures {
        match s {
            "Airport/Runway" => CommunityFeatures::AirportRunway,

            "Clubhouse" => CommunityFeatures::Clubhouse,

            "Curbs" => CommunityFeatures::Curbs,

            "Fishing" => CommunityFeatures::Fishing,

            "Fitness Center" => CommunityFeatures::FitnessCenter,

            "Gated" => CommunityFeatures::Gated,

            "Golf" => CommunityFeatures::Golf,

            "Lake" => CommunityFeatures::Lake,

            "None" => CommunityFeatures::None,

            "Other" => CommunityFeatures::Other,

            "Park" => CommunityFeatures::Park,

            "Playground" => CommunityFeatures::Playground,

            "Pool" => CommunityFeatures::Pool,

            "Racquetball" => CommunityFeatures::Racquetball,

            "Restaurant" => CommunityFeatures::Restaurant,

            "Sidewalks" => CommunityFeatures::Sidewalks,

            "Stable(s)" => CommunityFeatures::Stables,

            "Street Lights" => CommunityFeatures::StreetLights,

            "Suburban" => CommunityFeatures::Suburban,

            "Tennis Court(s)" => CommunityFeatures::TennisCourts,

            _ => CommunityFeatures::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a CommunityFeatures> for &'a str {
    fn from(s: &'a CommunityFeatures) -> &'a str {
        match s {
            CommunityFeatures::AirportRunway => "Airport/Runway",

            CommunityFeatures::Clubhouse => "Clubhouse",

            CommunityFeatures::Curbs => "Curbs",

            CommunityFeatures::Fishing => "Fishing",

            CommunityFeatures::FitnessCenter => "Fitness Center",

            CommunityFeatures::Gated => "Gated",

            CommunityFeatures::Golf => "Golf",

            CommunityFeatures::Lake => "Lake",

            CommunityFeatures::None => "None",

            CommunityFeatures::Other => "Other",

            CommunityFeatures::Park => "Park",

            CommunityFeatures::Playground => "Playground",

            CommunityFeatures::Pool => "Pool",

            CommunityFeatures::Racquetball => "Racquetball",

            CommunityFeatures::Restaurant => "Restaurant",

            CommunityFeatures::Sidewalks => "Sidewalks",

            CommunityFeatures::Stables => "Stable(s)",

            CommunityFeatures::StreetLights => "Street Lights",

            CommunityFeatures::Suburban => "Suburban",

            CommunityFeatures::TennisCourts => "Tennis Court(s)",

            CommunityFeatures::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for CommunityFeatures {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for CommunityFeatures {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_community_features_format {
    use super::CommunityFeatures;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<CommunityFeatures>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match items {
            None => return serializer.serialize_none(),
            Some(ref vec) if vec.len() == 0 => serializer.serialize_str(""),
            Some(ref vec) => {
                let items: Vec<&str> = vec.iter().map(|item| item.into()).collect();
                let joined = items.join(",");
                serializer.serialize_str(&joined)
            }
        }
    }

    #[allow(dead_code)]
    pub(crate) fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Option<Vec<CommunityFeatures>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == "" {
            return Ok(Some(vec![]));
        }

        let items = s.split(",").map(|i| From::<&str>::from(i)).collect();
        Ok(Some(items))
    }
}
