use yufmath::Yufmath;
use yufmath::formatter::{Formatter, LaTeXFormatter};

#[test]
fn test_sqrt_latex_formatting() {
    let mut engine = Yufmath::new();
    let formatter = LaTeXFormatter::new();
    
    let result = engine.parse("sqrt(3)").unwrap();
    let latex = formatter.format(&result);
    assert_eq!(latex, "\\sqrt{3}");
    
    let result = engine.parse("sqrt(x + 1)").unwrap();
    let latex = formatter.format(&result);
    assert_eq!(latex, "\\sqrt{x + 1}");
}

#[test]
fn test_trigonometric_functions_latex() {
    let mut engine = Yufmath::new();
    let formatter = LaTeXFormatter::new();
    
    let test_cases = vec![
        ("sin(x)", "\\sin\\left(x\\right)"),
        ("cos(x)", "\\cos\\left(x\\right)"),
        ("tan(x)", "\\tan\\left(x\\right)"),
    ];
    
    for (input, expected) in test_cases {
        let result = engine.parse(input).unwrap();
        let latex = formatter.format(&result);
        assert_eq!(latex, expected, "Failed for input: {}", input);
    }
}

#[test]
fn test_logarithmic_functions_latex() {
    let mut engine = Yufmath::new();
    let formatter = LaTeXFormatter::new();
    
    let test_cases = vec![
        ("ln(x)", "\\ln\\left(x\\right)"),
        ("log10(x)", "\\log_{10}\\left(x\\right)"),
        ("log2(x)", "\\log_2\\left(x\\right)"),
        ("exp(x)", "\\exp\\left(x\\right)"),
    ];
    
    for (input, expected) in test_cases {
        let result = engine.parse(input).unwrap();
        let latex = formatter.format(&result);
        assert_eq!(latex, expected, "Failed for input: {}", input);
    }
}

#[test]
fn test_absolute_value_latex() {
    let mut engine = Yufmath::new();
    let formatter = LaTeXFormatter::new();
    
    let result = engine.parse("abs(x)").unwrap();
    let latex = formatter.format(&result);
    assert_eq!(latex, "\\left|x\\right|");
}

#[test]
fn test_complex_expressions_latex() {
    let mut engine = Yufmath::new();
    let formatter = LaTeXFormatter::new();
    
    let test_cases = vec![
        ("2*sqrt(x+1)", "2\\sqrt{x + 1}"),
        ("sin(pi/4)", "\\sin\\left(\\frac{\\pi}{4}\\right)"),
    ];
    
    for (input, expected) in test_cases {
        let result = engine.parse(input).unwrap();
        let latex = formatter.format(&result);
        assert_eq!(latex, expected, "Failed for input: {}", input);
    }
}

#[test]
fn test_unknown_function_latex() {
    let formatter = LaTeXFormatter::new();
    
    // 测试未知函数仍然使用 \text{} 格式
    use yufmath::core::Expression;
    let result = Expression::function("unknown_func", vec![Expression::variable("x")]);
    
    let latex = formatter.format(&result);
    assert_eq!(latex, "\\text{unknown_func}\\left(x\\right)");
}