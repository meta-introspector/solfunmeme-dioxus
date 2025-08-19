use git2::{Repository, Error};
use std::path::Path;

pub fn clone_repository(url: &str, path: &Path) -> Result<Repository, Error> {
    Repository::clone(url, path)
}

pub fn open_repository(path: &Path) -> Result<Repository, Error> {
    Repository::open(path)
}

pub fn pull_repository(repo: &Repository) -> Result<(), Error> {
    // For simplicity, this is a basic pull. Real-world scenarios need more robust handling.
    let mut remote = repo.find_remote("origin")?;
    remote.fetch(&[], None, None)?;
    let fetch_head = repo.find_reference("FETCH_HEAD")?;
    let commit = repo.reference_to_annotated_commit(&fetch_head)?;
    repo.merge(&[&commit], None, None)?;
    Ok(())
}

pub fn commit_changes(repo: &Repository, message: &str) -> Result<(), Error> {
    let mut index = repo.index()?;
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let signature = repo.signature()?;
    let parent_commit = find_last_commit(repo)?;
    repo.commit(Some("HEAD"), &signature, &signature, message, &tree, &[&parent_commit])?;
    Ok(())
}

pub fn push_changes(repo: &Repository) -> Result<(), Error> {
    let mut remote = repo.find_remote("origin")?;
    remote.push(&["refs/heads/main:refs/heads/main"], None, None)
}

fn find_last_commit(repo: &Repository) -> Result<git2::Commit, Error> {
    let obj = repo.head()?.resolve()?.peel(git2::ObjectType::Commit)?;
    obj.into_commit().map_err(|_| Error::from_str("Couldn't find commit"))
}
