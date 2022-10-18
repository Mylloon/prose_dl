/// Download all the posts
pub async fn download_posts(posts: (String, Vec<String>), dir: &str, download_special_files: bool) {
    // Create folder, silently ignore if already exists
    std::fs::create_dir(&dir).unwrap_or_default();

    // Define client with custom user-agent
    let client = reqwest::Client::builder()
        .user_agent(format!("prose_dl/{}", env!("CARGO_PKG_VERSION")))
        .build()
        .unwrap();

    // Download all the posts
    for post in posts.1 {
        download(&posts.0, dir, post, "md", &client).await;
    }

    // Check if specials files need to be downloaded
    if download_special_files {
        let special_files = [
            (String::from("_readme"), "md"),
            (String::from("_footer"), "md"),
            (String::from("_styles"), "css"),
        ];

        for file in special_files {
            download(&posts.0, dir, file.0, file.1, &client).await;
        }
    }
}

/// Download a file from the raw endpoint
async fn download(
    url: &String,
    output_dir: &str,
    post_name: String,
    extension: &str,
    client: &reqwest::Client,
) {
    // Endpoint name
    let endpoint = "raw";

    let data = client
        .get(format!("{}/{}/{}", url, endpoint, post_name))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    if data != "post not found\n" {
        // Write file with the content
        let mut file =
            std::fs::File::create(format!("{}/{}.{}", output_dir, post_name, extension)).unwrap();
        std::io::Write::write_all(&mut file, data.as_bytes()).unwrap();
    }
}
