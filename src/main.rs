use reqwest::blocking;
use std::{fmt, io, process};

fn main() {
    println!("What website would you like to crawl?");

    let mut root = String::new();
    io::stdin()
        .read_line(&mut root)
        .expect("failed to read line");

    root = root.trim().to_string();
    let mut tree = Tree {
        root: Node {
            url: root.clone(),
            root: root.clone(),
            nodes: Vec::new(),
        },
        seen_links: vec![root.clone()],
    };

    tree.crawl();

    //println!("{:#?}", tree);
    println!("\n{tree}");
}

#[derive(Debug)]
struct Tree {
    root: Node,
    seen_links: Vec<String>,
}

#[derive(Debug)]
struct Node {
    url: String,
    root: String,
    nodes: Vec<Node>,
}

impl Tree {
    fn crawl(&mut self) {
        self.root.crawl(&mut self.seen_links)
    }
    fn build_string(&self, depth: i32) -> String {
        self.root.build_string(depth)
    }
}

impl Node {
    fn crawl(&mut self, seen_links: &mut Vec<String>) {
        let links = self.get_links();
        for url in links {
            if !seen_links.contains(&url) {
                seen_links.push(url.clone());
                self.add_child(url, self.root.clone())
            }
        }
        for child in &mut self.nodes {
            child.crawl(seen_links);
        }
    }

    fn build_string(&self, mut depth: i32) -> String {
        let mut string = String::new();
        for _i in 0..depth-1 {
            string.push_str("    ")
        }
        if depth > 0 {
            string.push_str("+---");
            string.push_str(&self.url[self.root.len()..]);
        } else {
            string.push_str(&self.url);
        }
        depth += 1;
        string.push('\n');
        for node in &self.nodes {
            string.push_str(&node.build_string(depth))
        }
        string
    }

    fn get_links(&self) -> Vec<String> {
        let mut links = Vec::new();
        let text = match blocking::get(&self.url) {
            Result::Ok(value) => match value.text() {
                Result::Ok(value) => value,
                Result::Err(e) => {
                    eprintln!("err: {e}");
                    eprintln!("{}", self.url);
                    process::exit(1);
                }
            },
            Result::Err(e) => {
                println!("err: {e}");
                eprintln!("{}", self.url);
                process::exit(1);
            }
        };
        eprintln!("Connected to {}", self.url);

        // remove any non ascii bcs that breaks it lol
        let chars: String = text.chars().filter(|x| x.is_ascii()).collect();

        for i in 0..chars.len() - 3 {
            if chars[i..i + 3] == *"<a " {
                for j in i + 3..chars.len() - 5 {
                    if chars[j..j + 6] == *"href=\"" {
                        let mut url = String::new();
                        for k in j + 6..chars.len() {
                            if chars[k..k + 1] == *"\"" {
                                url = chars[j + 6..k].to_string();
                                break;
                            }
                        }
                        if url.contains("https://") || url.contains("http://") || url.len() <= 1 {
                            break;
                        } else if url[0..1] == *"/" {
                            url = self.root.clone() + &url[1..];
                            links.push(url)
                        } else if url.len() > 1
                            && url.len() > 1
                            && !url.contains("#")
                            && !url.contains("mailto:")
                            && !links.contains(&url)
                        {
                            links.push(self.url.clone() + &url);
                        }
                    }
                }
            }
        }
        return links;
    }

    fn add_child(&mut self, url: String, root: String) {
        let child = Node {
            url,
            root,
            nodes: Vec::new(),
        };
        self.nodes.push(child);
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let depth = 0;
        let display = self.build_string(depth);
        write!(f, "{}", display)
    }
}
