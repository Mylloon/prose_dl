use scraper::{Html, Selector};

/// Return vector of all the posts of the user
pub async fn get_posts(scheme: String, username: String, domain: String) -> (String, Vec<String>) {
    // Defines the address
    let url = format!("{}://{}.{}", scheme, username, domain);

    // Parse index page: sheme://username.domain
    let document = Html::parse_document(&reqwest::get(&url).await.unwrap().text().await.unwrap());

    // Look at the posts
    let raw_posts = document
        .select(&Selector::parse("section.posts").unwrap())
        .next()
        .unwrap();

    // Get the name of them and push them into the vector
    let mut posts = Vec::new();
    for link in raw_posts.select(&Selector::parse("a").unwrap()) {
        posts.push(
            link.value()
                .attr("href")
                .unwrap()
                .split('/')
                .last()
                .unwrap()
                .to_owned(),
        );
    }

    // Return the vector
    (url, posts)
}
