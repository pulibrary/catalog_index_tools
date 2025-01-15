use std::env;
mod parse_json;

pub fn searches() -> [Search; 7] {
    [
        Search::new("/catalog.json?f[advanced_location_s][]=scsbcul", "Columbia"),
        Search::new("/catalog.json?f[advanced_location_s][]=scsbhl", "Harvard"),
        Search::new(
            "/catalog.json?f[advanced_location_s][]=scsbnypl",
            "New York Public",
        ),
        Search::new(
            "/catalog.json?f[access_facet][]=In+the+Library",
            "In the Library",
        ),
        Search::new("/catalog.json?f[access_facet][]=Online", "Online"),
        Search::new("/catalog.json?f[format][]=Coin", "Coins"),
        Search::new("/catalog.json?f[format][]=Senior+thesis", "Theses"),
    ]
}

#[derive(Debug, PartialEq)]
pub enum RecordCountComparison {
    UnusuallySmaller,
    Smaller,
    Same,
    Larger,
    UnusuallyLarger,
}

impl RecordCountComparison {
    pub fn compare(old_count: u32, new_count: u32) -> RecordCountComparison {
        match (old_count, new_count) {
            (old, new) if Self::is_too_big(old, new) => RecordCountComparison::UnusuallyLarger,
            (old, new) if new > old => RecordCountComparison::Larger,
            (old, new) if Self::is_too_small(old, new) => RecordCountComparison::UnusuallySmaller,
            (old, new) if new < old => RecordCountComparison::Smaller,
            _ => RecordCountComparison::Same,
        }
    }

    fn is_too_big(old_count: u32, new_count: u32) -> bool {
        let threshold = ((old_count as f64) * 1.0075).round() as u32;
        new_count > threshold
    }

    fn is_too_small(old_count: u32, new_count: u32) -> bool {
        let threshold = ((old_count as f64) * 0.9975).round() as u32;
        new_count < threshold
    }
}

pub struct Search {
    path: String,
    name: String,
    client: reqwest::blocking::Client,
}

impl Search {
    pub fn new(path: &str, name: &str) -> Search {
        Search {
            path: path.into(),
            name: name.into(),
            client: reqwest::blocking::Client::builder()
                .timeout(None)
                .build()
                .unwrap(),
        }
    }

    fn new_url(&self) -> String {
        let prefix =
            env::var("FUTURE_PROD").unwrap_or("https://catalog-qa.princeton.edu".to_string());
        format!("{}{}", prefix, self.path)
    }

    fn old_url(&self) -> String {
        let prefix =
            env::var("CURRENT_PROD").unwrap_or("https://catalog.princeton.edu".to_string());
        format!("{}{}", prefix, self.path)
    }

    pub fn old_count(&self) -> u32 {
        let body = self
            .client
            .get(self.old_url())
            .send()
            .unwrap()
            .text()
            .unwrap();
        parse_json::get_count_from_json(&body).unwrap() as u32
    }

    pub fn new_count(&self) -> u32 {
        let body = self
            .client
            .get(self.new_url())
            .send()
            .unwrap()
            .text()
            .unwrap();
        parse_json::get_count_from_json(&body).unwrap() as u32
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_comparison_can_identify_unusually_larger() {
        assert_eq!(
            RecordCountComparison::compare(14_237_145, 16_270_999),
            RecordCountComparison::UnusuallyLarger
        );
    }

    #[test]
    fn count_comparison_can_identify_larger() {
        assert_eq!(
            RecordCountComparison::compare(16_237_145, 16_270_999),
            RecordCountComparison::Larger
        );
        assert_eq!(
            RecordCountComparison::compare(6_100_282, 6_109_211),
            RecordCountComparison::Larger
        );
        assert_eq!(
            RecordCountComparison::compare(12_633_344, 12_667_545),
            RecordCountComparison::Larger
        );
        assert_eq!(
            RecordCountComparison::compare(17_596, 17_611),
            RecordCountComparison::Larger
        );
        assert_eq!(
            RecordCountComparison::compare(2_764_337, 2_783_208),
            RecordCountComparison::Larger
        );
    }

    #[test]
    fn count_comparison_can_identify_same() {
        assert_eq!(
            RecordCountComparison::compare(81_686, 81_686),
            RecordCountComparison::Same
        );
    }

    #[test]
    fn count_comparison_can_identify_smaller() {
        assert_eq!(
            RecordCountComparison::compare(1_719_106, 1_718_520),
            RecordCountComparison::Smaller
        );
    }

    #[test]
    fn count_comparison_can_identify_unusually_smaller() {
        assert_eq!(
            RecordCountComparison::compare(1_719_106, 1_218_520),
            RecordCountComparison::UnusuallySmaller
        );
    }

    #[test]
    fn search_can_create_new_url() {
        let search = Search::new("/catalog.json?f[advanced_location_s][]=scsbcul", "Columbia");
        assert_eq!(
            search.new_url(),
            "https://catalog-qa.princeton.edu/catalog.json?f[advanced_location_s][]=scsbcul"
        );
    }
}
