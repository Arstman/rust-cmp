fn main() {
    println!("Hello, world!");

    let test = "age >= 45".to_string();

    let mut result: Vec<DFState> = Vec::new();

    let mut dfstate = DFState {
        state: State::Initial,
        text: String::new(),
    };

    for c in test.chars() {
        match char_type(c) {
            TokenType::Identifier => {
                if dfstate.state == State::Initial || dfstate.state == State::Id {
                    dfstate.text.push(c);
                } else {
                     result.push(dfstate.clone());
                     dfstate.text = String::new();
                     dfstate.text.push(c);
                }

                    dfstate.state = State::Id;

            }
            TokenType::IntLiteral => {
                if dfstate.state == State::Initial || dfstate.state == State::Int {
                    dfstate.text.push(c);
                } else {
                     result.push(dfstate.clone());
                     dfstate.text = String::new();
                     dfstate.text.push(c);
                }

                    dfstate.state = State::Int;
            }
            TokenType::Punctuation => {
                if dfstate.state == State::Initial || dfstate.state == State::Pct {
                    dfstate.text.push(c);
                } else {
                     result.push(dfstate.clone());
                     dfstate.text = String::new();
                     dfstate.text.push(c);
                }

                    dfstate.state = State::Pct;
            }
            TokenType::WhiteSpace => {
                if dfstate.state == State::Initial || dfstate.state == State::WhiteSpace {
                    dfstate.text.push(c);
                } else {
                     result.push(dfstate.clone());
                     dfstate.text = String::new();
                     dfstate.text.push(c);
                }

                    dfstate.state = State::WhiteSpace;

            }
            TokenType::UnDefinte => {
              if dfstate.state == State::Initial || dfstate.state == State::UnDefinte {
                    dfstate.text.push(c);
                } else {
                     result.push(dfstate.clone());
                     dfstate.text = String::new();
                     dfstate.text.push(c);
                }

                    dfstate.state = State::UnDefinte;
            }
        }
    }

    result.push(dfstate);

    println!("{:?}", result);
}

pub fn auto_state(c: char, current: State, dfstate: &mut DFState, stack: &mut Vec<DFState> ) {
    if dfstate.state != current && dfstate.state != State::Initial {
        stack.push(dfstate.clone());
        dfstate.text = String::new();
    }
    dfstate.text.push(c);
    dfstate.state = current;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Initial,
    Id,
    Int,
    Pct,
    WhiteSpace,
    UnDefinte,
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
