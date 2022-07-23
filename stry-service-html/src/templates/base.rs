use windswept::{rsx, Render};

use super::partials;

pub fn base<R: Render>(main: R) -> impl Render {
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
            <main class="container mx-auto my-2 max-w-3xl">
                {partials::nav()}

                {main}
            </main>
        </body>
        </html>
    }
}
