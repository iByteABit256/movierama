use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sort {
    pub empty: bool,
    pub sorted: bool,
    pub unsorted: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub orders: Vec<SortOrder>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SortOrder {
    pub property: String,
    pub direction: String, // "asc" or "desc"
}

impl Sort {
    pub fn from_query(sort_param: &str) -> Self {
        let parts: Vec<&str> = sort_param.split(',').map(|s| s.trim()).collect();
        let property: &str = parts.first().map_or("dateAdded", |v| v);
        let direction: &str = parts.get(1).map_or("desc", |v| v);
        let order = SortOrder {
            property: property.to_string(),
            direction: direction.to_uppercase(),
        };

        let default_order = SortOrder {
            property: "dateAdded".to_owned(),
            direction: "desc".to_owned(),
        };

        let mut orders = vec![order.clone()];
        if order.property != "dateAdded" {
            orders.push(default_order);
        }

        Sort {
            empty: false,
            sorted: true,
            unsorted: false,
            orders,
        }
    }

    pub fn to_sql(&self, default_field: &str) -> String {
        if self.orders.is_empty() {
            return format!("{} DESC", default_field);
        }

        let mut field_map = std::collections::HashMap::new();
        field_map.insert("title", "m.title");
        field_map.insert("dateAdded", "m.date_added");
        field_map.insert("likeCount", "like_count");
        field_map.insert("hateCount", "hate_count");
        field_map.insert("username", "u.username");

        let parts: Vec<String> = self
            .orders
            .iter()
            .filter_map(|o| {
                let db_field = field_map.get(o.property.as_str())?;
                let dir = if o.direction.eq_ignore_ascii_case("desc") {
                    "DESC"
                } else {
                    "ASC"
                };
                Some(format!("{} {}", db_field, dir))
            })
            .collect();

        if parts.is_empty() {
            format!("{} DESC", default_field)
        } else {
            parts.join(", ")
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pageable {
    #[serde(rename = "pageNumber")]
    pub page_number: u32,
    #[serde(rename = "pageSize")]
    pub page_size: u32,
    pub sort: Sort,
    pub offset: u64,
    pub paged: bool,
    pub unpaged: bool,
}

impl Pageable {
    pub fn new(page_number: u32, page_size: u32, sort: Sort) -> Self {
        Pageable {
            page_number,
            page_size,
            offset: (page_number * page_size) as u64,
            paged: true,
            unpaged: false,
            sort,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Page<T> {
    pub content: Vec<T>,
    pub pageable: Pageable,
    #[serde(rename = "totalPages")]
    pub total_pages: u32,
    #[serde(rename = "totalElements")]
    pub total_elements: u64,
    pub last: bool,
    pub size: u32,
    pub number: u32,
    pub sort: Sort,
    pub first: bool,
    #[serde(rename = "numberOfElements")]
    pub number_of_elements: u32,
    pub empty: bool,
}

impl<T> Page<T> {
    pub fn new(content: Vec<T>, pageable: Pageable, total_elements: u64) -> Self {
        let number_of_elements = content.len() as u32;
        let total_pages = ((total_elements as f64) / (pageable.page_size as f64)).ceil() as u32;
        let last = pageable.page_number + 1 >= total_pages;

        Page {
            content,
            pageable: pageable.clone(),
            total_pages,
            total_elements,
            last,
            size: pageable.page_size,
            number: pageable.page_number,
            sort: pageable.sort.clone(),
            first: pageable.page_number == 0,
            number_of_elements,
            empty: number_of_elements == 0,
        }
    }
}
