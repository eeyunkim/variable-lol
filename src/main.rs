use std::collections::HashMap;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref VARIABLES: Mutex<HashMap<&'static str, String>> = Mutex::new(HashMap::new());
} // Mutex로 묶어서 안전성 확보

#[derive(Debug)]
enum Token {
    Variable(String),
    String(String),
    OpenBracket,
    CloseBracket,
    OpenSquareBracket,
    CloseSquareBracket,
} /* 토큰 종류 정의 */

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current_word = String::new();
    let mut inside_variable = false;

    for c in input.chars() {
        match c {
            '{' => {
                if !current_word.is_empty() {
                    tokens.push(Token::String(current_word.clone()));
                    current_word.clear();
                }
                inside_variable = true;
            }
            '}' => {
                if !current_word.is_empty() {
                    tokens.push(if inside_variable {
                        Token::Variable(current_word.clone())
                    } else {
                        Token::String(current_word.clone())
                    });
                    current_word.clear();
                }
                inside_variable = false;
                tokens.push(Token::CloseBracket);
            }
            '[' => {
                if !current_word.is_empty() {
                    tokens.push(Token::String(current_word.clone()));
                    current_word.clear();
                }
                tokens.push(Token::OpenSquareBracket);
            }
            ']' => {
                if !current_word.is_empty() {
                    tokens.push(Token::String(current_word.clone()));
                    current_word.clear();
                }
                tokens.push(Token::CloseSquareBracket);
            }
            _ => {
                current_word.push(c);
            }
        }
    }

    if !current_word.is_empty() {
        tokens.push(if inside_variable {
            Token::Variable(current_word.clone())
        } else {
            Token::String(current_word.clone())
        });
    }

    tokens
} /* 토큰 종류 정의 */

fn evaluate_tokens(tokens: &[Token], variables: &HashMap<&str, String>) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_expression = Vec::new();
    let mut current_array = Vec::new();
    let mut array_index = None;

    for token in tokens {
        match token {
            Token::Variable(var_name) => {
                if var_name.ends_with("]") && var_name.contains("[") { // 배열 and 벡터 판단 
                    let parts: Vec<&str> = var_name.split("[").collect();
                    if let Some(array_name) = parts.get(0) {
                        if let Some(index_str) = parts.get(1) {
                            let index = index_str.trim_end_matches("]").parse::<usize>();
                            if let Ok(index) = index {
                                array_index = Some(index);
                                current_expression.push(array_name.to_string());
                            }
                        }
                    }
                } else {
                    current_expression.push(var_name.clone());
                }
            }
            Token::String(s) => {
                current_expression.push(s.clone());
            }
            Token::OpenBracket => {
                result.push(current_expression.join(""));
                current_expression.clear();
            }
            Token::CloseBracket => {
                if !current_expression.is_empty() {
                    result.push(current_expression.join(""));
                    current_expression.clear();
                }
            }
            Token::OpenSquareBracket => {
                array_index = None;
                current_expression.push("[".to_string());
            }
            Token::CloseSquareBracket => {
                if let Some(index) = array_index {
                    current_array.push(index);
                }
                current_expression.push("]".to_string());
                array_index = None;
            }
        }
    }

    if !current_expression.is_empty() {
        result.push(current_expression.join(""));
    }

    if !current_array.is_empty() {
        // 배열 벡터 표현식 추가 즐
        result.push(format!(
            "[{}]",
            current_array
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(",")
        ));
    }

    result
} /* 토큰 리뷰 후 println! 생성 */

macro_rules! printv {
    ($($arg:tt)*) => {
        let formatted_str = format!($($arg)*); // input 문자열 토큰화
        let tokens = tokenize(&formatted_str);
        
        let variables = &*VARIABLES.lock().unwrap(); // 전역 변수 mut lock 가져와서 unwrap
        
        let processed_str = evaluate_tokens(&tokens, variables);
        println!("{}", processed_str.join(""));
    };
} /* 입력된 문자열 토큰화 후 결과 리뷰 */

fn main() {
    let a = 3;
    let b = vec![1, 2, 3];
    let c = 5;
    let sans = "sansdgsdgs";

    printv!("{a} is {c} {sans}");
}
