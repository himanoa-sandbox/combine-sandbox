use anyhow::Result;
use combine::error::ParseError;
use combine::parser::char::{newline, space, string};
use combine::*;


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
    BlankBlock
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

pub fn parse(s: &str) -> Result<Vec<Block>> {
    let mut parser = document();

    let trim_targets: &[_] = &['\n', ' '];
    let s  = s.trim_start_matches(trim_targets);

    return Ok(parser.parse(s).map(|(tokens, _)| tokens)?);
}

pub fn document<Input>() -> impl Parser<Input, Output = Vec<Block>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many::<Vec<Block>, _, _>(block())
}

pub fn block<Input>() -> impl Parser<Input, Output = Block>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    choice((
        heading_block(),
        paragraph_block(),
        blank_block()
    ))
}

parser! {
    fn inline[Input]()(Input) -> Inline
    where
        [Input: Stream<Token = char>] {
            inline_()
        }
}

pub fn inline_<Input>() -> impl Parser<Input, Output = Inline>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let inline_combinator = choice((value(), bold(), italic(), monospace(), line_break()));
    attempt(inline_combinator).or(satisfy(|c| c != '\n').map(|s: char| Inline::Value(s.to_string())))
}

pub fn line_break<Input>() -> impl Parser<Input, Output = Inline>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    choice(
        (
            newline().and(not_followed_by(newline())).map(|_| Inline::SoftBreak),
            space().and(string("+\n")).map(|_| Inline::HardBreak)
        )
    )
}

fn bold<Input>() -> impl Parser<Input, Output = Inline>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let symbol = '*';
    skip_many(token(' ')).and(between(token(symbol), token(symbol), inline())).map(|(_, children)| {
        Inline::Bold {
            children: Box::new(children),
        }
    })
}

pub fn monospace<Input>() -> impl Parser<Input, Output = Inline>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let symbol = '`';
    skip_many(token(' ')).and(between(token(symbol), token(symbol), inline())).map(|(_, children)| {
        Inline::Monospace {
            children: Box::new(children),
        }
    })
}

fn italic<Input>() -> impl Parser<Input, Output = Inline>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let symbol = '_';
    between(token(symbol), token(symbol), inline()).map(|children| Inline::Italic {
        children: Box::new(children),
    })
}

pub fn value<Input>() -> impl Parser<Input, Output = Inline>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let ignore_tokens: &_ = &['\n', '*', '_', '`'];
    return many1::<String, _, _>(satisfy(move |c| {
        &ignore_tokens.iter().skip_while(|i| c != **i).count() == &0
    }))
    .map(|s| Inline::Value(s));
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
    many1::<Vec<Inline>, _, _>(attempt(inline())).map(|children| Block::Paragraph { children })
    // many1::<Vec<Inline>, _, _>(inline()).and(look_ahead(count_min_max::<String, _, _>(1, 2, newline())))
}

pub fn blank_block<Input>() -> impl Parser<Input, Output = Block>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    count_min_max::<String, _, _>(2, 2, newline()).map(|_| Block::BlankBlock)
}

pub fn horizontal_ruled_line_block<Input>()->impl Parser<Input, Output = Block>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>
{
    token('<').and(token('<')).and(token('<')).map(|_| Block::HorizontalRuledLine)
}

