use serde_json::json;

static GET_JSON: &str = r#"
{
    "FragmentTask": {
        "id": {
            "offset": 0,
            "count": 8
        },
        "fractal": {
            "Julia": {
                "c": {
                    "re": 0.0,
                    "im": 0.1
                },
                "divergence_threshold_square": 0.0
            }
        },
        "max_iteration": 0,
        "resolution": {
            "nx": 160,
            "ny": 120
        },
        "range": {
            "min": {
                "x": 0.0,
                "y": 0.0
            },
            "max": {
                "x": 1.0,
                "y": 1.0
            }
        }
    }
}
"#;

fn main() {
    println!("{}", GET_JSON);
}