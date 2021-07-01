use anyhow::Result;
use combine::error::ParseError;
use combine::parser::char::{newline, space, spaces, string};
use combine::*;
use std::collections::HashMap;

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
        children: Vec<ListItem>,
    },
    OrderdList {
        children: Vec<ListItem>,
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
    BlankBlock,
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
    // Unsupport Superscript
    // Unsupport Subscript
    // Unsupport Curvequote
    // Unsupport Apostorofy
    Macro {
        attributes: Attributes,
        kind: String,
        id: String,
    },
    // Code section
    InlineCode {
        children: Box<Inline>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Attributes {
    Position(Vec<String>),
    Named(HashMap<String, String>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ListItem {
    Normal {
        children: Vec<Inline>,
        level: u32,
    },
    Check {
        children: Vec<Inline>,
        level: u32,
        checked: bool,
    },
}

pub fn parse(s: &str) -> Result<Vec<Block>> {
    let mut parser = document();

    let trim_targets: &[_] = &['\n', ' '];
    let s = s.trim_start_matches(trim_targets);

    return Ok(parser.parse(s).map(|(tokens, _)| tokens)?);
}

fn document<Input>() -> impl Parser<Input, Output = Vec<Block>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many::<Vec<Block>, _, _>(block())
}

fn block<Input>() -> impl Parser<Input, Output = Block>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    choice((
        heading_block(),
        horizontal_ruled_line_block(),
        ordered_list_block(),
        unordered_list_block(),
        paragraph_block(),
        blank_block(),
    ))
}

parser! {
    fn inline[Input]()(Input) -> Inline
    where
        [Input: Stream<Token = char>] {
            inline_()
        }
}

fn inline_<Input>() -> impl Parser<Input, Output = Inline>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let inline_combinator = choice((
        value(),
        bold(),
        italic(),
        attempt(inline_code()).or(monospace()),
        marker(),
        line_break(),
    ));
    attempt(inline_combinator)
        .or(satisfy(|c| c != '\n').map(|s: char| Inline::Value(s.to_string())))
}

parser! {
    fn list_item_inline[Input]()(Input) -> Inline
    where
        [Input: Stream<Token = char>] {
            list_item_inline_()
        }
}

fn list_item_inline_<Input>() -> impl Parser<Input, Output = Inline>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let inline_combinator = choice((value(), bold(), italic(), monospace()));
    attempt(inline_combinator)
        .or(satisfy(|c| c != '\n').map(|s: char| Inline::Value(s.to_string())))
}

fn line_break<Input>() -> impl Parser<Input, Output = Inline>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    choice((
        newline()
            .and(not_followed_by(newline()))
            .map(|_| Inline::SoftBreak),
        space().and(string("+\n")).map(|_| Inline::HardBreak),
    ))
}

fn bold<Input>() -> impl Parser<Input, Output = Inline>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let symbol = '*';
    skip_many(token(' '))
        .and(between(token(symbol), token(symbol), inline()))
        .map(|(_, children)| Inline::Bold {
            children: Box::new(children),
        })
}

pub fn monospace<Input>() -> impl Parser<Input, Output = Inline>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let symbol = '`';
    skip_many(token(' '))
        .and(between(token(symbol), token(symbol), inline()))
        .map(|(_, children)| Inline::Monospace {
            children: Box::new(children),
        })
}

fn italic<Input>() -> impl Parser<Input, Output = Inline>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let symbol = '_';
    skip_many(token(' '))
        .and(between(token(symbol), token(symbol), inline()))
        .map(|(_, children)| Inline::Italic {
            children: Box::new(children),
        })
}

fn marker<Input>() -> impl Parser<Input, Output = Inline>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let symbol = '#';
    skip_many(token(' '))
        .and(between(token(symbol), token(symbol), inline()))
        .map(|(_, children)| Inline::Marker {
            children: Box::new(children),
        })
}

fn inline_code<Input>() -> impl Parser<Input, Output = Inline>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let symbol = '`';
    (
        token(symbol),
        token(symbol),
        token(symbol),
        inline(),
        token(symbol),
        token(symbol),
        token(symbol),
    )
        .map(|(_, _, _, children, _, _, _)| Inline::InlineCode {
            children: Box::new(children),
        })
}

