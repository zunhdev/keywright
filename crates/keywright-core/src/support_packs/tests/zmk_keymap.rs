//! Reads what a support pack takes from a ZMK `.keymap` file: the layers, with their names,
//! bindings and sensor bindings, and the conditional layers.
//!
//! It knows just enough devicetree for keymaps like Keebart's: comments, `#define`d
//! numbers, nodes, and properties whose values are strings or lists of cells. A binding
//! comes out as a backup writes it, without the `&` and with the `#define`s replaced:
//! `&mo LOWER` is `mo 1`.

use std::collections::HashMap;

/// A keymap, as the file defines it.
#[derive(Debug)]
pub struct Keymap {
    /// The layers, in order: a layer's index is its ID.
    pub layers: Vec<Layer>,
    /// The conditional layers: the layers that must all be on, then the layer that is.
    pub conditional_layers: Vec<(Vec<u32>, u32)>,
}

/// A layer.
#[derive(Debug)]
pub struct Layer {
    /// Its `display-name`, or nothing.
    pub name: String,
    /// What each key does.
    pub bindings: Vec<String>,
    /// What each sensor, such as a knob, does.
    pub sensor_bindings: Vec<String>,
}

/// Reads a keymap file.
pub fn read(source: &str) -> Keymap {
    let mut defines = HashMap::new();
    let mut code = String::new();
    for line in strip_comments(source).lines() {
        let line = line.trim();
        if let Some(define) = line.strip_prefix("#define") {
            let mut words = define.split_whitespace();
            let (Some(name), Some(value)) = (words.next(), words.next()) else {
                panic!("a #define without a value: {line}");
            };
            defines.insert(name.to_owned(), value.to_owned());
        } else if !line.starts_with('#') {
            code.push_str(line);
            code.push('\n');
        }
    }

    // The file is one node's body, with the root node, `/ { … };`, in it.
    let file = Node::parse_body(&mut tokenize(&code).into_iter());
    let nodes = file.descendants();
    let keymap = nodes
        .iter()
        .find(|node| node.string("compatible") == Some("zmk,keymap"))
        .expect("the file should have a keymap");
    let layers = keymap
        .children
        .iter()
        .map(|layer| Layer {
            name: layer.string("display-name").unwrap_or_default().to_owned(),
            bindings: layer.bindings("bindings", &defines),
            sensor_bindings: layer.bindings("sensor-bindings", &defines),
        })
        .collect();
    let conditional_layers = nodes
        .iter()
        .filter(|node| node.string("compatible") == Some("zmk,conditional-layers"))
        .flat_map(|node| node.children.iter())
        .map(|layer| {
            let numbers = |property| -> Vec<u32> {
                layer
                    .cells(property)
                    .split_whitespace()
                    .map(|word| {
                        let number = defines.get(word).map_or(word, String::as_str);
                        number.parse().expect("a layer should be a number")
                    })
                    .collect()
            };
            (numbers("if-layers"), numbers("then-layer")[0])
        })
        .collect();

    Keymap {
        layers,
        conditional_layers,
    }
}

/// The source without its `/* */` and `//` comments.
fn strip_comments(source: &str) -> String {
    let mut code = String::new();
    let mut rest = source;
    while let Some(start) = rest.find(['/']) {
        code.push_str(&rest[..start]);
        let after = &rest[start..];
        if let Some(block) = after.strip_prefix("/*") {
            let end = block.find("*/").expect("a block comment should end");
            rest = &block[end + 2..];
        } else if after.starts_with("//") {
            rest = &after[after.find('\n').unwrap_or(after.len())..];
        } else {
            code.push('/');
            rest = &after[1..];
        }
    }
    code.push_str(rest);
    code
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Open,
    Close,
    Semicolon,
    Equals,
    Word(String),
    String(String),
    /// What's between `<` and `>`.
    Cells(String),
}

fn tokenize(code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = code.chars().peekable();
    while let Some(c) = chars.next() {
        let mut until = |end: char| chars.by_ref().take_while(|c| *c != end).collect();
        match c {
            '{' => tokens.push(Token::Open),
            '}' => tokens.push(Token::Close),
            ';' => tokens.push(Token::Semicolon),
            '=' => tokens.push(Token::Equals),
            ',' => {}
            '"' => tokens.push(Token::String(until('"'))),
            '<' => tokens.push(Token::Cells(until('>'))),
            c if c.is_whitespace() => {}
            c => {
                let mut word = c.to_string();
                while let Some(next) =
                    chars.next_if(|c| !c.is_whitespace() && !"{};=,\"<".contains(*c))
                {
                    word.push(next);
                }
                tokens.push(Token::Word(word));
            }
        }
    }
    tokens
}

#[derive(Debug, Default)]
struct Node {
    properties: Vec<(String, Token)>,
    children: Vec<Node>,
}

impl Node {
    /// Reads a node's body, after its name, up to the end of its `{ … };`.
    fn parse_body(tokens: &mut impl Iterator<Item = Token>) -> Self {
        let mut node = Self::default();
        let mut words = Vec::new();
        while let Some(token) = tokens.next() {
            match token {
                Token::Word(word) => words.push(word),
                Token::Equals => {
                    let name = words.pop().expect("a property should have a name");
                    let value = tokens.next().expect("a property should have a value");
                    node.properties.push((name, value));
                    // Further values, as in `bindings = <&kp>, <&kp>;`, aren't needed.
                    for token in tokens.by_ref() {
                        if token == Token::Semicolon {
                            break;
                        }
                    }
                    words.clear();
                }
                Token::Open => {
                    node.children.push(Self::parse_body(tokens));
                    words.clear();
                }
                Token::Close => {
                    assert_eq!(tokens.next(), Some(Token::Semicolon));
                    return node;
                }
                Token::Semicolon | Token::String(_) | Token::Cells(_) => words.clear(),
            }
        }
        node
    }

    fn descendants(&self) -> Vec<&Self> {
        let mut nodes = vec![self];
        for child in &self.children {
            nodes.extend(child.descendants());
        }
        nodes
    }

    fn property(&self, name: &str) -> Option<&Token> {
        self.properties
            .iter()
            .find(|(property, _)| property == name)
            .map(|(_, value)| value)
    }

    fn string(&self, name: &str) -> Option<&str> {
        match self.property(name)? {
            Token::String(text) => Some(text),
            _ => None,
        }
    }

    fn cells(&self, name: &str) -> &str {
        match self.property(name) {
            Some(Token::Cells(cells)) => cells,
            _ => "",
        }
    }

    /// A list of bindings, such as `<&kp A &mo LOWER>`, as `["kp A", "mo 1"]`.
    fn bindings(&self, name: &str, defines: &HashMap<String, String>) -> Vec<String> {
        let mut bindings: Vec<Vec<&str>> = Vec::new();
        for word in self.cells(name).split_whitespace() {
            if let Some(behavior) = word.strip_prefix('&') {
                bindings.push(vec![behavior]);
            } else {
                let binding = bindings.last_mut().expect("a binding should start with &");
                binding.push(defines.get(word).map_or(word, String::as_str));
            }
        }
        bindings.into_iter().map(|words| words.join(" ")).collect()
    }
}
