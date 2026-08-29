pub fn encode(strs: Vec<String>) -> String {
    let mut code = String::new();

    for i in strs {
        code += &i;
        code += " ";
    }

    code
}

pub fn decode(s: String) -> Vec<String> {
    let code: Vec<String> = s
        .split_ascii_whitespace()
        .map(|word| word.to_string())
        .collect();

    code
}

fn main() {
    let mut string_list: Vec<String> = Vec::new();
    string_list.push("hola".to_string());
    string_list.push("muy".to_string());
    string_list.push("ok".to_string());
    let code = encode(string_list);
    println!("{}", code);
    let solution = decode(code);

    for i in solution {
        println!("{}", i);
    }
}
