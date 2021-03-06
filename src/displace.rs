use super::argparse::{Action, Options};
use super::errors::*;
use super::udiff;
use async_std::{
  fs::{self, Metadata},
  path::PathBuf,
};
use either::Either::*;
use uuid::Uuid;

fn replace(before: &str, opts: &Options) -> String {
  match &opts.pattern {
    Left(ac) => ac.replace_all(&before, &[opts.replace.as_str()]),
    Right(re) => String::from(re.replace_all(&before, opts.replace.as_str())),
  }
}

async fn safe_write(canonical: &PathBuf, meta: &Metadata, text: &str) -> SadResult<()> {
  let uuid = Uuid::new_v4().to_simple().to_string();
  let mut file_name = canonical
    .file_name()
    .and_then(|s| s.to_str())
    .map(String::from)
    .ok_or_else(|| Failure::Simple(format!("Bad file name - {}", canonical.to_string_lossy())))?;
  file_name.push_str("___");
  file_name.push_str(&uuid);
  let backup = canonical.with_file_name(file_name);
  fs::rename(&canonical, &backup).await.halp()?;
  fs::write(&canonical, text).await.halp()?;
  fs::set_permissions(&canonical, meta.permissions())
    .await
    .halp()?;
  fs::remove_file(&backup).await.halp()?;
  Ok(())
}

pub async fn displace(path: PathBuf, opts: &Options) -> SadResult<String> {
  let name = String::from(path.to_string_lossy());
  let canonical = fs::canonicalize(&path).await.halp()?;
  let meta = fs::metadata(&canonical).await.halp()?;
  if !meta.is_file() {
    let msg = format!("Not a file - {}", canonical.to_string_lossy());
    return Err(Failure::Simple(msg));
  }
  let before = fs::read_to_string(&canonical).await.halp()?;
  let after = replace(&before, opts);
  if before == after {
    Ok(String::from(""))
  } else {
    let print = match opts.action {
      Action::Diff => udiff::udiff(&name, &before, &after),
      Action::Write => {
        safe_write(&canonical, &meta, &after).await?;
        format!("{}\n", name)
      }
    };
    Ok(print)
  }
}
