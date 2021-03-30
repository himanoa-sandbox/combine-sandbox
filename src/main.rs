use anyhow::{Result, anyhow};
use combine::*;
use combine::{many, token, skip_many};

#[derive(Debug, PartialEq, Eq)]
pub enum HeadingLevel {
    Title,
    Level1,
    Level2,
    Level3,
    Level4,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ListLevel {
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FootnoteType {
    Note,
    Tip,
    Important,
    Warning,
    Caution
}

#[derive(Debug, PartialEq, Eq)]
pub enum VideoProvider {
    Youtube,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TableColumn { 
    name: String
}

#[derive(Debug, PartialEq, Eq)]
pub struct TableRow { 
    children: Node
}


#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    // Paragraph section
    Value(String),
    Paragraph { children: Box<Node> },
    Literal { children: Box<Node> },
    DocumentTitle { children: Box<Node>},
    Footnote { kind: FootnoteType, children: Box<Node> },
    Lead { children: Box<Node> },
    // Text format section

    Bold { children: Box<Node> },
    Italic { children: Box<Node> },
    Monospace { children: Box<Node> },
    Marker { children: Box<Node> },
    Underline { children: Box<Node> },
    LineThrough { children: Box<Node> },
    Big { children: Box<Node> },
    // Unsupport Superscript
    // Unsupport Subscript
    // Unsupport Curvequote
    // Unsupport Apostorofy

    // Document header section
    Heading { level: HeadingLevel, children: Box<Node>, id: Option<String>},
    // Horizontal ruled line section
    HorizontalRuledLine,
    NextPage,
    // List section
    UnorderdList { children: Box<Node> },
    UnorderdListItem { level: ListLevel, children: Box<Node> },
    CheckList { children: Box<Node> },
    CheclListItem { level: ListLevel, children: Box<Node>, checked: bool },
    OrderdList { children: Box<Node> },
    OrderdListItem { level: ListLevel, children: Box<Node> },
    // Label section
    Label { children:Box<Node>, key: Box<Node> },
    Qanda { question: Box<Node>, answer:Box<Node> },
    // Link section
    Link { href: String, children: Box<Node> },
    Mail { to: String, children: Box<Node> },
    // Unsupport LinkWithAttribute
    // Unsupport InlineAnchor
    // Unsupport InnerCrossReference
    // Unsupport DocumentCrossReference
    
    // Image section
    Image { src:String, caption:Option<String> },
    InlineImage { src:String, caption:Option<String> },
    // Video section
    Video { id:String, provider: VideoProvider },
    // Code section
    InlineCode { children: Box<Node> },
    CodeBlock { children: Box<Node>, title: Option<String>, file_type: Option<String> },
    // Unsupport CodeBlockWithSpeachBaloon
    Block { children: Box<Node>, title: Option<Box<Node>> },
    Table { columns: Vec<TableColumn>, rows: Vec<TableRow>, title: Option<String> }
}


pub fn parse_heading(s: &str) -> Result<Node> {
    let (head, remaining) = many::<Vec<_>, _, _>(token('=')).parse(s)?;
    let (_, remaining) = skip_many(token(' ')).parse(remaining)?;
    let (children, _remining) = many::<String, _, _>(parser::char::letter()).parse(remaining)?;

    let level = match head.len() {
        1 => HeadingLevel::Title,
        2 => HeadingLevel::Level1,
        3 => HeadingLevel::Level2,
        4 => HeadingLevel::Level3,
        5 => HeadingLevel::Level4,
        0 => { return Err(anyhow!("parse error")) },
        _ => { return Err(anyhow!("NotHeading")) }
    };
    
    Ok(Node::Heading { level, children: Box::new(Node::Value(children)), id: None})
}

fn main() -> () {
    dbg!("{:?}", "hello");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_heading() {
        assert_eq!(
            parse_heading("= Heading").unwrap(),
            Node::Heading {
                level: HeadingLevel::Title,
                children: Box::new(Node::Value("Heading".to_string())),
                id: None
            }
        );

        assert_eq!(
            parse_heading("== Heading").unwrap(),
            Node::Heading {
                level: HeadingLevel::Level1,
                children: Box::new(Node::Value("Heading".to_string())),
                id: None
            }
        );

        assert_eq!(
            parse_heading("=== Heading").unwrap(),
            Node::Heading {
                level: HeadingLevel::Level2,
                children: Box::new(Node::Value("Heading".to_string())),
                id: None
            }
        );

        assert_eq!(
            parse_heading("==== Heading").unwrap(),
            Node::Heading {
                level: HeadingLevel::Level3,
                children: Box::new(Node::Value("Heading".to_string())),
                id: None
            }
        );

        assert_eq!(
            parse_heading("===== Heading").unwrap(),
            Node::Heading {
                level: HeadingLevel::Level4,
                children: Box::new(Node::Value("Heading".to_string())),
                id: None
            }
        );
    }
}
