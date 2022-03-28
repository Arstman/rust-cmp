fn main() {
    let test = "if age1 >= 45".to_string();

    let mut result: Vec<DFState> = Vec::new();

    let mut dfstate = DFState {
        state: State::Initial,
        text: String::new(),
    };

    for c in test.chars() {
        match char_type(c) {
            TokenType::Identifier => {
                let current = State::Id;
                auto_state(c, current, &mut dfstate, &mut result)
            }
            TokenType::IntLiteral => {
                let current = State::Int;
                auto_state(c, current, &mut dfstate, &mut result)
            }
            TokenType::Punctuation => {
                let current = State::Pct;
                auto_state(c, current, &mut dfstate, &mut result)
            }
            TokenType::WhiteSpace => {
                let current = State::WhiteSpace;

                auto_state(c, current, &mut dfstate, &mut result)
            }
            TokenType::UnDefinte => {
                let current = State::UnDefinte;
                auto_state(c, current, &mut dfstate, &mut result)
            }
        }
    }

    result.push(dfstate);

    println!("{:?}", result);
}

pub fn auto_state(c: char, current: State, dfstate: &mut DFState, stack: &mut Vec<DFState>) {
    if dfstate.state == State::Id && current == State::Int {
        dfstate.text.push(c);
        return;
    }
    if dfstate.state != current && dfstate.state != State::Initial {
        if dfstate.state == State::Id && check_keywords(&dfstate.text) {
            dfstate.state = State::KeyWord;
        }

        
        stack.push(dfstate.clone());
        dfstate.text = String::new();
    }
    dfstate.text.push(c);
    dfstate.state = current;
}

pub fn check_keywords(s: &str) -> bool {
    let keywords = ["if", "Int", "else"];
    keywords.contains(&s)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Initial,
    Id,
    Int,
    Pct,
    WhiteSpace,
    UnDefinte,
    KeyWord,
}
#[derive(Debug)]
pub enum TokenType {
    Identifier,
    IntLiteral,
    Punctuation,
    WhiteSpace,
    UnDefinte,
}

pub fn char_type(c: char) -> TokenType {
    if c.is_digit(10) {
        return TokenType::IntLiteral;
    }

    if c.is_whitespace() {
        return TokenType::WhiteSpace;
    }

    if c.is_ascii_alphabetic() {
        return TokenType::Identifier;
    }

    if c.is_ascii_punctuation() {
        return TokenType::Punctuation;
    }

    TokenType::UnDefinte
}

#[derive(Debug, Clone)]
pub struct DFState {
    state: State,
    text: String,
}
