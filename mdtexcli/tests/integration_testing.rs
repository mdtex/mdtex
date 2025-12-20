// #[test]
//     fn test_url_works() {
//         let url_res = CTANSend {
//             phrase: "knuth".to_string(),
//             max: Some(1),
//             ext: Some(true),
//             pkg: Some(true),
//             authors: Some(true),
//             topics:Some(true),
//             portal: Some(true),
//         }.construct_url();

//         assert!(url_res.is_ok());

//         let url = url_res.unwrap();

//         assert_eq!(
//             url.to_string(),
//             "https://ctan.org/search/json?phrase=knuth&max=1&ext=true&PKG=true&AUTHORS=true&TOPICS=true&PORTAL=true"
//         );
//     }

//     #[test]
//     fn test_query_runs() {
//         // ctan query with knuth
//         let query = CTANSend {
//             phrase: "knuth".to_string(),
//             ..Default::default()
//         };

//         let result = CTANReturn::query_ctan(query);

//         assert!(result.is_ok());

//         let ctan_return = result.unwrap();

//         let expected_json = r#"
//             {
//                 "numberOfHits": 110,
//                 "offset": 0,
//                 "max": 16,
//                 "phrase": "knuth",
//                 "hits": [
//                     {
//                         "title": "Package knuth-base",
//                         "path": "/pkg/knuth-base",
//                         "text": "The current state of Knuth's contributions"
//                     },
//                     {
//                         "title": "Package knuth-dist",
//                         "path": "/pkg/knuth-dist",
//                         "text": "The current state of Knuth's contributions"
//                     },
//                     {
//                         "title": "Package knuth-lib",
//                         "path": "/pkg/knuth-lib",
//                         "text": "Core TeX and Metafont sources from Knuth"
//                     },
//                     {
//                         "title": "Package knuth-errata",
//                         "path": "/pkg/knuth-errata",
//                         "text": "Knuth’s published errata"
//                     },
//                     {
//                         "title": "Package knuth-local",
//                         "path": "/pkg/knuth-local",
//                         "text": "Knuth’s local information"
//                     },
//                     {
//                         "title": "Package knuth-letter",
//                         "path": "/pkg/knuth-letter",
//                         "text": "Knuth’s example letter macros"
//                     },
//                     {
//                         "title": "Package vf-knuth",
//                         "path": "/pkg/vf-knuth",
//                         "text": "Knuth on virtual fonts"
//                     },
//                     {
//                         "title": "Package knuth-pdf",
//                         "path": "/pkg/knuth-pdf",
//                         "text": "PDF collection of typeset C/WEB sources in TeX Live"
//                     },
//                     {
//                         "title": "Package knuth-hint",
//                         "path": "/pkg/knuth-hint",
//                         "text": "HINT collection of typeset C/WEB sources in TeX Live"
//                     },
//                     {
//                         "title": "Package type1cm",
//                         "path": "/pkg/type1cm",
//                         "text": "Arbitrary size font selection in LaTeX"
//                     },
//                     {
//                         "title": "Package aweb",
//                         "path": "/pkg/aweb",
//                         "text": "Web system for programs written in Ada"
//                     },
//                     {
//                         "title": "Package afmtopl-elwell",
//                         "path": "/pkg/afmtopl-elwell",
//                         "text": "An early converter from AFM to PL"
//                     },
//                     {
//                         "title": "Package beton",
//                         "path": "/pkg/beton",
//                         "text": "Use Concrete fonts"
//                     },
//                     {
//                         "title": "Package cmtest",
//                         "path": "/pkg/cmtest",
//                         "text": "CM fonts test sources"
//                     },
//                     {
//                         "title": "Package aifont",
//                         "path": "/pkg/aifont",
//                         "text": "Remap Computer Modern fonts"
//                     },
//                     {
//                         "title": "Package genealogy",
//                         "path": "/pkg/genealogy",
//                         "text": "A compilation genealogy font"
//                     }
//                 ]
//             }
//         "#;

//         let expected_return = serde_json::from_str::<CTANReturn>(expected_json).unwrap();

//         assert_eq!(ctan_return, expected_return);
//     }

//     #[test]
//     fn test_query_nothing() {
//         let query = CTANSend {
//             phrase: "this should return nothing".to_string(),
//             ..Default::default()
//         };

//         let result = CTANReturn::query_ctan(query);

//         assert!(result.is_ok());

//         let ctan_return = result.unwrap();

//         let expected_json = r#"
//             {
//                 "numberOfHits": 0,
//                 "offset": 0,
//                 "max": 16,
//                 "phrase": "this should return nothing",
//                 "hits": []
//             }
//         "#;

//         let expected_return = serde_json::from_str::<CTANReturn>(expected_json).unwrap();

//         assert_eq!(ctan_return, expected_return);
//     }