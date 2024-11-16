use std::{io, process};
use reqwest::blocking;

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
            nodes: Vec::new(),
        }
    };

    tree.crawl();

    println!("{:#?}", tree);
}

#[derive(Debug)]
struct Tree { root: Node }

#[derive(Debug)]
struct Node {
    url: String,
    nodes: Vec<Node>,
}

impl Tree {
    fn crawl(&mut self) {
        self.root.crawl()
    }
}

impl Node {
    fn crawl(&mut self) {
        let links = self.get_links();
        for url in links {
            self.add_child(self.url.clone() + &url)
        }
        for child in &mut self.nodes {
            child.crawl();
        }
    }
    fn get_links(&self) -> Vec<String> {
        let mut links = Vec::new();
        let text = match blocking::get(&self.url) {
            Result::Ok(value) => {
                match value.text() {
                    Result::Ok(value) => value,
                    Result::Err(e) => {
                        eprintln!("err: {e}");
                        eprintln!("{}", self.url);
                        process::exit(1);
                    }
                }
            }
            Result::Err(e) => {
                println!("err: {e}");
                eprintln!("{}", self.url);
                process::exit(1);
            }
        };
        eprintln!("Connected to {}", self.url);
        
        let chars: Vec<_> = text.chars().collect();

        for i in 0..chars.len()-3 {
            if chars[i..i+3].to_string().contains("<a ") {
                for j in i..chars.len()-6 {
                    if chars[j..j+6].contains("href=\"") {
                        let mut url = String::new();
                        for k in j+7..chars.len() {
                            if chars[k] == '"' {
                                url = chars[j+7..k].to_string();
                                break;
                            }
                        }
                        dbg!(&url);
                        if url.contains("https://") || url.contains("http://") {
                            links.push(url);
                        } else {
                            links.push(self.url.clone() + &url);
                        }
                    }
                }
            }
        }
        return links;
    }

    fn add_child(&mut self, url: String) {
        let child = Node {
            url,
            nodes: Vec::new(),
        };
        self.nodes.push(child);
    }
}
