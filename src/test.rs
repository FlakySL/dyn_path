use serde_json::{json, Value};

use crate::dyn_access;
#[cfg(feature = "alloc")]
use crate::dyn_path;

const ERROR: &str = "nested value to exist.";

fn map() -> Value {
    json!({
        "very": {
            "nested": [
                "bunch",
                "of",
                "values"
            ],
            "or": {
                "numbers": 50
            }
        }
    })
}

#[test]
pub fn access_types() {
    let map = map();

    let _1 = dyn_access!(map.very);
    let _2 = dyn_access!(map["very"]);

    assert!(_1.is_some());
    assert!(_2.is_some());
}

#[test]
pub fn nested_access() {
    let map = map();

    let _1 = dyn_access!(map.very.nested[0])
        .expect(ERROR);
    let _2 = dyn_access!(map.very.or.numbers)
        .expect(ERROR);

    assert_eq!(_1, "bunch");
    assert_eq!(_2, 50);
}

#[test]
pub fn direct_expression() {
    let vector = vec![map()];
    let _1 = dyn_access!((vector[0]).very.nested[1])
        .expect(ERROR);

    assert_eq!(_1, "of");
}

#[cfg(feature = "alloc")]
#[test]
pub fn path_descriptor() {
    let _1 = dyn_path!(very.nested["value"].on.index[1 + 1]);

    assert_eq!(_1, r#"very.nested["value"].on.index[2]"#)
}
