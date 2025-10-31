use citrine::{read_str, eval_str, standard_env};
use citrine::reader::Value;

#[test]
fn test_read_number() {
    let value = read_str("42").unwrap();
    assert_eq!(value, Value::Number(42.0));
}

#[test]
fn test_read_string() {
    let value = read_str("\"hello\"").unwrap();
    assert_eq!(value, Value::String("hello".to_string()));
}

#[test]
fn test_read_symbol() {
    let value = read_str("foo").unwrap();
    assert_eq!(value, Value::Symbol("foo".to_string()));
}

#[test]
fn test_read_keyword() {
    let value = read_str(":foo").unwrap();
    assert_eq!(value, Value::Keyword("foo".to_string()));
}

#[test]
fn test_read_list() {
    let value = read_str("(1 2 3)").unwrap();
    assert_eq!(
        value,
        Value::List(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0)
        ])
    );
}

#[test]
fn test_read_vector() {
    let value = read_str("[1 2 3]").unwrap();
    assert_eq!(
        value,
        Value::Vector(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0)
        ])
    );
}

#[test]
fn test_read_map() {
    let value = read_str("{:a 1 :b 2}").unwrap();
    
    if let Value::Map(map) = value {
        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&Value::Keyword("a".to_string())), Some(&Value::Number(1.0)));
        assert_eq!(map.get(&Value::Keyword("b".to_string())), Some(&Value::Number(2.0)));
    } else {
        panic!("Expected a map");
    }
}

#[test]
fn test_read_set() {
    let value = read_str("#{1 2 3}").unwrap();
    
    if let Value::Set(set) = value {
        assert_eq!(set.len(), 3);
        assert!(set.contains(&Value::Number(1.0)));
        assert!(set.contains(&Value::Number(2.0)));
        assert!(set.contains(&Value::Number(3.0)));
    } else {
        panic!("Expected a set");
    }
}

#[test]
fn test_read_quote() {
    let value = read_str("'foo").unwrap();
    assert_eq!(
        value,
        Value::List(vec![
            Value::Symbol("quote".to_string()),
            Value::Symbol("foo".to_string())
        ])
    );
}

#[test]
fn test_read_backtick() {
    let value = read_str("`(1 2 ,x)").unwrap();
    
    if let Value::List(items) = value {
        assert_eq!(items[0], Value::Symbol("quasiquote".to_string()));
        
        if let Value::List(inner) = &items[1] {
            assert_eq!(inner[0], Value::Number(1.0));
            assert_eq!(inner[1], Value::Number(2.0));
            
            if let Value::List(unquote) = &inner[2] {
                assert_eq!(unquote[0], Value::Symbol("unquote".to_string()));
                assert_eq!(unquote[1], Value::Symbol("x".to_string()));
            } else {
                panic!("Expected an unquote list");
            }
        } else {
            panic!("Expected a list");
        }
    } else {
        panic!("Expected a list");
    }
}

#[test]
fn test_eval_number() {
    let env = standard_env();
    let result = eval_str("42", &env).unwrap();
    assert_eq!(result, Value::Number(42.0));
}

#[test]
fn test_eval_string() {
    let env = standard_env();
    let result = eval_str("\"hello\"", &env).unwrap();
    assert_eq!(result, Value::String("hello".to_string()));
}

#[test]
fn test_eval_symbol() {
    let env = standard_env();
    env.borrow_mut().set("x".to_string(), Value::Number(42.0));
    
    let result = eval_str("x", &env).unwrap();
    assert_eq!(result, Value::Number(42.0));
}

#[test]
fn test_eval_setq() {
    let env = standard_env();
    let result = eval_str("(setq x 42)", &env).unwrap();
    assert_eq!(result, Value::Number(42.0));
    assert_eq!(env.borrow().get("x"), Some(Value::Number(42.0)));
}

#[test]
fn test_eval_fn() {
    let env = standard_env();
    let result = eval_str("(fn [x] (+ x 1))", &env).unwrap();
    
    if let Value::Function(f) = result {
        assert_eq!(f.params, vec!["x".to_string()]);
        assert_eq!(f.body.len(), 1);
    } else {
        panic!("Expected a function");
    }
}

#[test]
fn test_eval_macro() {
    let env = standard_env();
    let result = eval_str("(macro [x] (quote x))", &env).unwrap();
    
    if let Value::Macro(m) = result {
        assert_eq!(m.params, vec!["x".to_string()]);
        assert_eq!(m.body.len(), 1);
    } else {
        panic!("Expected a macro");
    }
}

#[test]
fn test_eval_vector() {
    let env = standard_env();
    env.borrow_mut().set("x".to_string(), Value::Number(42.0));
    
    let result = eval_str("[1 x 3]", &env).unwrap();
    assert_eq!(
        result,
        Value::Vector(vec![
            Value::Number(1.0),
            Value::Number(42.0),
            Value::Number(3.0)
        ])
    );
}

#[test]
fn test_eval_map() {
    let env = standard_env();
    env.borrow_mut().set("x".to_string(), Value::Number(42.0));
    
    let result = eval_str("{:a 1 :b x}", &env).unwrap();
    
    if let Value::Map(result_map) = result {
        assert_eq!(result_map.len(), 2);
        assert_eq!(result_map.get(&Value::Keyword("a".to_string())), Some(&Value::Number(1.0)));
        assert_eq!(result_map.get(&Value::Keyword("b".to_string())), Some(&Value::Number(42.0)));
    } else {
        panic!("Expected a map");
    }
}

#[test]
fn test_eval_set() {
    let env = standard_env();
    env.borrow_mut().set("x".to_string(), Value::Number(42.0));
    
    let result = eval_str("#{1 x}", &env).unwrap();
    
    if let Value::Set(result_set) = result {
        assert_eq!(result_set.len(), 2);
        assert!(result_set.contains(&Value::Number(1.0)));
        assert!(result_set.contains(&Value::Number(42.0)));
    } else {
        panic!("Expected a set");
    }
}

