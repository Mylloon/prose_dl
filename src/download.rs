/// Download all the posts from the raw endpoint
pub async fn download_posts(posts: (String, Vec<String>), dir: String) {
    // Create folder, silently ignore if already exists
    std::fs::create_dir(&dir).unwrap_or_default();

    // Endpoint name
    let endpoint = "raw";

    for post in posts.1 {
        let mut file = std::fs::File::create(format!("{}/{}.md", dir, post)).unwrap();
        std::io::Write::write_all(
            &mut file,
            reqwest::get(format!("{}/{}/{}", posts.0, endpoint, post))
                .await
                .unwrap()
                .text()
                .await
                .unwrap()
                .as_bytes(),
        )
        .unwrap();
    }
}
