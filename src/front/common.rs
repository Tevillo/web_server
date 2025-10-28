use hypertext::prelude::*;
use hypertext::{Buffer, Renderable, maud};

pub async fn nav_bar(page: &str, buffer: &mut Buffer) {
    let pages = ["/home", "/schedule", "/server"];
    maud! {
        link rel="stylesheet" href=("style/base.css");
        body {
            div .navbar {
                ul {
                    @for p in pages.iter() {
                        @let li = &p[1..p.len()];
                        @if li == page {
                            li .active { a href=(p) { (li) } }
                        } @else {
                            li { a href=(p) { (li) } }
                        }
                    }
                }
            }
        }
    }
    .render_to(buffer);
}
