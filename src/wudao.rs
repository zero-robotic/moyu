use console::style;
use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Class, Name, Predicate};

#[derive(Debug, Default)]
struct Word {
    keyword: String,
    phonetics: Vec<String>,
    translations: Vec<String>,
    sentences: Vec<String>,
}

impl Word {
    pub fn print(self) {
        println!("{}", style(self.keyword).bold().red());
        for phonetic in self.phonetics.iter() {
            print!("{}  ", style(phonetic).cyan());
        }
        println!();
        for translation in self.translations.iter() {
            println!("{}", style(translation).green());
        }
        println!("例句:");
        for sentence in self.sentences.iter() {
            println!("{}", style(sentence).magenta());
        }
    }
}

pub async fn youdao_dict(word: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path: String = format!("http://dict.youdao.com/w/{}/#keyfrom=dict2.top", word);
    let resp = reqwest::get(path).await?;
    let content: String = resp.text().await?;

    let document = Document::from(content.as_str());

    let mut word = Word::default();
    let result: Node = match document.find(Class("results-content")).next() {
        Some(node) => node,
        None => {
            println!("{}", style("有道查询单词出错").red());
            return Ok(());
        }
    };

    word.keyword = result.find(Class("keyword")).next().unwrap().text();
    word.phonetics = get_phonetics(&result)?;
    word.translations = get_translations(&result)?;
    word.sentences = get_sentences(&result)?;

    word.print();

    Ok(())
}

fn get_phonetics(node: &Node) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut phonetics: Vec<String> = Vec::new();

    let nodes = node.find(Class("baav").descendant(Class("pronounce")));
    for phonetic in nodes {
        let phonetic = phonetic.text().split_whitespace().collect::<String>();
        phonetics.push(phonetic);
    }

    Ok(phonetics)
}

fn get_translations(node: &Node) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut translations: Vec<String> = Vec::new();

    let nodes: Node = node.find(Class("trans-container")).take(1).next().unwrap();
    for translation in nodes.find(Name("ul").descendant(Name("li"))) {
        translations.push(translation.text().trim().to_string());
    }

    Ok(translations)
}

fn get_sentences(node: &Node) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut sentences: Vec<String> = Vec::new();

    let examples = node.find(Attr("id", "examplesToggle")).next();
    if examples.is_none() {
        return Ok(sentences);
    }

    let authority = node.find(Attr("id", "authority")).next();
    if authority.is_some() {
        for node in authority.unwrap().find(Name("ul").descendant(Name("li"))) {
            let sentence = node.find(Name("p")).take(1).next().unwrap().text();
            sentences.push(sentence.trim().to_string());
        }
    }

    Ok(sentences)
}