use serde::Deserialize;

#[derive(Deserialize)]
pub struct Meta {
    pages: Pages,
}

#[derive(Deserialize)]
pub struct Pages {
    total_count: i64,
}

#[derive(Deserialize)]
pub struct Response {
    meta: Meta,
}

pub fn get_count_from_json(json_data: &str) -> Result<i64, serde_json::Error> {
    let response: Response = serde_json::from_str(json_data)?;
    Ok(response.meta.pages.total_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_count_from_json() {
        let raw_json = r#"
    {
        "links": {
            "self": "https://catalog.princeton.edu/catalog.json?f%5Badvanced_location_s%5D%5B%5D=scsbcul",
            "next": "https://catalog.princeton.edu/catalog.json?f%5Badvanced_location_s%5D%5B%5D=scsbcul\u0026page=2"
        },
        "meta": {
            "pages": {
                "current_page": 1,
                "next_page": 2,
                "total_count": 3665491
            }
        }
    }
    "#;
        assert_eq!(get_count_from_json(raw_json).unwrap(), 3_665_491);
    }
}
