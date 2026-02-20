use trpl::{Either, Html};

// main can't be async
fn main() {
    println!("CPU-bound/IO-bound");
    println!("parallelism/concurrency");

    println!("polling to check a future's ready state");
    println!("Future trait, async scope/function, await a future");

    // runtime(here we use tokio crate)
    // it manages the details of executing asynchronous code
    trpl::run(async {
        let url = "https://www.rust-lang.org";
        match page_title(url).await {
            (_, Some(title)) => println!("The title for {url} was {title}"),
            (_, None) => println!("{url} had no title"),
        }
    });

    trpl::run(async {
        let title_fut_1 = page_title("https://www.rust-lang.org");
        let title_fut_2 = page_title("https://rust-lang.org/learn/");
        // select
        let (url, maybe_title) = match trpl::race(title_fut_1, title_fut_2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };
        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title is: '{title}'"),
            None => println!("Its title could not be parsed."),
        }
    });
}

async fn page_title(url: &str) -> (&str, Option<String>) {
    // futures are lazy
    let response = trpl::get(url).await; // await: postfix keyword
    let response_text = response.text().await;
    // or: let response_text = trpl::get(url).await.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html());
    (url, title)
}
/* same as:
fn page_title(url: &str) -> impl Future<Output = Option<String>> {
    async move {
        let text = trpl::get(url).await.text().await;
        Html::parse(&text)
            .select_first("title")
            .map(|title| title.inner_html())
    }
}
 */
