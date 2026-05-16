use trpl::Html;

async fn page_title(url: &str) -> Option<String> {
    // let response = trpl::get(url).await;
    // let response_text = response.text().await;

    // Chaining with the await keyword
    let response_text = trpl::get(url).await.text().await;

    Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html())
}

// Эквивалентное для компилятора написание функции page_title
// use std::future::Future;
// use trpl::Html;

// fn page_title(url: &str) -> impl Future<Output = Option<String>> {
//     async move {
//         let text = trpl::get(url).await.text().await;
//         Html::parse(&text)
//             .select_first("title")
//             .map(|title| title.inner_html())
//     }
// }

fn main() {}