fn value<Input>() -> impl Parser<Input, Output = Inline>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let ignore_tokens: &_ = &['\n', '*', '_', '`', '#'];
    return many1::<String, _, _>(satisfy(move |c| {
        &ignore_tokens.iter().skip_while(|i| c != **i).count() == &0
    }))
    .map(|s| Inline::Value(s));
}

fn heading_block<Input>() -> impl Parser<Input, Output = Block>
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

fn paragraph_block<Input>() -> impl Parser<Input, Output = Block>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many1::<Vec<Inline>, _, _>(attempt(inline())).map(|children| Block::Paragraph { children })
    // many1::<Vec<Inline>, _, _>(inline()).and(look_ahead(count_min_max::<String, _, _>(1, 2, newline())))
}

fn blank_block<Input>() -> impl Parser<Input, Output = Block>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    count_min_max::<String, _, _>(2, 2, newline()).map(|_| Block::BlankBlock)
}

fn horizontal_ruled_line_block<Input>() -> impl Parser<Input, Output = Block>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    string("<<<").map(|_| Block::HorizontalRuledLine)
}

fn unordered_list_block<Input>() -> impl Parser<Input, Output = Block>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many1::<Vec<ListItem>, _, _>(
        list_item('*')
            .and(count_min_max::<Vec<char>, _, _>(0, 1, newline()))
            .map(|(list_item, _)| list_item),
    )
    .map(|items| Block::UnorderdList { children: items })
}

fn ordered_list_block<Input>() -> impl Parser<Input, Output = Block>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many1::<Vec<ListItem>, _, _>(
        list_item('.')
            .and(count_min_max::<Vec<char>, _, _>(0, 1, newline()))
            .map(|(list_item, _)| list_item),
    )
    .map(|items| Block::OrderdList { children: items })
}

fn list_item<Input>(list_char: char) -> impl Parser<Input, Output = ListItem>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    choice!(
        attempt(checked_list_item(list_char)),
        normal_list_item(list_char)
    )
}

fn normal_list_item<Input>(list_char: char) -> impl Parser<Input, Output = ListItem>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many1::<String, _, _>(token(list_char))
        .and(spaces())
        .and(many1::<Vec<Inline>, _, _>(attempt(list_item_inline_())))
        .map(|((list_tokens, _), inline)| ListItem::Normal {
            level: list_tokens.len() as u32,
            children: inline,
        })
}

fn checked_list_item<Input>(list_char: char) -> impl Parser<Input, Output = ListItem>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many1::<String, _, _>(token(list_char))
        .and(spaces())
        .and(between(
            token('['),
            token(']'),
            satisfy(|c| c == '*' || c == 'x' || c == ' '),
        ))
        .and(spaces())
        .and(many1::<Vec<Inline>, _, _>(attempt(list_item_inline_())))
        .map(
            |((((list_tokens, _), check_box_char), _), inline)| ListItem::Check {
                level: list_tokens.len() as u32,
                children: inline,
                checked: check_box_char != ' ',
            },
        )
}

fn named_atteributes<Input>() -> impl Parser<Input, Output = Attributes>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let value = || satisfy(|c| c != '=' && c != ']' && c != ',' && c != '\n');
    let expression = || (
        many::<String, _, _>(value()),
        token('='),
        many::<String, _, _>(value()),
    );

    let one_expression = || (
        token('['),
        expression(),
        token(']')
    ).map(|(_, (key, _, value), _)| {
        let mut attributes = HashMap::new();
        attributes.insert(key, value);
        Attributes::Named(attributes)
    });

    let expression_with_delimiter = || (expression(), token(','), skip_many(space()));
    let many_expressions = || (
        token('['),
        many1::<Vec<((String, _, String), _, ())>, _, _>(
            attempt(expression_with_delimiter())
        ),
        expression(),
        token(']')
    )
        .map(|(_, exprs, (last_key, _, last_value), _)| {
            let mut attributes: HashMap<String, String> = HashMap::new();

            for ((key, _, value), _, ()) in exprs.iter() {
                attributes.insert(key.clone(), value.clone());
            }
            attributes.insert(last_key, last_value);
            Attributes::Named(attributes)
        });

    choice!(attempt(many_expressions()), attempt(one_expression()))
}

