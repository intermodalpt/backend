/*
 * libgit2 "pull" example - shows how to pull remote data into a local branch.
 *
 * Written by the libgit2 contributors
 *
 * To the extent possible under law, the author(s) have dedicated all copyright
 * and related and neighboring rights to this software to the public domain
 * worldwide. This software is distributed without any warranty.
 *
 * You should have received a copy of the CC0 Public Domain Dedication along
 * with this software. If not, see
 * <http://creativecommons.org/publicdomain/zero/1.0/>.
 */

use chrono::{DateTime, Utc};
use git2::Repository;
use std::io::{self, Write};
use std::str;

fn do_fetch<'a>(
    repo: &'a Repository,
    refs: &[&str],
    remote: &'a mut git2::Remote,
) -> Result<git2::AnnotatedCommit<'a>, git2::Error> {
    let mut cb = git2::RemoteCallbacks::new();

    // Print out our transfer progress.
    cb.transfer_progress(|stats| {
        if stats.received_objects() == stats.total_objects() {
            print!(
                "Resolving deltas {}/{}\r",
                stats.indexed_deltas(),
                stats.total_deltas()
            );
        } else if stats.total_objects() > 0 {
            print!(
                "Received {}/{} objects ({}) in {} bytes\r",
                stats.received_objects(),
                stats.total_objects(),
                stats.indexed_objects(),
                stats.received_bytes()
            );
        }
        io::stdout().flush().unwrap();
        true
    });

    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(cb);
    // Always fetch all tags.
    // Perform a download and also update tips
    fo.download_tags(git2::AutotagOption::All);
    println!("Fetching {} for repo", remote.name().unwrap());
    remote.fetch(refs, Some(&mut fo), None)?;

    // If there are local objects (we got a thin pack), then tell the user
    // how many objects we saved from having to cross the network.
    let stats = remote.stats();
    if stats.local_objects() > 0 {
        println!(
            "\rReceived {}/{} objects in {} bytes (used {} local \
             objects)",
            stats.indexed_objects(),
            stats.total_objects(),
            stats.received_bytes(),
            stats.local_objects()
        );
    } else {
        println!(
            "\rReceived {}/{} objects in {} bytes",
            stats.indexed_objects(),
            stats.total_objects(),
            stats.received_bytes()
        );
    }

    let fetch_head = repo.find_reference("FETCH_HEAD")?;
    repo.reference_to_annotated_commit(&fetch_head)
}

fn fast_forward(
    repo: &Repository,
    lb: &mut git2::Reference,
    rc: &git2::AnnotatedCommit,
) -> Result<(), git2::Error> {
    let name = match lb.name() {
        Some(s) => s.to_string(),
        None => String::from_utf8_lossy(lb.name_bytes()).to_string(),
    };
    let msg = format!("Fast-Forward: Setting {} to id: {}", name, rc.id());
    println!("{msg}");
    lb.set_target(rc.id(), &msg)?;
    repo.set_head(&name)?;
    repo.checkout_head(Some(
        git2::build::CheckoutBuilder::default()
            // For some reason the force is required to make the working directory actually get updated
            // I suspect we should be adding some logic to handle dirty working directory states
            // but this is just an example so maybe not.
            .force(),
    ))?;
    Ok(())
}

fn normal_merge(
    repo: &Repository,
    local: &git2::AnnotatedCommit,
    remote: &git2::AnnotatedCommit,
) -> Result<(), git2::Error> {
    let local_tree = repo.find_commit(local.id())?.tree()?;
    let remote_tree = repo.find_commit(remote.id())?.tree()?;
    let ancestor = repo
        .find_commit(repo.merge_base(local.id(), remote.id())?)?
        .tree()?;
    let mut idx =
        repo.merge_trees(&ancestor, &local_tree, &remote_tree, None)?;

    if idx.has_conflicts() {
        println!("Merge conflicts detected...");
        repo.checkout_index(Some(&mut idx), None)?;
        return Ok(());
    }
    let result_tree = repo.find_tree(idx.write_tree_to(repo)?)?;
    // now create the merge commit
    let msg = format!("Merge: {} into {}", remote.id(), local.id());
    let sig = repo.signature()?;
    let local_commit = repo.find_commit(local.id())?;
    let remote_commit = repo.find_commit(remote.id())?;
    // Do our merge commit and set current branch head to that commit.
    let _merge_commit = repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        &msg,
        &result_tree,
        &[&local_commit, &remote_commit],
    )?;
    // Set working tree to match head.
    repo.checkout_head(None)?;
    Ok(())
}

