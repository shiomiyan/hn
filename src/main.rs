use clap::{App, Arg};
use git2::Repository;
use std::io::Result;
use std::process::{Command, Stdio};

fn main() {
    let matches = App::new("rugo")
        .version("0.1")
        .about("Make faster your blogging.")
        .author("Created by shiomiya.")
        .arg(
            Arg::new("INPUT")
                .about("input post title")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("edit")
                .about("open with Vim")
                .long("edit")
                .short('e'),
        )
        .get_matches();

    if let Some(title) = matches.value_of("INPUT") {
        git_checkout(title).unwrap_or_else(|e| panic!("Error: failed to checkout branch {}.", e));
        create_post(title).unwrap_or_else(|e| panic!("Error: failed to create new post {}.", e));
        // NOTE: code for not being able to open Vim
        println!("created on `content/posts/{}/index.md`", title);
    }

    if let Some(title) = matches.value_of("edit") {
        edit(title).unwrap_or_else(|e| panic!("Error: failed to open with Vim {}", e));
    }
}

fn create_post(title: &str) -> Result<()> {
    let cmdargs = format!("posts/{}/index.md", title);
    Command::new("hugo")
        .arg("new")
        .arg(cmdargs)
        .stdout(Stdio::null())
        .spawn()
        .expect("Can't run command `hugo new`");
    Ok(())
}

fn git_checkout(title: &str) -> Result<()> {
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

fn edit(title: &str) -> Result<()> {
    let path = format!("content/posts/{}/index.md", title);
    Command::new("vi")
        .arg(path)
        .stdout(Stdio::piped())
        .spawn()?
        .wait()
        .expect("Error: failed to start command Vim");
    Ok(())
}
