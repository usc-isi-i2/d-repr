///// Convert a string annotated type: "2015-01-01T00:00:00"^^xsd::dateTime
///// into a correct value for
///// To a correct
//pub fn string2value(val: String) -> Value {
//
//}

/// Checking if a string is absolute URI or relative URI.
///
/// An absolute URI is a URL with no fragment component, e.g: `https?://...`
/// See more [here](https://en.wikipedia.org/wiki/Uniform_Resource_Identifier#URI_resolution)
///
/// # Arguments
///
/// * `uri` - A string to test
///
/// # Examples
///
/// ```
/// # use drepr::utils::rdf;
/// assert!(rdf::is_absolute_uri("http://example.org/users"))
/// ```
pub fn is_absolute_uri(uri: &str) -> bool {
  return uri.starts_with("http://") || uri.starts_with("https://");
}


/// Split a prefixed URI to two parts: (namespace, relative_url)
///
/// # Arguments
///
/// * `uri` - A string to split
///
/// # Examples
///
/// ```
/// # use drepr::utils::rdf;
/// assert_eq!(rdf::split_prefixed_uri("eg:Student"), ("eg", "Student"))
/// ```
pub fn split_prefixed_uri(uri: &str) -> (&str, &str) {
  let mut iter = uri.splitn(2, ":");
  (iter.next().unwrap(), iter.next().unwrap())
}