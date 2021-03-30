
#[derive(Debug)]
enum HeadingLevel {
    Level1,
    Level2,
    Level3,
    Level4,
}

#[derive(Debug)]
enum ListLevel {
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
}

#[derive(Debug)]
enum FootnoteType {
    Note,
    Tip,
    Important,
    Warning,
    Caution
}

#[derive(Debug)]
enum VideoProvider {
    Youtube,
}

#[derive(Debug)]
struct TableColumn { 
    name: String
}

#[derive(Debug)]
struct TableRow { 
    children: Node
}


#[derive(Debug)]
enum Node {
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
    Heading { level: HeadingLevel, children: Box<Node>, id: String},
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


fn main() -> () {
    dbg!("{:?}", "hello");
}
