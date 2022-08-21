pub fn mc_format_float(s: &str) -> f64 {
    if s == "NaN" {
        0_f64
    } else if s.starts_with('<') {
        0.555_f64
    } else if s.starts_with('>') {
        666.666_f64
    } else {
        let r: f64 = s.parse().unwrap();
        let a = format!("{:.3}", r);
        a.parse::<f64>().unwrap()
    }
}

pub fn mc_format_test_id(test_id: &str) -> String {
    let l = test_id.len();
    if l == 1 {
        return "00".to_string() + test_id;
    }
    if l == 2 {
        "0".to_string() + test_id
    } else {
        test_id.to_string()
    }
}
