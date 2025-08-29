use kuchikikiki::traits::*;

fn main() {
    let html = r"
        <!DOCTYPE html>
        <html>
        <head></head>
        <body>
            <template>
                <h1>Example</h1>
                <p class='foo'>Hello, world!</p>
                <p class='foo'>I love HTML</p>
                <div element />
            </template>
        </body>
        </html>
    ";
    let document = kuchikikiki::parse_html().one(html);

    let template_node = document.select_first("template").unwrap();
    let content_node = template_node.template_contents.as_ref().unwrap();

    println!("{}", content_node);
}
