pub fn encode(strs: Vec<String>) -> String {
    let mut code = String::new();

    for i in strs {
        code += &i.len().to_string();
        code += "#";
        code += &i;
    }

    code
}

pub fn decode(s: String) -> Vec<String> {
    let mut code: Vec<String> = Vec::new();
    let mut i = 0;
    let bytes = s.as_bytes();

    while i != s.len() {
        let mut j = i;

        while bytes[j] != b'#' {
            j += 1;
        }

        let word_len: usize = s[i..j].parse().unwrap();
        let word = s[j + 1..j + 1 + word_len].to_string();

        code.push(word);

        i = j + word_len + 1;
    }

    code
}

fn main() {
    let string_list: Vec<String> = vec!["hola".to_string(), "muy".to_string()];
    let code = encode(string_list);
    println!("{}", code);
    let solution = decode(code);

    for i in solution {
        println!("{}", i);
    }
}
