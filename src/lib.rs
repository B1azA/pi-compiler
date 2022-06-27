use std::collections::HashMap;

pub fn compile(code: String) -> Result<Vec<u8>, String> {
    let mut bytecode: Vec<u8> = vec![];
    let mut code: String = code;
    code.push(' ');

    let mut chars = code.chars();
    // println!("{:?}", chars);
    
    let signs = ['+', '-', '*', '/', ';', '(', ')', '[', ']', '{', '}', ':'];
    let whitespace = [' ', '\n', '\r'];

    let mut word = String::new();
    let mut args: usize = 0;
    let mut bytes: usize = 0;
    let mut functions: HashMap<String, usize> = HashMap::new();

    while let Some(ch) = chars.next() {
        if whitespace.contains(&ch) { // it is a whitespace
            if word.len() != 0 {
                // process word
                if args == 0 {
                    let result = process_instruction(word, &mut bytecode);
                    if let Ok(arguments) = result {
                        args = arguments.0;
                        bytes = arguments.1;
                    } else if let Err(error) = result {
                        return Err(error);
                    }
                } else {
                    let result = process_value(word, bytes, &mut bytecode);
                    args -= 1;
                    bytes = 0;
                    if let Err(error) = result {
                        return Err(error);
                    }
                }

                // println!("Whitespace: {}", word);
                word = String::new();
            }
            continue;
        }

        if signs.contains(&ch) { // it is a sign
            if word.len() != 0 {
                // process word

                match ch {
                    ':' => {
                        let result = process_fun(word, &mut functions, &mut bytecode);
                        if let Err(error) = result {
                            return Err(error);
                        }
                    }

                    _ => {}
                }

                // println!("Signs: {}", word);
                word = String::new();
            }
            continue;
        }

        word.push(ch);
    }

    Ok(bytecode)
}

fn process_fun(word: String, functions: &mut HashMap<String, usize>, bytecode: &mut Vec<u8>) -> Result<(), String> {    
    let ip = bytecode.len();

    if functions.contains_key(&word) {
        let message = format!("Function with name: \"{word}\" already exists.");
        return Err(message);
    } else if functions.values().all(|&r| r == ip) && functions.len() != 0 {
        let message = format!("Function with ip: \"{ip}\" already exists.");
        return Err(message);
    }

    functions.insert(word, ip);

    Ok(())
}

/// returns number of arguments for instruction
fn process_instruction(word: String, bytecode: &mut Vec<u8>) -> Result<(usize, usize), String> {
    match word.as_str() {
        "halt" => { bytecode.push(0); return Ok((0, 0)); },

        "pu1" => { bytecode.push(1); return Ok((1, 1)); },
        "pu2" => { bytecode.push(2);return Ok((1, 2)); },
        "pu4" => { bytecode.push(3); return Ok((1, 4)); },
        "pu8" => { bytecode.push(4); return Ok((1, 8)); },
        "pux" => { bytecode.push(5); return Ok((0, 0)); },

        "set" => { bytecode.push(6); return Ok((1, 1)); },
        "get" => { bytecode.push(7); return Ok((1, 1)); },

        "setx" => { bytecode.push(8); return Ok((0, 0)); },

        "off" => { bytecode.push(9); return Ok((0, 0)); },

        "cp" => { bytecode.push(10); return Ok((0, 0)); },

        "load1" => { bytecode.push(11); return Ok((0, 0)); },
        "load2" => { bytecode.push(12); return Ok((0, 0)); },
        "load4" => { bytecode.push(13); return Ok((0, 0)); },
        "load8" => { bytecode.push(14); return Ok((0, 0)); },

        "and" => { bytecode.push(15); return Ok((0, 0)); },
        "or" => { bytecode.push(16); return Ok((0, 0)); },
        "xor" => { bytecode.push(17); return Ok((0, 0)); },
        "lshift" => { bytecode.push(18); return Ok((0, 0)); },
        "rshift" => { bytecode.push(19); return Ok((0, 0)); },

        "add" => { bytecode.push(20); return Ok((0, 0)); },
        "sub" => { bytecode.push(21); return Ok((0, 0)); },
        "mul" => { bytecode.push(22); return Ok((0, 0)); },
        "div" => { bytecode.push(23); return Ok((0, 0)); },

        "addf" => { bytecode.push(24); return Ok((0, 0)); },
        "subf" => { bytecode.push(25); return Ok((0, 0)); },
        "mulf" => { bytecode.push(26); return Ok((0, 0)); },
        "divf" => { bytecode.push(27); return Ok((0, 0)); },
        
        "addd" => { bytecode.push(28); return Ok((0, 0)); },
        "subd" => { bytecode.push(29); return Ok((0, 0)); },
        "muld" => { bytecode.push(30); return Ok((0, 0)); },
        "divd" => { bytecode.push(31); return Ok((0, 0)); },

        "eq" => { bytecode.push(32); return Ok((0, 0)); },
        "neq" => { bytecode.push(33); return Ok((0, 0)); },
        "gr" => { bytecode.push(34); return Ok((0, 0)); },
        "sm" => { bytecode.push(35); return Ok((0, 0)); },
        "not" => { bytecode.push(36); return Ok((0, 0)); },

        "jmp" => { bytecode.push(37); return Ok((1, 8)); },
        "jmpif" => { bytecode.push(38); return Ok((1, 8)); },
        "jmpifn" => { bytecode.push(39); return Ok((1, 8)); },
        
        "jmpdy" => { bytecode.push(40); return Ok((0, 0)); },
        "jmpifdy" => { bytecode.push(41); return Ok((0, 0)); },
        "jmpifndy" => { bytecode.push(42); return Ok((0, 0)); },
        
        "ret" => { bytecode.push(43); return Ok((0, 0)); },
        "call" => { bytecode.push(44); return Ok((1, 8)); },
        "calldy" => { bytecode.push(45); return Ok((0, 0)); },
        "fun" => { bytecode.push(46); return Ok((1, 8)); },
        "fundy" => { bytecode.push(47); return Ok((0, 0)); },

        "ftod" => { bytecode.push(48); return Ok((0, 0)); },
        "dtof" => { bytecode.push(48); return Ok((0, 0)); },

        _ => {
            let message = format!("Unknown instruction: \"{word}\"");
            return Err(message); 
        }
    }
}

