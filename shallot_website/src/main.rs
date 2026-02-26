//! Shallot Website Binary
//!
//! Generates the static website HTML and CSS files.

<<<<<<< HEAD
use shallot_website::{homepage, main_css, retro_hero, rss, showcase_css};
=======
use shallot_website::{homepage, main_css, retro_hero, showcase_css};
>>>>>>> bdf16c3 (Initial commit: Shallot.rs project)
use std::fs;
use std::path::Path;

fn main() {
    // Create output directories
    let out_dir = "output";
    let styles_dir = Path::new(out_dir).join("styles");

    fs::create_dir_all(&styles_dir).expect("Failed to create output directories");

    // Generate main HTML
    let html = homepage();
    fs::write(Path::new(out_dir).join("index.html"), html.into_string())
        .expect("Failed to write index.html");

<<<<<<< HEAD
    // Generate RSS feed
    fs::write(Path::new(out_dir).join("feed.xml"), rss::rss_string())
        .expect("Failed to write feed.xml");

=======
>>>>>>> bdf16c3 (Initial commit: Shallot.rs project)
    // Generate CSS files
    fs::write(styles_dir.join("main.css"), main_css()).expect("Failed to write main.css");

    fs::write(styles_dir.join("retro.css"), retro_hero::retro_css())
        .expect("Failed to write retro.css");

    fs::write(styles_dir.join("showcase.css"), showcase_css())
        .expect("Failed to write showcase.css");

    println!("✅ Website generated successfully in {}/", out_dir);
    println!("📄 Files created:");
    println!("   - {}/index.html", out_dir);
<<<<<<< HEAD
    println!("   - {}/feed.xml", out_dir);
=======
>>>>>>> bdf16c3 (Initial commit: Shallot.rs project)
    println!("   - {}/styles/main.css", out_dir);
    println!("   - {}/styles/retro.css", out_dir);
    println!("   - {}/styles/showcase.css", out_dir);
    println!(
        "\n🚀 Open {}/index.html in your browser to view the website!",
        out_dir
    );
}
