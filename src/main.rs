use combine::error::ParseError;
use combine::parser::char::{letter, newline, space, string};
use combine::*;
use combine::{eof, count_min_max, token, look_ahead, Parser};

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
    Caution,
}

#[derive(Debug, PartialEq, Eq)]
pub enum VideoProvider {
    Youtube,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TableColumn {
    name: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TableRow {
    children: Box<Block>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Block {
    Paragraph {
        children: Vec<Inline>,
    },
    Heading {
        level: HeadingLevel,
        children: Vec<Inline>,
        id: Option<String>,
    },
    // Horizontal ruled line section
    HorizontalRuledLine,
    NextPage,
    // List section
    UnorderdList {
        children: Vec<Inline>,
    },
    UnorderdListItem {
        level: ListLevel,
        children: Vec<Inline>,
    },
    CheckList {
        children: Vec<Inline>,
    },
    CheclListItem {
        level: ListLevel,
        children: Vec<Inline>,
        checked: bool,
    },
    OrderdList {
        children: Vec<Inline>,
    },
    OrderdListItem {
        level: ListLevel,
        children: Vec<Inline>,
    },
    Label {
        children: Vec<Inline>,
        key: Vec<Inline>,
    },
    Qanda {
        question: Vec<Inline>,
        answer: Vec<Inline>,
    },
    CodeBlock {
        children: Vec<Inline>,
        title: Option<String>,
        file_type: Option<String>,
    },
    // Unsupport CodeBlockWithSpeachBaloon
    Block {
        children: Vec<Inline>,
        title: Option<Vec<Inline>>,
    },
    Table {
        columns: Vec<TableColumn>,
        rows: Vec<TableRow>,
        title: Option<String>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Inline {
    // Paragraph section
    Value(String),
    HardBreak,
    SoftBreak,
    Literal {
        children: Box<Inline>,
    },
    Footnote {
        kind: FootnoteType,
        children: Box<Inline>,
    },
    Lead {
        children: Box<Inline>,
    },
    // Text format section
    Bold {
        children: Box<Inline>,
    },
    Italic {
        children: Box<Inline>,
    },
    Monospace {
        children: Box<Inline>,
    },
    Marker {
        children: Box<Inline>,
    },
    Underline {
        children: Box<Inline>,
    },
    LineThrough {
        children: Box<Inline>,
    },
    Big {
        children: Box<Inline>,
    },
    // Unsupport Superscript
    // Unsupport Subscript
    // Unsupport Curvequote
    // Unsupport Apostorofy

    // Label section
    // Link section
    Link {
        href: String,
        children: Box<Inline>,
    },
    Mail {
        to: String,
        children: Box<Inline>,
    },
    // Unsupport LinkWithAttribute
    // Unsupport InlineAnchor
    // Unsupport InnerCrossReference
    // Unsupport DocumentCrossReference

    // Image section
    Image {
        src: String,
        caption: Option<String>,
    },
    InlineImage {
        src: String,
        caption: Option<String>,
    },
    // Video section
    Video {
        id: String,
        provider: VideoProvider,
    },
    // Code section
    InlineCode {
        children: Box<Inline>,
    },
}

parser! {
    fn document[Input]()(Input) -> Block
    where [Input: Stream<Token = char>]
    {
        block()
    }
}

pub fn block<Input>() -> impl Parser<Input, Output = Block>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    heading_block()
}

pub fn inline<Input>() -> impl Parser<Input, Output = Inline>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    choice((
        value(),
        line_break(),
    ))
}

pub fn line_break<Input>() -> impl Parser<Input, Output = Inline>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    choice(
        (
            newline().and(count_min_max::<Vec<char>, _, _>(0, 1, any())).map(|(_, c)|{
                if let Some(first) = c.first() {
                    if *first == '\n' {
                        return Inline::HardBreak;
                    }
                }
                Inline::SoftBreak
            }),
            space().and(string("+\n")).map(|_| Inline::HardBreak),
        )
    )
}

pub fn value<Input>() -> impl Parser<Input, Output = Inline>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    return many1::<String, _, _>(any()).map(|s| Inline::Value(s))
    // return many1::<String, _, _>(satisfy(|c| {
    //     let ignore_chars = ['\n', '+'];

    //     return ignore_chars
    //         .iter()
    //         .take_while(|ignore_char| **ignore_char == c)
    //         .count()
    //         == 0;
    // }))
    // .map(|s| Inline::Value(s));
}

pub fn heading_block<Input>() -> impl Parser<Input, Output = Block>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many1::<Vec<char>, _, _>(token('='))
        .and(many1::<String, _, _>(token(' ')))
        .and(inline())
        .map(|((heading, spaces), children)| {
            if heading.len() > 5 {
                let heading_raw = heading.iter().collect::<String>();
                return Block::Paragraph {
                    children: vec![Inline::Value(heading_raw + spaces.as_str()), children],
                };
            }
            let level = match heading.len() {
                1 => HeadingLevel::Title,
                2 => HeadingLevel::Level1,
                3 => HeadingLevel::Level2,
                4 => HeadingLevel::Level3,
                5 => HeadingLevel::Level4,
                _ => {
                    unreachable!()
                }
            };
            Block::Heading {
                id: None,
                children: vec![children],
                level,
            }
        })
}