fn main() -> () {
    let asciidoc = "
    == This is a Heading
    This is a Paragraph";

    let _result = parse(asciidoc).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use combine::error::StringStreamError;
    use pretty_assertions::assert_eq;
    // -- utils
    fn take_parse_result<T, E>(t: (T, E)) -> T {
        return t.0;
    }
    // --

    #[test]
    fn test_parse_function() {
        let asciidoc = "
== This is a Heading

This is a Paragraph

== Foobar

This is a *bold* text

This is a _italic_ text

wrap break *
a

";

        let result = parse(asciidoc).unwrap();
        assert_eq!(
            result,
            vec![
                Block::Heading {
                    level: HeadingLevel::Level1,
                    id: None,
                    children: vec![Inline::Value("This is a Heading".to_string())]
                },
                Block::BlankBlock,
                Block::Paragraph {
                    children: vec![Inline::Value("This is a Paragraph".to_string())]
                },
                Block::BlankBlock,
                Block::Heading {
                    level: HeadingLevel::Level1,
                    id: None,
                    children: vec![Inline::Value("Foobar".to_string())]
                },
                Block::BlankBlock,
                Block::Paragraph {
                    children: vec![
                        Inline::Value("This is a ".to_string()),
                        Inline::Bold {
                            children: Box::new(Inline::Value("bold".to_string()))
                        },
                        Inline::Value(" text".to_string()),
                    ]
                },
                Block::BlankBlock,
                Block::Paragraph {
                    children: vec![
                        Inline::Value("This is a ".to_string()),
                        Inline::Italic {
                            children: Box::new(Inline::Value("italic".to_string()))
                        },
                        Inline::Value(" text".to_string()),
                    ]
                },
                Block::BlankBlock, 
                Block::Paragraph {
                    children: vec![
                        Inline::Value("wrap break ".to_string()),
                        Inline::Value("*".to_string()),
                        Inline::SoftBreak,
                        Inline::Value("a".to_string())
                    ]
                },
                Block::BlankBlock
            ]
        )
    }
    #[test]
    fn test_inline() {
        let actual = inline().parse(" aadf").map(take_parse_result);
        assert_eq!(actual, Ok(Inline::Value(" aadf".to_string())));
    }

    #[test]
    fn test_block() {
        let actual = block()
            .parse("=== Head\n\nHelloWorld")
            .map(take_parse_result);
        assert_eq!(
            actual,
            Ok(Block::Heading {
                level: HeadingLevel::Level2,
                id: None,
                children: vec![Inline::Value("Head".to_string())]
            })
        );
    }

    #[test]
    fn test_parse_heading() {
        let (actual, _) = heading_block().parse("= Heading").unwrap();
        assert_eq!(
            actual,
            Block::Heading {
                level: HeadingLevel::Title,
                children: vec![Inline::Value("Heading".to_string())],
                id: None
            }
        );

        let (actual, _) = heading_block().parse("== Heading").unwrap();
        assert_eq!(
            actual,
            Block::Heading {
                level: HeadingLevel::Level1,
                children: vec![Inline::Value("Heading".to_string())],
                id: None
            }
        );

        let (actual, _) = heading_block().parse("=== Heading").unwrap();
        assert_eq!(
            actual,
            Block::Heading {
                level: HeadingLevel::Level2,
                children: vec![Inline::Value("Heading".to_string())],
                id: None
            }
        );

        let (actual, _) = heading_block().parse("==== Heading").unwrap();
        assert_eq!(
            actual,
            Block::Heading {
                level: HeadingLevel::Level3,
                children: vec![Inline::Value("Heading".to_string())],
                id: None
            }
        );

        let (actual, _) = heading_block().parse("===== Heading").unwrap();
        assert_eq!(
            actual,
            Block::Heading {
                level: HeadingLevel::Level4,
                children: vec![Inline::Value("Heading".to_string())],
                id: None
            }
        );

        let (actual, _) = heading_block().parse("====== Heading").unwrap();
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
    fn test_bold() {
        let actual = bold().parse("*人間*").map(take_parse_result);
        assert_eq!(
            actual,
            Ok(Inline::Bold {
                children: Box::new(Inline::Value("人間".to_string()))
            })
        );

        let actual = bold().parse("*人間*").map(take_parse_result);
        assert_eq!(
            actual,
            Ok(Inline::Bold {
                children: Box::new(Inline::Value("人間".to_string()))
            })
        );
    }

    #[test]
    fn test_italic() {
        let actual = italic().parse("_人間_").map(take_parse_result);
        assert_eq!(
            actual,
            Ok(Inline::Italic {
                children: Box::new(Inline::Value("人間".to_string()))
            })
        );
    }

    #[test]
    fn test_monospace() {
        let actual = monospace().parse("`人間`").map(take_parse_result);
        assert_eq!(
            actual,
            Ok(Inline::Monospace {
                children: Box::new(Inline::Value("人間".to_string()))
            })
        );
    }

    #[test]
    fn test_value() {
        let actual = value().parse("人間").map(take_parse_result);
        assert_eq!(actual, Ok(Inline::Value("人間".to_string())));
    }

    #[test]
    fn test_line_break() {
        let actual = line_break().parse("\n").map(take_parse_result);
        assert_eq!(actual, Ok(Inline::SoftBreak));

        let actual = line_break().parse("\n\n").is_err();
        assert_eq!(actual, true);

        let actual = line_break().parse(" +\n").map(take_parse_result);
        assert_eq!(actual, Ok(Inline::HardBreak));
    }

    #[test]
    fn test_paragraph() {
        let actual = paragraph_block().parse("人間 *a* 人間").map(take_parse_result);
        assert_eq!(
            actual,
            Ok(Block::Paragraph {
                children: vec![
                    Inline::Value("人間 ".to_string()),
                    Inline::Bold{children: Box::new(Inline::Value("a".to_string()))},
                    Inline::Value(" 人間".to_string())
                ]
            })
        );

        let actual = paragraph_block().parse("人間 ").map(take_parse_result);
        assert_eq!(
            actual,
            Ok(Block::Paragraph {
                children: vec![
                    Inline::Value("人間 ".to_string()),
                ]
            })
        );
        let actual = paragraph_block().parse("人間").map(take_parse_result);
        assert_eq!(
            actual,
            Ok(Block::Paragraph {
                children: vec![Inline::Value("人間".to_string())]
            })
        );

        let actual = paragraph_block().parse("人間\n").map(take_parse_result);
        assert_eq!(
            actual,
            Ok(Block::Paragraph {
                children: vec![Inline::Value("人間".to_string()), Inline::SoftBreak]
            })
        );
    }

    #[test]
    fn test_horizontal_ruled_line_block() {
        let actual = horizontal_ruled_line_block().parse("<<<").map(take_parse_result);
        assert_eq!(
            actual,
            Ok(Block::HorizontalRuledLine)
        );

        let actual = horizontal_ruled_line_block().parse("<<").map(take_parse_result);
        assert_eq!(actual, Err(StringStreamError::Eoi));
    }
}
