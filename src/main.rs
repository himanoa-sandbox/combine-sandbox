use combine::*;
use combine::{many1, token, Parser,count_min_max} ;
use combine::parser::char::spaces;
use combine::error::ParseError;


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
    children: Box<Block>
}



#[derive(Debug, PartialEq, Eq)]
pub enum Block {
    Paragraph { children: Vec<Inline> },
    Heading { level: HeadingLevel, children: Vec<Inline>, id: Option<String>},
    // Horizontal ruled line section
    HorizontalRuledLine,
    NextPage,
    // List section
    UnorderdList { children: Vec<Inline> },
    UnorderdListItem { level: ListLevel, children: Vec<Inline> },
    CheckList { children: Vec<Inline> },
    CheclListItem { level: ListLevel, children: Vec<Inline>, checked: bool },
    OrderdList { children: Vec<Inline> },
    OrderdListItem { level: ListLevel, children: Vec<Inline> },
    Label { children:Vec<Inline>, key: Vec<Inline> },
    Qanda { question: Vec<Inline>, answer:Vec<Inline> },
    CodeBlock { children: Vec<Inline>, title: Option<String>, file_type: Option<String> },
    // Unsupport CodeBlockWithSpeachBaloon
    Block { children: Vec<Inline>, title: Option<Vec<Inline>> },
    Table { columns: Vec<TableColumn>, rows: Vec<TableRow>, title: Option<String> }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Inline {
    // Paragraph section
    Value(String),
    Literal { children: Box<Inline> },
    Footnote { kind: FootnoteType, children: Box<Inline> },
    Lead { children: Box<Inline> },
    // Text format section

    Bold { children: Box<Inline> },
    Italic { children: Box<Inline> },
    Monospace { children: Box<Inline> },
    Marker { children: Box<Inline> },
    Underline { children: Box<Inline> },
    LineThrough { children: Box<Inline> },
    Big { children: Box<Inline> },
    // Unsupport Superscript
    // Unsupport Subscript
    // Unsupport Curvequote
    // Unsupport Apostorofy

    // Label section
    // Link section
    Link { href: String, children: Box<Inline> },
    Mail { to: String, children: Box<Inline> },
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
    InlineCode { children: Box<Inline> },
}



parser!{
    fn expr[Input]()(Input) -> Block
    where [Input: Stream<Token = char>]
    {
        block_expr()
    }
}

pub fn block_expr<Input>() -> impl Parser<Input, Output = Block>
    where Input: Stream<Token = char>,
          Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    heading_expr()
}

pub fn inline_expr<Input>() -> impl Parser<Input, Output = Inline>
    where Input: Stream<Token = char>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position>
{
    value_expr()
}


pub fn value_expr<Input>() -> impl Parser<Input, Output = Inline>
    where Input: Stream<Token = char>,
          Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    return many1::<String, _, _>(any()).map(|s| Inline::Value(s))
}

pub fn heading_expr<Input>() -> impl Parser<Input, Output = Block>
    where Input: Stream<Token = char>,
          Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    
        many1::<Vec<char>, _, _>(token('='))
            .and(many1::<String, _, _>(token(' ')))
            .and(inline_expr())
            .map(|((heading, spaces), children)| {
                if heading.len() > 5 {
                    let heading_raw = heading.iter().collect::<String>();
                    return Block::Paragraph { children: vec![Inline::Value(heading_raw + spaces.as_str()), children] };
                }
                let level = match heading.len() {
                    1 => HeadingLevel::Title,
                    2 => HeadingLevel::Level1,
                    3 => HeadingLevel::Level2,
                    4 => HeadingLevel::Level3,
                    5 => HeadingLevel::Level4,
                    _ => {
                        unreachable!()
                    },
                };
                Block::Heading { id: None, children: vec![children], level }
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
            Block::Heading {
                level: HeadingLevel::Title,
                children: vec![Inline::Value("Heading".to_string())],
                id: None
            }
        );

        let (actual, _) = expr().parse("== Heading").unwrap();
        assert_eq!(
            actual,
            Block::Heading {
                level: HeadingLevel::Level1,
                children: vec![Inline::Value("Heading".to_string())],
                id: None
            }
        );

        let (actual, _) = expr().parse("=== Heading").unwrap();
        assert_eq!(
            actual,
            Block::Heading {
                level: HeadingLevel::Level2,
                children: vec![Inline::Value("Heading".to_string())],
                id: None
            }
        );

        let (actual, _) = expr().parse("==== Heading").unwrap();
        assert_eq!(
            actual,
            Block::Heading {
                level: HeadingLevel::Level3,
                children: vec![Inline::Value("Heading".to_string())],
                id: None
            }
        );


        let (actual, _) = expr().parse("===== Heading").unwrap();
        assert_eq!(
            actual,
            Block::Heading {
                level: HeadingLevel::Level4,
                children: vec![Inline::Value("Heading".to_string())],
                id: None
            }
        );

        let (actual, _) = expr().parse("====== Heading").unwrap();
        assert_eq!(
            actual,
            Block::Paragraph {
                children: vec![Inline::Value("====== ".to_string()), Inline::Value("Heading".to_string())],
            }
        );
    }
}
