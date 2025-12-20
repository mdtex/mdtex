use url::Url;

use serde::{Serialize, Deserialize};

use crate::project_management::Package;

/// Link to CTAN's querying API
pub const CTAN_URL: &str = "https://ctan.org/search/json?";

/// Structure of search query to CTAN's API
#[derive(Default, Debug)]
pub struct CTANSend {
    /// Search phrase used in our query
    pub phrase: String,

    /// Maximum number of hits allowed
    pub max: Option<i32>,

    /// Toggle to determine whether or not sections are explicitly requested
    pub ext: Option<bool>,

    /// Determines if package section should be included in result
    pub pkg: Option<bool>,

    /// Determines if authors section should be included in result
    pub authors: Option<bool>,

    /// Determines if topics section should be included in result
    pub topics: Option<bool>,

    /// Determines if portal section should be included in result
    pub portal: Option<bool>,
}

impl CTANSend {
    fn construct_url_with_base(&self, url: &str) -> anyhow::Result<Url> {
        let mut url = Url::parse(url)?;
        
        /* urlify our fields */
        url.query_pairs_mut().append_pair("phrase", &self.phrase);

        if let Some(max) = self.max {
            url.query_pairs_mut().append_pair("max", &max.to_string());
        }

        if let Some(ext) = self.ext {
            url.query_pairs_mut().append_pair("ext", &ext.to_string());
        }

        if let Some(pkg) = self.pkg {
            url.query_pairs_mut().append_pair("PKG", &pkg.to_string());
        }

        if let Some(authors) = self.authors {
            url.query_pairs_mut().append_pair("AUTHORS", &authors.to_string());
        }

        if let Some(topics) = self.topics {
            url.query_pairs_mut().append_pair("TOPICS", &topics.to_string());
        }
        
        if let Some(portal) = self.portal {
            url.query_pairs_mut().append_pair("PORTAL", &portal.to_string());
        }

        Ok(url)
    }

    /// Construct a query url given the CTANSend structs fields
    fn construct_ctan_url(&self) -> anyhow::Result<Url> {
        self.construct_url_with_base(CTAN_URL)
    }
}

/// Structure of a returned query to CTAN's API
#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct CTANReturn {
    /// Number of hits from a given search phrase
    #[serde(alias="numberOfHits")]
    pub number_of_hits: i32,

    /// Page offset at which the returned hits for a search phrase start
    offset: i32,

    /// Maximum number of hits as specified by our query
    max: i32,

    /// Search phrase used in our query
    phrase: String,

    /// Packages returned by our query
    pub hits: Vec<Package>
}

impl CTANReturn {
    /// For testing purposes
    pub(crate) fn query_ctan_with_base(query: CTANSend, url: &str) -> anyhow::Result<Self> {
        let url = query.construct_url_with_base(url)?;

        // use reqwest to fetch the url result - naïve blocking is alright since we don't have any parallelism (yet)
        let resp = reqwest::blocking::get(url)?;

        // return result
        Ok(resp.json::<Self>()?)
    }

