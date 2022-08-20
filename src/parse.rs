use scraper::{Html, Selector};

/// Return vector of all the posts of the user
pub async fn get_posts(scheme: String, username: String, domain: String) -> Vec<String> {
    // Parse index page: sheme://username.domain
    let document = Html::parse_document(
        &reqwest::get(format!("{}://{}.{}", scheme, username, domain))
            .await
            .unwrap()
            .text()
            .await
            .unwrap(),
    );

    // Look at the posts
    let raw_posts = document
        .select(&Selector::parse("section.posts").unwrap())
        .next()
        .unwrap();

    // Get the name of them and push them into the vector
    let mut posts = Vec::new();
    for link in raw_posts.select(&Selector::parse("a").unwrap()) {
        posts.push(link.inner_html());
    }

    // Return the vector
    posts
}
