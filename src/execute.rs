use std::collections::HashMap;

use colored::Colorize;

use crate::parser::{self, ParsedTokens};


pub fn execute(tokens: &Vec<parser::ParsedTokens>) -> Result<(), String> {

    let mut vars: HashMap<String, f64> = HashMap::new();
    let mut last_val = 0 as f64;

    for (index, token) in tokens.iter().enumerate() {
        let last_token = tokens.get(if index == 0 { 1 } else { index } - 1);
        let next_token = tokens.get(if index == tokens.len() - 1 { tokens.len() - 1} else { index } + 1);

        match token {
            ParsedTokens::Add => {
                if let None = last_token {
                    return Err("Cannot get last token".to_string());
                }
        
                if let None = next_token {
                    return Err("Cannot get next token".to_string());
                }

                let last = last_token.unwrap();
                let next = next_token.unwrap();

                let last_v;
                let next_v;

                match last {
                    ParsedTokens::Number(val) => last_v = val,
                    ParsedTokens::Var(var) => {
                        match vars.get(var) {
                            Some(val) => {
                                last_v = val;
                            },
                            None => {
                                return Err(format!("{}: The varible {} does not have a value", "syntax error".red(), var));
                            }
                        }
                    },
                    _ => return Err(format!("{}: Last token is not an var or an number", "syntax error".red())),
                }

                match next {
                    ParsedTokens::Number(val) => next_v = val,
                    ParsedTokens::Var(var) => {
                        match vars.get(var) {
                            Some(val) => {
                                next_v = val;
                            },
                            None => {
                                return Err(format!("{}: The varible {} does not have a value", "syntax error".red(), var));
                            }
                        }
                    },
                    _ => return Err(format!("{}: Next token is not an var or an number", "syntax error".red())),
                }

                last_val = last_v + next_v;

            },
            ParsedTokens::Subtract => {
                if let None = last_token {
                    return Err("Cannot get last token".to_string());
                }
        
                if let None = next_token {
                    return Err("Cannot get next token".to_string());
                }

                let last = last_token.unwrap();
                let next = next_token.unwrap();

                let last_v;
                let next_v;

                match last {
                    ParsedTokens::Number(val) => last_v = val,
                    ParsedTokens::Var(var) => {
                        match vars.get(var) {
                            Some(val) => {
                                last_v = val;
                            },
                            None => {
                                return Err(format!("{}: The varible {} does not have a value", "syntax error".red(), var));
                            }
                        }
                    },
                    _ => return Err(format!("{}: Last token is not an var or an number", "syntax error".red())),
                }

                match next {
                    ParsedTokens::Number(val) => next_v = val,
                    ParsedTokens::Var(var) => {
                        match vars.get(var) {
                            Some(val) => {
                                next_v = val;
                            },
                            None => {
                                return Err(format!("{}: The varible {} does not have a value", "syntax error".red(), var));
                            }
                        }
                    },
                    _ => return Err(format!("{}: Next token is not an var or an number", "syntax error".red())),
                }

                last_val = last_v - next_v;

            },
            ParsedTokens::Multiplie => {
                if let None = last_token {
                    return Err("Cannot get last token".to_string());
                }
        
                if let None = next_token {
                    return Err("Cannot get next token".to_string());
                }

                let last = last_token.unwrap();
                let next = next_token.unwrap();

                let last_v;
                let next_v;

                match last {
                    ParsedTokens::Number(val) => last_v = val,
                    ParsedTokens::Var(var) => {
                        match vars.get(var) {
                            Some(val) => {
                                last_v = val;
                            },
                            None => {
                                return Err(format!("{}: The varible {} does not have a value", "syntax error".red(), var));
                            }
                        }
                    },
                    _ => return Err(format!("{}: Last token is not an var or an number", "syntax error".red())),
                }

                match next {
                    ParsedTokens::Number(val) => next_v = val,
                    ParsedTokens::Var(var) => {
                        match vars.get(var) {
                            Some(val) => {
                                next_v = val;
                            },
                            None => {
                                return Err(format!("{}: The varible {} does not have a value", "syntax error".red(), var));
                            }
                        }
                    },
                    _ => return Err(format!("{}: Next token is not an var or an number", "syntax error".red())),
                }

                last_val = last_v * next_v;

            },
            ParsedTokens::Divide => {
                if let None = last_token {
                    return Err("Cannot get last token".to_string());
                }
        
                if let None = next_token {
                    return Err("Cannot get next token".to_string());
                }

                let last = last_token.unwrap();
                let next = next_token.unwrap();

                let last_v;
                let next_v;

                match last {
                    ParsedTokens::Number(val) => last_v = val,
                    ParsedTokens::Var(var) => {
                        match vars.get(var) {
                            Some(val) => {
                                last_v = val;
                            },
                            None => {
                                return Err(format!("{}: The varible {} does not have a value", "syntax error".red(), var));
                            }
                        }
                    },
                    _ => return Err(format!("{}: Last token is not an var or an number", "syntax error".red())),
                }

                match next {
                    ParsedTokens::Number(val) => next_v = val,
                    ParsedTokens::Var(var) => {
                        match vars.get(var) {
                            Some(val) => {
                                next_v = val;
                            },
                            None => {
                                return Err(format!("{}: The varible {} does not have a value", "syntax error".red(), var));
                            }
                        }
                    },
                    _ => return Err(format!("{}: Next token is not an var or an number", "syntax error".red())),
                }

                last_val = last_v / next_v;

            },
            ParsedTokens::Modulo => {
                if let None = last_token {
                    return Err("Cannot get last token".to_string());
                }
        
                if let None = next_token {
                    return Err("Cannot get next token".to_string());
                }

                let last = last_token.unwrap();
                let next = next_token.unwrap();

                let last_v;
                let next_v;

                match last {
                    ParsedTokens::Number(val) => last_v = val,
                    ParsedTokens::Var(var) => {
                        match vars.get(var) {
                            Some(val) => {
                                last_v = val;
                            },
                            None => {
                                return Err(format!("{}: The varible {} does not have a value", "syntax error".red(), var));
                            }
                        }
                    },
                    _ => return Err(format!("{}: Last token is not an var or an number", "syntax error".red())),
                }

                match next {
                    ParsedTokens::Number(val) => next_v = val,
                    ParsedTokens::Var(var) => {
                        match vars.get(var) {
                            Some(val) => {
                                next_v = val;
                            },
                            None => {
                                return Err(format!("{}: The varible {} does not have a value", "syntax error".red(), var));
                            }
                        }
                    },
                    _ => return Err(format!("{}: Next token is not an var or an number", "syntax error".red())),
                }

                last_val = last_v % next_v;

            },
            ParsedTokens::Power => {
                if let None = last_token {
                    return Err("Cannot get last token".to_string());
                }
        
                if let None = next_token {
                    return Err("Cannot get next token".to_string());
                }

                let last = last_token.unwrap();
                let next = next_token.unwrap();

                let last_v;
                let next_v;

                match last {
                    ParsedTokens::Number(val) => last_v = val,
                    ParsedTokens::Var(var) => {
                        match vars.get(var) {
                            Some(val) => {
                                last_v = val;
                            },
                            None => {
                                return Err(format!("{}: The varible {} does not have a value", "syntax error".red(), var));
                            }
                        }
                    },
                    _ => return Err(format!("{}: Last token is not an var or an number", "syntax error".red())),
                }

                match next {
                    ParsedTokens::Number(val) => next_v = val,
                    ParsedTokens::Var(var) => {
                        match vars.get(var) {
                            Some(val) => {
                                next_v = val;
                            },
                            None => {
                                return Err(format!("{}: The varible {} does not have a value", "syntax error".red(), var));
                            }
                        }
                    },
                    _ => return Err(format!("{}: Next token is not an var or an number", "syntax error".red())),
                }

                last_val = last_v.powf(*next_v);

            },
            ParsedTokens::Equal => {
                if let None = last_token {
                    return Err("Cannot get last token".to_string());
                }
        
                if let None = next_token {
                    return Err("Cannot get next token".to_string());
                }

                let last = last_token.unwrap();
                let next = next_token.unwrap();

                match next {
                    ParsedTokens::Var(var) => {
                        vars.insert(var.clone(), last_val);
                    },
                    ParsedTokens::Number(n) => {
                        if let ParsedTokens::Var(var) = last {
                            vars.insert(var.clone(), n.clone());
                        }
                    },
                    _ => return Err(format!("{}: Next token is not an var or an number", "syntax error".red())),
                }

            },
            ParsedTokens::Print => {
                if let None = next_token {
                    return Err("Cannot get next token".to_string());
                }

                let next = next_token.unwrap();

                match next {
                    ParsedTokens::Var(var) => {
                        match vars.get(var) {
                            Some(val) => {
                                println!("{} = {}", var, val);
                            },
                            None => {
                                return Err(format!("{}: The varible {} does not have a value", "syntax error".red(), var));
                            }
                        }
                    },
                    _ => return Err(format!("{}: Next token is not an var", "syntax error".red())),

                }
            },
            ParsedTokens::Copy => {
                if let None = next_token {
                    return Err("Cannot get next token".to_string());
                }

                let next = next_token.unwrap();

                let next_two_token = tokens.get(if index == tokens.len() - 2 { tokens.len() - 1} else { index } + 2);

                if let None = next_two_token {
                    return Err("Cannot get next token".to_string());
                }

                let next_two = next_two_token.unwrap();

                match next {
                    ParsedTokens::Var(from) => {
                        match next_two {
                            ParsedTokens::Var(to) => {
                                vars.insert(to.clone(), match vars.get(from) {
                                    Some(var) => var,
                                    None => {
                                        return Err(format!("{}: The varible {} does not have a value", "syntax error".red(), to));
                                    },
                                }.clone());
                            },
                            _ => {
                                return Err("Next token is not a varible".to_string());
                            },
                        }
                    },
                    _ => {
                        return Err("Next token is not a varible".to_string());
                    },
                }

            },
            _ => {},
        }
    }

    Ok(())
}