fn do_merge<'a>(
    repo: &'a Repository,
    remote_branch: &str,
    fetch_commit: &git2::AnnotatedCommit<'a>,
) -> Result<(), git2::Error> {
    // 1. do a merge analysis
    let analysis = repo.merge_analysis(&[fetch_commit])?;

    // 2. Do the appropriate merge
    if analysis.0.is_fast_forward() {
        println!("Doing a fast forward");
        // do a fast forward
        let refname = format!("refs/heads/{remote_branch}");
        if let Ok(mut r) = repo.find_reference(&refname) {
            fast_forward(repo, &mut r, fetch_commit)?;
        } else {
            // The branch doesn't exist so just set the reference to the
            // commit directly. Usually this is because you are pulling
            // into an empty repository.
            repo.reference(
                &refname,
                fetch_commit.id(),
                true,
                &format!("Setting {} to {}", remote_branch, fetch_commit.id()),
            )?;
            repo.set_head(&refname)?;
            repo.checkout_head(Some(
                git2::build::CheckoutBuilder::default()
                    .allow_conflicts(true)
                    .conflict_style_merge(true)
                    .force(),
            ))?;
        };
    } else if analysis.0.is_normal() {
        // do a normal merge
        let head_commit = repo.reference_to_annotated_commit(&repo.head()?)?;
        normal_merge(repo, &head_commit, fetch_commit)?;
    } else {
        println!("Nothing to do...");
    }
    Ok(())
}

pub fn pull(
    repo: &Repository,
    remote_name: &str,
    remote_branch: &str,
) -> Result<(), String> {
    let mut remote = repo
        .find_remote(remote_name)
        .map_err(|e| format!("Failed to find remote: {e:?}"))?;
    let fetch_commit = do_fetch(repo, &[remote_branch], &mut remote)
        .map_err(|e| format!("Failed fetch remote: {e:?}"))?;
    do_merge(repo, remote_branch, &fetch_commit)
        .map_err(|e| format!("Failed to merge: {e:?}"))
}

/// # Panics
///
/// Will panic on invalid commit timestamps
pub fn head_date(repo: &Repository) -> Result<DateTime<Utc>, git2::Error> {
    // Get the HEAD reference
    let head = repo.head()?;
    // Get the commit the HEAD reference points to
    let head_commit = head.peel_to_commit()?;
    // Get the commit time (as a UNIX timestamp) and convert it to a DateTime
    let commit_time = head_commit.time().seconds();
    let commit_date = chrono::DateTime::from_timestamp(commit_time, 0)
        .expect("Commit with invalid date");

    Ok(commit_date)
}

pub fn update_repo(
    url: &str,
    path: &str,
    remote_name: &str,
    remote_branch: &str,
) -> Result<DateTime<Utc>, String> {
    match Repository::open(path) {
        Ok(repo) => {
            println!("Pulling {url} into {path}...");
            pull(&repo, remote_name, remote_branch)?;

            let commit_date = head_date(&repo).map_err(|e| e.to_string())?;
            println!("Last commit date: {commit_date}");
            Ok(commit_date)
        }
        Err(e) => {
            if e.class() == git2::ErrorClass::Os {
                println!("Cloning {url} into {path}...");
                let repo = match Repository::clone(url, path) {
                    Ok(repo) => Ok(repo),
                    Err(e) => Err(format!("failed to clone: {e}")),
                }?;

                let commit_date =
                    head_date(&repo).map_err(|e| e.to_string())?;
                Ok(commit_date)
            } else {
                println!("failed to open: {e}");
                Err(e.to_string())
            }
        }
    }
}
