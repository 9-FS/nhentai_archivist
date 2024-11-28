// Copyright (c) 2024 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.
use crate::error::*;


/// # Summary
/// Checks if a directory is empty and if so attempts to delete it or forwards any error. If the directory is not empty, does nothing and silently returns Ok(()). This function is needed because std::io::ErrorKind::NotADirectory is still unstable and can't be matched against. Otherwise tokio::fs::remove_dir would be enough and errors logged depending on the error kind.
///
/// # Arguments
/// - `path`: the path to the directory to check and delete if empty
///
/// # Returns
/// - Ok(()) if the directory was empty and successfully deleted or if the directory was not empty, Err(std::io::Error) if an error occurred while deleting the directory
pub async fn remove_only_empty_dir(path: String) -> Result<(), RemoveOnlyEmptyDirError>
{
    match tokio::fs::read_dir(&path).await // read directory
    {
        Ok(mut o) => // reading directory succeeded
        {
            match o.next_entry().await // read first entry
            {
                Ok(o) => // reading first entry succeeded
                {
                    if !o.is_none() {return Ok(());} // if directory is not empty: ignore and return
                }
                Err(e) => return Err(RemoveOnlyEmptyDirError::StdIo {path: path, source: e}), // reading first entry failed
            }
        },
        Err(e) => return Err(RemoveOnlyEmptyDirError::StdIo {path: path, source: e}), // reading directory failed
    }


    if let Err(e) = tokio::fs::remove_dir(&path).await // attempt to delete empty directory
    {
        return Err(RemoveOnlyEmptyDirError::StdIo {path: path, source: e}); // deleting directory failed
    }

    return Ok(());
}