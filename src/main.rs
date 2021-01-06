use clap::{App, Arg};
use git2::Repository;
use std::fs;

struct Cli {
    title: String,
}

fn main() {
    let matches = App::new("hugo-new")
        .version("1.0")
        .about("Make faster your blogging.")
        .author("Created by shiomiya.")
        .arg(
            Arg::new("new")
                .about("input post title")
                .short('n')
                .long("new")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    if let Some(title) = matches.value_of("new") {
        // git_checkout(title).unwrap_or_else(|e| panic!("Error: failed to checkout branch {}.", e));
        create_post(title).unwrap_or_else(|e| panic!("Error: failed to create new post {}.", e));
    }
}

fn create_post(title: &str) -> std::io::Result<()> {
    fs::create_dir(title).unwrap_or_else(|e| panic!("Error: creating dir: {}", e));
    let path_to = format!("{}/index.md", title);
    fs::File::create(path_to).unwrap_or_else(|e| panic!("Error: creating file: {}", e));
    Ok(())
}

fn git_checkout(title: &str) -> std::io::Result<()> {
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("Error: failed to open site repository: {}", e),
    };

    let head = repo.head().unwrap();
    let oid = head.target().unwrap();
    let commit = repo.find_commit(oid).unwrap();

    repo.branch(title, &commit, false)
        .unwrap_or_else(|e| panic!("Error: failed to create branch: {}", e));

    let obj = repo
        .revparse_single(&("refs/heads/".to_owned() + title))
        .unwrap();

    repo.checkout_tree(&obj, None)
        .unwrap_or_else(|e| panic!("Error: failed to checkout branch to {}.\n {}", title, e));

    repo.set_head(&("refs/heads/".to_owned() + title))
        .unwrap_or_else(|e| panic!("Error: failed to set head to {}. \n {}", title, e));

    Ok(())
}
