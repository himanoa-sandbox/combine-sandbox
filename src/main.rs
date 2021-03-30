#[macro_use]
use combine::*;
use combine::{many1, token, Parser, choice};
use combine::parser::char::spaces;
use combine::parser::char::letter;

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



parser!{
    fn expr[Input]()(Input) -> Node
    where [Input: Stream<Token = char>]
    {
        expr_()
    }
}

pub fn expr_<Input>() -> impl Parser<Input, Output = Node>
    where Input: Stream<Token = char>,
          Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    choice((
        heading_expr(),
        value_expr()
    ))
}



pub fn value_expr<Input>() -> impl Parser<Input, Output = Node>
    where Input: Stream<Token = char>,
          Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    return many1::<String, _, _>(letter()).map(|s| Node::Value(s))
}

pub fn heading_expr<Input>() -> impl Parser<Input, Output = Node>
    where Input: Stream<Token = char>,
          Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many1::<Vec<char>, _, _>(token('='))
        .skip(spaces().silent())
        .and(expr())
        .map(|(heading, children)| {
            let level = match heading.len() {
                1 => HeadingLevel::Title,
                2 => HeadingLevel::Level1,
                3 => HeadingLevel::Level2,
                4 => HeadingLevel::Level3,
                5 => HeadingLevel::Level4,
                _ => HeadingLevel::Level4
            };
            Node::Heading { id: None, children: Box::new(children), level }
        })
}

fn main() -> () {
    dbg!("{:?}", "hello");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_heading() {
        let (actual, _) = expr().parse("= Heading").unwrap();
        assert_eq!(
            actual,
            Node::Heading {
                level: HeadingLevel::Title,
                children: Box::new(Node::Value("Heading".to_string())),
                id: None
            }
        );

        let (actual, _) = expr().parse("== Heading").unwrap();
        assert_eq!(
            actual,
            Node::Heading {
                level: HeadingLevel::Level1,
                children: Box::new(Node::Value("Heading".to_string())),
                id: None
            }
        );

        let (actual, _) = expr().parse("=== Heading").unwrap();
        assert_eq!(
            actual,
            Node::Heading {
                level: HeadingLevel::Level2,
                children: Box::new(Node::Value("Heading".to_string())),
                id: None
            }
        );

        let (actual, _) = expr().parse("==== Heading").unwrap();
        assert_eq!(
            actual,
            Node::Heading {
                level: HeadingLevel::Level3,
                children: Box::new(Node::Value("Heading".to_string())),
                id: None
            }
        );


        let (actual, _) = expr().parse("====== Heading").unwrap();
        assert_eq!(
            actual,
            Node::Heading {
                level: HeadingLevel::Level4,
                children: Box::new(Node::Value("Heading".to_string())),
                id: None
            }
        );
    }
}
