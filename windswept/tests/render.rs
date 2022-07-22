use windswept::{rsx, Render};

fn button(text: &str) -> impl Render + '_ {
    rsx! {
        <>
            <div>{text}</div>
        </>
    }
}

fn nav() -> impl Render {
    rsx! {
        <>
            {button("Hello World!")}
        </>
    }
}

fn base(main: impl Render) -> impl Render {
    rsx! {
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8" />
            <meta http-equiv="X-UA-Compatible" content="IE=edge" />
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <title>"Document"</title>
            <link rel="stylesheet" href="/assets/main.css" />
            <script defer src="/assets/alpinejs-collapse.js"></script>
            <script defer src="/assets/alpinejs.js"></script>
            <script defer src="/assets/main.js"></script>
        </head>
        <body class="bg-zinc-900">
            <main class="container mx-auto my-2 max-w-screen-xl">
                {nav()}

                {main}
            </main>
        </body>
        </html>
    }
}

#[test]
fn test_button() {
    let expected = "<div>Hello World!</div>".to_string();
    let got = button("Hello World!").render().unwrap();
    assert_eq!(expected, got);
}

#[test]
fn test_nav() {
    let expected = "<div>Hello World!</div>".to_string();
    let got = nav().render().unwrap();
    assert_eq!(expected, got);
}

#[test]
fn test_base() {
    let expected = concat!(
        r#"<!DOCTYPE html>"#,
        r#"<html lang="en">"#,
        r#"<head>"#,
        r#"<meta charset="UTF-8">"#,
        r#"<meta http-equiv="X-UA-Compatible" content="IE=edge">"#,
        r#"<meta name="viewport" content="width=device-width, initial-scale=1.0">"#,
        r#"<title>Document</title>"#,
        r#"<link rel="stylesheet" href="/assets/main.css">"#,
        r#"<script defer src="/assets/alpinejs-collapse.js"></script>"#,
        r#"<script defer src="/assets/alpinejs.js"></script>"#,
        r#"<script defer src="/assets/main.js"></script>"#,
        r#"</head>"#,
        r#"<body class="bg-zinc-900">"#,
        r#"<main class="container mx-auto my-2 max-w-screen-xl">"#,
        r#"<div>Hello World!</div>"#,
        r#"</main>"#,
        r#"</body>"#,
        r#"</html>"#,
    )
    .to_string();
    let got = base("").render().unwrap();
    assert_eq!(expected, got);
}

#[test]
fn test_if() {
    fn render(name: Option<&str>) -> String {
        rsx! {
            <p>
            "Hello, "
            {if let Some(name) = name {
                name
            } else {
                "World"
            }}
            "!"
            </p>
        }
        .render()
        .unwrap()
    }

    assert_eq!("<p>Hello, World!</p>", render(None).as_str());

    assert_eq!("<p>Hello, John!</p>", render(Some("John")).as_str());
}

#[test]
fn test_for() {
    let items = vec![1, 2, 3, 4, 5];

    let frag = rsx! {
        <ul>
        {for item in items {
            rsx! { <li>{item}</li> }
        }}
        </ul>
    }
    .render()
    .unwrap();

    let expected = concat!(
        "<ul>",
        "<li>1</li>",
        "<li>2</li>",
        "<li>3</li>",
        "<li>4</li>",
        "<li>5</li>",
        "</ul>",
    );

    assert_eq!(expected, frag.as_str());
}

#[test]
fn test_match() {
    fn render(name: Option<&str>) -> String {
        rsx! {
            <p>
            "Hello, "
            {match name {
                Some(name) => name,
                None => "World",
            }}
            "!"
            </p>
        }
        .render()
        .unwrap()
    }

    assert_eq!("<p>Hello, World!</p>", render(None).as_str());

    assert_eq!("<p>Hello, John!</p>", render(Some("John")).as_str());
}