    /// Query CTAN using a given CTANSend struct; returns a CTANReturn result
    pub fn query_ctan(query: CTANSend) -> anyhow::Result<Self> {
        // construct our url to use in the query
       Self::query_ctan_with_base(query, CTAN_URL)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    

    const EXPECTED_TEST_JSON: &str = r#"
        {
            "numberOfHits": 110,
            "offset": 0,
            "max": 16,
            "phrase": "knuth",
            "hits": [
                { "title": "Package knuth-base", "path": "/pkg/knuth-base", "text": "The current state of Knuth's contributions" },
                { "title": "Package knuth-dist",  "path": "/pkg/knuth-dist",  "text": "The current state of Knuth's contributions" },
                { "title": "Package knuth-lib",  "path": "/pkg/knuth-lib",  "text": "Core TeX and Metafont sources from Knuth" },
                { "title": "Package knuth-errata", "path": "/pkg/knuth-errata", "text": "Knuth’s published errata" },
                { "title": "Package knuth-local", "path": "/pkg/knuth-local", "text": "Knuth’s local information" },
                { "title": "Package knuth-letter", "path": "/pkg/knuth-letter", "text": "Knuth’s example letter macros" },
                { "title": "Package vf-knuth", "path": "/pkg/vf-knuth", "text": "Knuth on virtual fonts" },
                { "title": "Package knuth-pdf", "path": "/pkg/knuth-pdf", "text": "PDF collection of typeset C/WEB sources in TeX Live" },
                { "title": "Package knuth-hint", "path": "/pkg/knuth-hint", "text": "HINT collection of typeset C/WEB sources in TeX Live" },
                { "title": "Package type1cm", "path": "/pkg/type1cm", "text": "Arbitrary size font selection in LaTeX" },
                { "title": "Package aweb", "path": "/pkg/aweb", "text": "Web system for programs written in Ada" },
                { "title": "Package afmtopl-elwell", "path": "/pkg/afmtopl-elwell", "text": "An early converter from AFM to PL" },
                { "title": "Package beton", "path": "/pkg/beton", "text": "Use Concrete fonts" },
                { "title": "Package cmtest", "path": "/pkg/cmtest", "text": "CM fonts test sources" },
                { "title": "Package aifont", "path": "/pkg/aifont", "text": "Remap Computer Modern fonts" },
                { "title": "Package genealogy", "path": "/pkg/genealogy", "text": "A compilation genealogy font" }
            ]
        }
    "#;

    #[test]
    fn test_url_works() {
        let url_res = CTANSend {
            phrase: "knuth".to_string(),
            max: Some(1),
            ext: Some(true),
            pkg: Some(true),
            authors: Some(true),
            topics: Some(true),
            portal: Some(true),
        }.construct_ctan_url();

        assert!(url_res.is_ok());
        let url = url_res.unwrap();

        assert_eq!(
            url.to_string(),
            "https://ctan.org/search/json?phrase=knuth&max=1&ext=true&PKG=true&AUTHORS=true&TOPICS=true&PORTAL=true"
        );
    }

    #[test]
    fn test_query_runs() {
        let mut server = mockito::Server::new();
        let mut url = server.url();

        // Mock server behavior
        let mock = server.mock("GET", "/search/json")
            .match_query(mockito::Matcher::UrlEncoded("phrase".into(), "knuth".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(EXPECTED_TEST_JSON)
            .create();

        // Query with mock base URL
        let query = CTANSend {
            phrase: "knuth".to_string(),
            ..Default::default()
        };

        url.push_str("/search/json?");

        let result = CTANReturn::query_ctan_with_base(query, &url);
        assert!(result.is_ok());

        let ctan_return = result.unwrap();
        let expected_return = serde_json::from_str::<CTANReturn>(EXPECTED_TEST_JSON).unwrap();
        assert_eq!(ctan_return, expected_return);

        mock.assert();
    }

    #[test]
    fn test_query_nothing() {
        let expected_json = r#"
            {
                "numberOfHits": 0,
                "offset": 0,
                "max": 16,
                "phrase": "this should return nothing",
                "hits": []
            }
        "#;

        let mut server = mockito::Server::new();
        let mut url = server.url();

        // Mock server behavior
        let mock = server.mock("GET", "/search/json")
            .match_query(mockito::Matcher::UrlEncoded("phrase".into(), "knuth".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(expected_json)
            .create();

        // Query with mock base URL
        let query = CTANSend {
            phrase: "knuth".to_string(),
            ..Default::default()
        };

        url.push_str("/search/json");

        let result = CTANReturn::query_ctan_with_base(query, &url);
        assert!(result.is_ok());

        let ctan_return = result.unwrap();
        let expected_return = serde_json::from_str::<CTANReturn>(expected_json).unwrap();
        assert_eq!(ctan_return, expected_return);

        mock.assert();
    }

    #[test]
    fn test_url_with_defaults_only_phrase() {
        let query = CTANSend {
            phrase: "test".to_string(),
            ..Default::default()
        };

        let url = query.construct_ctan_url().unwrap();
        assert_eq!(url.as_str(), "https://ctan.org/search/json?phrase=test");
    }

    #[test]
    fn test_url_mixed_optional_fields() {
        let query = CTANSend {
            phrase: "hello".to_string(),
            max: Some(5),
            ext: None,
            pkg: Some(true),
            authors: None,
            topics: Some(false),
            portal: None,
        };

        let url = query.construct_ctan_url().unwrap();
        let s = url.as_str();

        assert!(s.contains("phrase=hello"));
        assert!(s.contains("max=5"));
        assert!(s.contains("PKG=true"));
        assert!(s.contains("TOPICS=false"));

        // Ensure omitted ones do not appear
        assert!(!s.contains("ext="));
        assert!(!s.contains("AUTHORS="));
        assert!(!s.contains("PORTAL="));
    }

    #[test]
    fn test_url_encoding_phrase() {
        let query = CTANSend {
            phrase: "C++ programming".to_string(),
            ..Default::default()
        };

        let url = query.construct_ctan_url().unwrap();
        let s = url.as_str();

        assert!(s.contains("C%2B%2B+programming"));
        assert_eq!(
            s,
            "https://ctan.org/search/json?phrase=C%2B%2B+programming"
        );
    }

    #[test]
    fn test_bad_base_url_rejected() {
        let query = CTANSend {
            phrase: "x".to_string(),
            ..Default::default()
        };

        let result = query.construct_url_with_base("ht!tp:// not a valid url");
        assert!(result.is_err());
    }

    #[test]
    fn test_query_http_500_error() {
        let mut server = mockito::Server::new();
        let mut url = server.url();
        url.push_str("/search/json?");

        let mock = server.mock("GET", "/search/json")
            .match_query(mockito::Matcher::Any)
            .with_status(500)
            .with_body("internal error")
            .create();

        let query = CTANSend {
            phrase: "knuth".to_string(),
            ..Default::default()
        };

        let result = CTANReturn::query_ctan_with_base(query, &url);

        assert!(result.is_err());
        mock.assert();
    }

    #[test]
    fn test_query_malformed_json_error() {
        let bad_json = "{ this is not valid JSON ";

        let mut server = mockito::Server::new();
        let mut url = server.url();
        url.push_str("/search/json?");

        let mock = server.mock("GET", "/search/json")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(bad_json)
            .create();

        let query = CTANSend {
            phrase: "knuth".to_string(),
            ..Default::default()
        };

        let result = CTANReturn::query_ctan_with_base(query, &url);

        assert!(result.is_err());
        mock.assert();
    }

    #[test]
    fn test_deserialize_missing_optional_hits() {
        let json = r#"
        {
            "numberOfHits": 0,
            "offset": 0,
            "max": 16,
            "phrase": "x"
        }
        "#;

        let parsed = serde_json::from_str::<CTANReturn>(json);
        // Deserialization must fail because hits: Vec<Package> is required
        assert!(parsed.is_err());
    }

    #[test]
    fn test_deserialize_ignores_unknown_fields() {
        let json = r#"
        {
            "numberOfHits": 1,
            "offset": 0,
            "max": 16,
            "phrase": "abc",
            "hits": [],
            "extra_field": "ignored value",
            "another": 123
        }
        "#;

        let parsed = serde_json::from_str::<CTANReturn>(json);
        assert!(parsed.is_ok());
        let ret = parsed.unwrap();
        assert_eq!(ret.number_of_hits, 1);
        assert!(ret.hits.is_empty());
    }

    #[test]
    fn test_round_trip_serialization() {
        let ret = CTANReturn {
            number_of_hits: 2,
            offset: 0,
            max: 10,
            phrase: "abc".to_string(),
            hits: vec![
                Package {
                    title: "One".to_string(),
                    path: "/pkg/one".to_string(),
                    text: Some("First".to_string()),
                },
                Package {
                    title: "Two".to_string(),
                    path: "/pkg/two".to_string(),
                    text: Some("Second".to_string()),
                },
            ],
        };

        let json = serde_json::to_string(&ret).unwrap();
        let parsed = serde_json::from_str::<CTANReturn>(&json).unwrap();

        assert_eq!(ret, parsed);
    }
}