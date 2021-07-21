struct ModelNumber {
    product_code: String,
    branch: String,
    lot: String,
}

impl ModelNumber {
    fn new(product_code: String, branch: String, lot: String) -> Self {
        if product_code.is_empty() {
            panic!("product_code is empty")
        }
        if branch.is_empty() {
            panic!("branch is empty")
        }
        if lot.is_empty() {
            panic!("lot is empty")
        }
        Self {
            product_code,
            branch,
            lot,
        }
    }
}

impl ToString for ModelNumber {
    fn to_string(&self) -> String {
        format!("{}-{}-{}", self.product_code, self.branch, self.lot)
    }
}

#[test]
fn test_to_string_model_number() {
    let mn = ModelNumber::new("a20421".to_string(), "100".to_string(), "1".to_string());
    assert_eq!("a20421-100-1".to_string(), mn.to_string());
}