fn process_value(word: String, bytes: usize, bytecode: &mut Vec<u8>) -> Result<(), String> {
    // println!("Value: {}\nBytes: {}", word, bytes);
    let mut word = word;

    let last_char = word.chars().last().unwrap();
    if last_char == 'f' {
        word.remove(word.len() - 1);
        let result = word.parse::<f32>();

        if let Err(_) = result {
            let message = format!("Unable to parse value: \"{word}\" to float.");
            return Err(message);
        } else {
            let bytes_data = result.unwrap().to_be_bytes();

            if bytes != bytes_data.len() {
                let message = format!("Instructions expects: \"{bytes} bytes\", float is: \"4 bytes\" wide.");
                return Err(message);
            }

            for byte in bytes_data {
                bytecode.push(byte);
            }
        }

    } else if last_char == 'd' {
        word.remove(word.len() - 1);
        let result = word.parse::<f64>();

        if let Err(_) = result {
            let message = format!("Unable to parse value: \"{word}\" to double.");
            return Err(message);
        } else {
            let bytes_data = result.unwrap().to_be_bytes();

            if bytes != bytes_data.len() {
                let message = format!("Instructions expects: \"{bytes} bytes\", double is: \"8 bytes\" wide.");
                return Err(message);
            }

            for byte in bytes_data {
                bytecode.push(byte);
            }
        }
    }
    else {
        match bytes {
            0 => {
                let message = format!("Expects value with 0 byte size.");
                return Err(message);
            },
    
            1 => {
                let result = word.parse::<u8>();
    
                if let Err(_) = result {
                    let message = format!("Unable to parse value: \"{word}\" to 1 byte num.");
                    return Err(message);
                } else {
                    let val = result.unwrap();
                    bytecode.push(val);
                }
            },
    
            2 => {
                let result = word.parse::<u16>();
    
                if let Err(_) = result {
                    let message = format!("Unable to parse value: \"{word}\" to 2 bytes num.");
                    return Err(message);
                } else {
                    let bytes = result.unwrap().to_be_bytes();
                    for byte in bytes {
                        bytecode.push(byte);
                    }
                }
            },
    
            4 => {
                let result = word.parse::<u32>();
    
                if let Err(_) = result {
                    let message = format!("Unable to parse value: \"{word}\" to 4 bytes num.");
                    return Err(message);
                } else {
                    let bytes = result.unwrap().to_be_bytes();
                    for byte in bytes {
                        bytecode.push(byte);
                    }
                }
            },
    
            8 => {
                let result = word.parse::<u64>();
    
                if let Err(_) = result {
                    let message = format!("Unable to parse value: \"{word}\" to 8 bytes num.");
                    return Err(message);
                } else {
                    let bytes = result.unwrap().to_be_bytes();
                    for byte in bytes {
                        bytecode.push(byte);
                    }
                }
            },
    
            _ => {
                let message = format!("Unsupported number of bytes: \"{bytes}\".");
                return Err(message);
            }
        }
    }


    Ok(())
}