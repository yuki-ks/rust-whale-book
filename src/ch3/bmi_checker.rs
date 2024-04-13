fn main() {
    let body = Body::new(163.0, 75.2, "田中");
    body.print_result();
    let body = Body::new(158.2, 55.0, "鈴木");
    body.print_result();
    let body = Body::new(174.2, 54.2, "井上");
    body.print_result();
}

struct BmiRange {
    min: f64,
    max: f64,
    label: String,
}

impl BmiRange {
    fn new(min: f64, max: f64, label: &str) -> Self {
        BmiRange {
            min,
            max,
            label: label.to_string(),
        }
    }

    fn test(&self, v: f64) -> bool {
        (self.min <= v) && (v < self.max)
    }
}

struct Body {
    height: f64,
    weight: f64,
    name: String,
}

impl Body {
    fn new(height: f64, weight: f64, name: &str) -> Self {
        Body {
            height,
            weight,
            name: name.to_string(),
        }
    }

    fn calc_bmi(&self) -> f64 {
        self.weight / (self.height / 100.0).powf(2.0)
    }

    fn print_result(&self) {
        let bmi = self.calc_bmi();
        let bmi_list = [
            BmiRange::new(0.0, 18.5, "低体重"),
            BmiRange::new(18.5, 25.0, "普通体重"),
            BmiRange::new(25.0, 30.0, "肥満1度"),
            BmiRange::new(30.0, 35.0, "肥満2度"),
            BmiRange::new(35.0, 40.0, "肥満3度"),
            BmiRange::new(45.0, 99.0, "肥満4度"),
        ];

        let mut result = String::from("不明");
        for range in bmi_list {
            if range.test(bmi) {
                result = range.label.clone();
                break;
            }
        }
        println!("{}さん, BMI={:.1}, 判定={}", self.name, bmi, result);
    }
}
