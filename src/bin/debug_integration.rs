use bccl::{Lexer, Parser, Evaluator};

fn test_step_by_step(name: &str, steps: Vec<&str>) {
    println!("\n=== {} ===", name);
    let mut evaluator = Evaluator::new();
    
    for (i, step) in steps.iter().enumerate() {
        println!("Step {}: {}", i + 1, step);
        
        let mut lexer = Lexer::new(step);
        let tokens = match lexer.tokenize() {
            Ok(tokens) => tokens,
            Err(e) => {
                println!("  ❌ Lexer error: {:?}", e);
                continue;
            }
        };
        
        let mut parser = Parser::new(tokens);
        let program = match parser.parse() {
            Ok(program) => program,
            Err(e) => {
                println!("  ❌ Parser error: {:?}", e);
                continue;
            }
        };
        
        match evaluator.evaluate_program(&program) {
            Ok(Some(value)) => {
                println!("  → {}", value.display());
            }
            Ok(None) => {
                println!("  → (no result)");
            }
            Err(e) => {
                println!("  ❌ Runtime error: {:?}", e);
            }
        }
    }
}

fn main() {
    println!("BCCL Integration Test Debugging");
    println!("===============================");
    
    // Debug inventory management system
    test_step_by_step("Inventory Management Debug", vec![
        r#"inventory = {"items": [{"name": "sword", "quantity": 5, "price": 100}, {"name": "shield", "quantity": 3, "price": 75}, {"name": "potion", "quantity": 12, "price": 25}], "currency": 500}"#,
        r#"sword = inventory["items"][0]"#,
        r#"shield = inventory["items"][1]"#,
        r#"sword["quantity"]"#,
        r#"sword["price"]"#,
        r#"sword_cost = sword["quantity"] * sword["price"]"#,
        r#"shield["quantity"]"#,
        r#"shield["price"]"#,
        r#"shield_cost = shield["quantity"] * shield["price"]"#,
        r#"total_cost = sword_cost + shield_cost"#,
        r#"inventory["currency"]"#,
        r#"can_afford = total_cost <= inventory["currency"]"#,
        r#"has_sword = sword["quantity"] > 0"#,
        r#"has_shield = shield["quantity"] > 0"#,
        r#"purchase_possible = can_afford and has_sword and has_shield"#,
    ]);
    
    // Debug complex boolean logic
    test_step_by_step("Complex Boolean Logic Debug", vec![
        r#"values = [45, 67, 23, 89, 12]"#,
        r#"thresholds = {"min": 20, "max": 80}"#,
        r#"first_val = values[0]"#,
        r#"last_val = values[4]"#,
        r#"values[3]"#,
        r#"highest = max(first_val, values[3])"#,
        r#"thresholds["min"]"#,
        r#"thresholds["max"]"#,
        r#"first_val >= thresholds["min"]"#,
        r#"first_val <= thresholds["max"]"#,
        r#"within_range = first_val >= thresholds["min"] and first_val <= thresholds["max"]"#,
        r#"highest > thresholds["max"]"#,
        r#"exceeds_max = highest > thresholds["max"]"#,
        r#"last_val >= thresholds["min"]"#,
        r#"valid_minimum = last_val >= thresholds["min"]"#,
        r#"final_check = within_range and exceeds_max and valid_minimum"#,
    ]);
    
    // Debug data validation scenario
    test_step_by_step("Data Validation Debug", vec![
        r#"records = [{"id": 1, "valid": true}, {"id": 2, "valid": false}, {"id": 3, "valid": true}]"#,
        r#"records[0]"#,
        r#"records[1]"#,
        r#"records[2]"#,
    ]);
}