fn position_attributes<Input>() -> impl Parser<Input, Output = Attributes>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let attribute = || many1::<String, _, _>(satisfy(|c| c != ']' && c != '\n' && c != ','));

    let single_attribute = || (token('['), attribute(), token(']')).map(|(_, attrs, _)| Attributes::Position(vec![attrs]));

    let attribute_with_delimiter = || (attribute(), token(','));
    let multi_attribute = || (
        token('['),
        many::<Vec<(String, _)>, _, _>(attempt(attribute_with_delimiter())),
        attribute(),
        token(']')
        )
        .map(|(_, attrs, attr, _)| {
            let mut attributes: Vec<String> = vec![];

            for (attr, _) in attrs.iter() {
                attributes.push(attr.clone())
            }
            attributes.push(attr);

            Attributes::Position(attributes)
        });

    choice!(attempt(multi_attribute()), single_attribute())
}

fn inline_macro<Input>() -> impl Parser<Input, Output = Inline>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    unimplemented!()
}

fn main() -> () {
    let asciidoc = "
    == This is a Heading
    This is a Paragraph";

    let _result = parse(asciidoc).unwrap();
    dbg!(_result);
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

This is a `monospace` text

This is a #marker# text

This is a ```inline code``` text

wrap break *
a

* foo
* bar


. foo
. bar
<<<

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
                        Inline::Value("This is a ".to_string()),
                        Inline::Monospace {
                            children: Box::new(Inline::Value("monospace".to_string()))
                        },
                        Inline::Value(" text".to_string()),
                    ]
                },
                Block::BlankBlock,
                Block::Paragraph {
                    children: vec![
                        Inline::Value("This is a ".to_string()),
                        Inline::Marker {
                            children: Box::new(Inline::Value("marker".to_string()))
                        },
                        Inline::Value(" text".to_string()),
                    ]
                },
                Block::BlankBlock,
                Block::Paragraph {
                    children: vec![
                        Inline::Value("This is a ".to_string()),
                        Inline::InlineCode {
                            children: Box::new(Inline::Value("inline code".to_string()))
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
                Block::BlankBlock,
                Block::UnorderdList {
                    children: vec![
                        ListItem::Normal {
                            children: vec![Inline::Value("foo".to_string())],
                            level: 1
                        },
                        ListItem::Normal {
                            children: vec![Inline::Value("bar".to_string())],
                            level: 1
                        }
                    ]
                },
                Block::BlankBlock,
                Block::OrderdList {
                    children: vec![
                        ListItem::Normal {
                            children: vec![Inline::Value("foo".to_string())],
                            level: 1
                        },
                        ListItem::Normal {
                            children: vec![Inline::Value("bar".to_string())],
                            level: 1
                        }
                    ]
                },
                Block::HorizontalRuledLine,
                Block::BlankBlock,
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
    fn test_marker() {
        let actual = marker().parse("#人間#").map(take_parse_result);
        assert_eq!(
            actual,
            Ok(Inline::Marker {
                children: Box::new(Inline::Value("人間".to_string()))
            })
        );
    }

    #[test]
    fn test_inline_code() {
        let actual = inline_code().parse(r"```npm```").map(take_parse_result);
        assert_eq!(
            actual,
            Ok(Inline::InlineCode {
                children: Box::new(Inline::Value("npm".to_string()))
            })
        );

        let actual = inline_code().parse(r"`npm`").map(take_parse_result);
        assert_eq!(
            actual,
            Err(combine::error::StringStreamError::UnexpectedParse)
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
        let actual = paragraph_block()
            .parse("人間 *a* 人間")
            .map(take_parse_result);
        assert_eq!(
            actual,
            Ok(Block::Paragraph {
                children: vec![
                    Inline::Value("人間 ".to_string()),
                    Inline::Bold {
                        children: Box::new(Inline::Value("a".to_string()))
                    },
                    Inline::Value(" 人間".to_string())
                ]
            })
        );

        let actual = paragraph_block().parse("人間 ").map(take_parse_result);
        assert_eq!(
            actual,
            Ok(Block::Paragraph {
                children: vec![Inline::Value("人間 ".to_string()),]
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
        let actual = horizontal_ruled_line_block()
            .parse("<<<")
            .map(take_parse_result);
        assert_eq!(actual, Ok(Block::HorizontalRuledLine));

        let actual = horizontal_ruled_line_block()
            .parse("<<")
            .map(take_parse_result);
        assert_eq!(actual, Err(StringStreamError::Eoi));
    }

    #[test]
    fn test_unordered_list() {
        let blocks = "* abc
* def";

        let actual = unordered_list_block().parse(blocks).map(take_parse_result);
        assert_eq!(
            actual,
            Ok(Block::UnorderdList {
                children: vec![
                    ListItem::Normal {
                        level: 1,
                        children: vec![Inline::Value("abc".to_string())]
                    },
                    ListItem::Normal {
                        level: 1,
                        children: vec![Inline::Value("def".to_string())]
                    }
                ]
            })
        );

        let blocks = "* [x] abc";

        let actual = unordered_list_block().parse(blocks).map(take_parse_result);
        assert_eq!(
            actual,
            Ok(Block::UnorderdList {
                children: vec![ListItem::Check {
                    level: 1,
                    children: vec![Inline::Value("abc".to_string())],
                    checked: true
                },]
            })
        );
    }

    #[test]
    fn test_ordered_list() {
        let blocks = ". abc
. def";

        let actual = ordered_list_block().parse(blocks).map(take_parse_result);
        assert_eq!(
            actual,
            Ok(Block::OrderdList {
                children: vec![
                    ListItem::Normal {
                        level: 1,
                        children: vec![Inline::Value("abc".to_string())]
                    },
                    ListItem::Normal {
                        level: 1,
                        children: vec![Inline::Value("def".to_string())]
                    }
                ]
            })
        )
    }

    #[test]
    fn test_ordered_list_item() {
        let actual = list_item('.')
            .parse(". foobar *foo* bar _foo_")
            .map(take_parse_result);
        assert_eq!(
            actual,
            Ok(ListItem::Normal {
                level: 1,
                children: vec![
                    Inline::Value("foobar ".to_string()),
                    Inline::Bold {
                        children: Box::new(Inline::Value("foo".to_string()))
                    },
                    Inline::Value(" bar ".to_string()),
                    Inline::Italic {
                        children: Box::new(Inline::Value("foo".to_string()))
                    },
                ]
            })
        );

        let actual = list_item('.').parse(". foobar\na").map(take_parse_result);
        assert_eq!(
            actual,
            Ok(ListItem::Normal {
                level: 1,
                children: vec![Inline::Value("foobar".to_string()),]
            })
        );

        let actual = list_item('.').parse(".. foobar\na").map(take_parse_result);
        assert_eq!(
            actual,
            Ok(ListItem::Normal {
                level: 2,
                children: vec![Inline::Value("foobar".to_string()),]
            })
        );
    }

    #[test]
    fn test_position_atteributes() {
        let expect_atteributes = vec!["foo".to_string()];

        let actual = position_attributes()
            .parse(r"[foo]")
            .map(take_parse_result);
        assert_eq!(actual, Ok(Attributes::Position(expect_atteributes)))
    }

    #[test]
    fn test_position_atteributes_when_multiple() {
        let expect_atteributes = vec!["foo".to_string(), "bar".to_string()];

        let actual = position_attributes()
            .parse(r"[foo,bar]")
            .map(take_parse_result);
        assert_eq!(actual, Ok(Attributes::Position(expect_atteributes)))
    }

    #[test]
    fn test_named_atteributes() {
        let mut expect_atteributes = HashMap::new();
        expect_atteributes.insert("foo".to_string(), "bar".to_string());

        let actual = named_atteributes()
            .parse(r"[foo=bar]")
            .map(take_parse_result);
        assert_eq!(actual, Ok(Attributes::Named(expect_atteributes)))
    }

    #[test]
    fn test_named_atteributes_when_multiple() {
        let mut expect_atteributes = HashMap::new();
        expect_atteributes.insert("foo".to_string(), "bar".to_string());
        expect_atteributes.insert("poe".to_string(), "fuga".to_string());

        let actual = named_atteributes()
            .parse(r"[foo=bar, poe=fuga]")
            .map(take_parse_result);
        assert_eq!(actual, Ok(Attributes::Named(expect_atteributes)))
    }
}
