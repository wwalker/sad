pub fn udiff(name: &str, before: &str, after: &str) -> String {
  let delim = '\n';
  let before_split = before.split_terminator(delim).collect::<Vec<&str>>();
  let after_split = after.split_terminator(delim).collect::<Vec<&str>>();
  let diff = difflib::unified_diff(&before_split, &after_split, name, name, "", "", 3);
  let prefix = vec![
    format!("\ndiff --git {} {}", name, name),
    format!("--- {}", name),
    format!("+++ {}", name),
  ];
  let body = diff
    .into_iter()
    .map(|line| match line.chars().next() {
      Some(ch) if ch == '@' => String::from(line.trim()),
      _ => line,
    })
    .skip(2);
  let end = vec![String::from("\n")];
  let print = itertools::chain(itertools::chain(prefix, body), end);
  itertools::join(print, "\n")
}