pub fn paragraph_block<Input>() -> impl Parser<Input, Output = Block>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    fn when_last_line<Input>() -> impl Parser<Input, Output = Block>
    where
        Input: Stream<Token = char>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    {
        inline()
            .and(eof())
            .map(|(line_element, _)| Block::Paragraph {
                children: vec![line_element],
            })
    }

    fn when_two_staright_line_breaks<Input>() -> impl Parser<Input, Output = Block>
    where
        Input: Stream<Token = char>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    {
        inline()
            .skip(newline().silent())
            .skip(newline().silent())
            .map(|line_element| Block::Paragraph {
                children: vec![line_element],
            })
    }

    choice((
        inline().map(|line_element| Block::Paragraph {
            children: vec![line_element],
        }),
        when_two_staright_line_breaks(),
        when_last_line(),
    ))
}

fn main() -> () {
    dbg!("{:?}", "hello");
}

#[cfg(test)]
mod tests {
    use combine::error::ParseError;
    use combine::parser::char::{letter, newline, space, string};
    use combine::{eof, many1, token, look_ahead, Parser};
    use super::*;

    fn take_parse_result<T, E>(t: (T, E)) -> T {
        return t.0;
    }

    #[test]
    fn test_parse_heading() {
        let (actual, _) = document().parse("= Heading").unwrap();
        assert_eq!(
            actual,
            Block::Heading {
                level: HeadingLevel::Title,
                children: vec![Inline::Value("Heading".to_string())],
                id: None
            }
        );

        let (actual, _) = document().parse("== Heading").unwrap();
        assert_eq!(
            actual,
            Block::Heading {
                level: HeadingLevel::Level1,
                children: vec![Inline::Value("Heading".to_string())],
                id: None
            }
        );

        let (actual, _) = document().parse("=== Heading").unwrap();
        assert_eq!(
            actual,
            Block::Heading {
                level: HeadingLevel::Level2,
                children: vec![Inline::Value("Heading".to_string())],
                id: None
            }
        );

        let (actual, _) = document().parse("==== Heading").unwrap();
        assert_eq!(
            actual,
            Block::Heading {
                level: HeadingLevel::Level3,
                children: vec![Inline::Value("Heading".to_string())],
                id: None
            }
        );

        let (actual, _) = document().parse("===== Heading").unwrap();
        assert_eq!(
            actual,
            Block::Heading {
                level: HeadingLevel::Level4,
                children: vec![Inline::Value("Heading".to_string())],
                id: None
            }
        );

        let (actual, _) = document().parse("====== Heading").unwrap();
        assert_eq!(
            actual,
            Block::Paragraph {
                children: vec![
                    Inline::Value("====== ".to_string()),
                    Inline::Value("Heading".to_string())
                ],
            }
        );
    }

    #[test]
    fn test_value() {
        let actual = value().parse("人間").map(take_parse_result);
        assert_eq!(actual, Ok(Inline::Value("人間".to_string())));

        let actual = value().parse("人間\n人間").map(take_parse_result);
        assert_eq!(actual, Ok(Inline::Value("人間".to_string())))
    }

    #[test]
    fn test_line_break() {
        let actual = line_break().parse(" +\n").map(take_parse_result);
        assert_eq!(actual, Ok(Inline::HardBreak));

        let actual = line_break().parse("\n\n").map(take_parse_result);
        assert_eq!(actual, Ok(Inline::HardBreak));

        let actual = line_break().parse("\n").map(take_parse_result);
        assert_eq!(actual, Ok(Inline::SoftBreak));
    }

    #[test]
    fn test_inline() {
        let actual = inline().parse("\naadf").map(take_parse_result);
        assert_eq!(actual, Ok(Inline::Value("aadf".to_string())));
    }

    #[test]
    fn test_paragraph_parser() {
        let actual = paragraph_block().parse("人間").map(take_parse_result);
        assert_eq!(
            actual,
            Ok(Block::Paragraph {
                children: vec![Inline::Value("人間".to_string())]
            })
        );

        let actual = paragraph_block()
            .parse("cccc\n\nccccc")
            .map(take_parse_result);
        assert_eq!(
            actual,
            Ok(Block::Paragraph {
                children: vec![Inline::Value("cccc".to_string())]
            })
        );

        // line break
        let actual = paragraph_block()
            .parse("人間 +\n改行された人間")
            .map(take_parse_result);
        assert_eq!(
            actual,
            Ok(Block::Paragraph {
                children: vec![
                    Inline::Value("人間".to_string()),
                    Inline::HardBreak,
                    Inline::Value("改行された人間".to_string())
                ]
            })
        )
    }
}
