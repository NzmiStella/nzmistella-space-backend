// ********************* import ********************* //
use std::fmt::Debug;

use anyhow::{anyhow, Result};
use serde::Serialize;

// ********************* content ********************* //
#[derive(Debug, Serialize)]
pub struct Page<T: Debug + Serialize> {
    /// 当前页码，自1起计数
    #[serde(rename = "pageNum")]
    pub page_num: u64,
    /// 分页大小
    #[serde(rename = "pageSize")]
    pub page_size: u64,
    /// 总记录数
    #[serde(rename = "recordTotal")]
    pub record_total: u64,
    /// 总页数
    #[serde(rename = "pageTotal")]
    pub page_total: u64,
    /// 分页记录
    pub records: Vec<T>,
}

impl<T: Debug + Serialize> Page<T> {
    pub fn new(page_num: u64, page_size: u64, record_total: u64, records: Vec<T>) -> Result<Self> {
        if page_size == 0 {
            return Err(anyhow!("Page size must be greater than zero"));
        }
        if record_total == 0 {
            return Ok(Self {
                page_num: 1,
                page_size,
                record_total,
                page_total: 1,
                records: Vec::new(),
            });
        }
        if page_size < records.len() as u64 {
            return Err(anyhow!("Number of records exceeds the specified page size"));
        }
        let page_total = (record_total + page_size - 1) / page_size;
        if page_num < 1 || page_num > page_total {
            return Err(anyhow!(
                "Invalid page number: {}. It must be between 1 and page_total {}",
                page_num,
                page_total
            ));
        }
        Ok(Self {
            page_num,
            page_size,
            record_total,
            page_total,
            records,
        })
    }
    pub fn has_prev(&self) -> bool {
        self.page_num > 1
    }
    pub fn has_next(&self) -> bool {
        self.page_num < self.page_total
    }
    pub fn first_page(&self) -> u64 {
        1
    }
    pub fn last_page(&self) -> u64 {
        self.page_total
    }
    pub fn is_active(&self, page: &u64) -> bool {
        self.page_num == *page
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paginate_valid() {
        let records = vec![1, 2, 3];
        let page = Page::new(1, 3, 10, records.clone()).unwrap();
        assert_eq!(page.page_num, 1);
        assert_eq!(page.page_size, 3);
        assert_eq!(page.record_total, 10);
        assert_eq!(page.page_total, 4); // (10 + 3 - 1) / 3
        assert_eq!(page.records, records);
    }

    #[test]
    fn test_paginate_with_zero_page_size() {
        let records = vec![1, 2, 3];
        let result = Page::new(1, 0, 10, records);
        assert!(result.is_err());
    }

    #[test]
    fn test_paginate_with_excess_records() {
        let records = vec![1, 2, 3, 4];
        let result = Page::new(1, 3, 10, records);
        assert!(result.is_err());
    }

    #[test]
    fn test_paginate_with_invalid_page_number() {
        let records = vec![1, 2, 3];
        let result = Page::new(5, 3, 10, records); // page_num should be between 1 and 4
        assert!(result.is_err());
    }

    #[test]
    fn test_paginate_navigation_methods() {
        let records = vec![1, 2, 3];
        let page = Page::new(2, 3, 10, records.clone()).unwrap();
        assert!(page.has_prev());
        assert!(page.has_next());
        assert_eq!(page.first_page(), 1);
        assert_eq!(page.last_page(), 4);
        assert!(!page.is_active(&1));
        assert!(page.is_active(&2));
    }

    #[test]
    fn test_paginate_empty_records() {
        let records: Vec<i32> = Vec::new();
        let page = Page::new(1, 3, 0, records).unwrap();
        assert_eq!(page.page_num, 1);
        assert_eq!(page.page_size, 3);
        assert_eq!(page.record_total, 0);
        assert_eq!(page.page_total, 1);
        assert!(page.records.is_empty());
        assert!(!page.has_prev());
        assert!(!page.has_next());
        assert!(page.is_active(&1));
    }
}
