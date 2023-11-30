pub mod types {

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize)]
    pub struct Identifier {
        pub id: String,
        pub uuid: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Location {
        #[serde(rename = "$mid")]
        pub mid: i32,
        pub path: String,
        pub scheme: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Metadata {
        pub id: String,
        pub publisherId: String,
        pub publisherDisplayName: String,
        pub targetPlatform: String,
        pub isApplicationScoped: Option<bool>,
        pub updated: bool,
        pub isPreReleaseVersion: bool,
        pub installedTimestamp: i64,
        pub preRelease: Option<bool>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Extension {
        pub identifier: Identifier,
        pub version: String,
        pub location: Location,
        pub relativeLocation: String,
        pub metadata: Metadata,
    }
}
