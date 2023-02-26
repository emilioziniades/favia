use fs_extra::dir::{self, CopyOptions};
use mktemp::Temp;
use std::path::Path;

#[test]
fn test_basic() -> anyhow::Result<()> {
    build_tests(TestCase::new("./examples/basic", vec!["index.html"]))
}

#[test]
fn test_two_page() -> anyhow::Result<()> {
    build_tests(TestCase::new(
        "./examples/two-page",
        vec!["index.html", "about.html"],
    ))
}

#[test]
fn test_blog() -> anyhow::Result<()> {
    build_tests(TestCase::new(
        "./examples/blog",
        vec![
            "index.html",
            "blog/index.html",
            "blog/post-one.html",
            "blog/post-two.html",
            "blog/post-three.html",
        ],
    ))
}

fn build_tests(test_case: TestCase) -> anyhow::Result<()> {
    let tmpdir = Temp::new_dir()?;
    let input_dir = Path::new(test_case.input_path).canonicalize()?;
    dir::copy(input_dir, &tmpdir, &CopyOptions::new().content_only(true))?;
    favia::build(tmpdir.to_path_buf())?;

    for output_html in test_case.expected_html_outputs {
        let output_path = Path::new(&tmpdir.to_path_buf())
            .join(".favia")
            .join(output_html);
        assert!(output_path.exists(), "{output_path:?} should exist");
    }

    Ok(())
}

struct TestCase<'a> {
    input_path: &'a str,
    expected_html_outputs: Vec<&'a str>,
}

impl<'a> TestCase<'a> {
    fn new(input_path: &'a str, expected_html_outputs: Vec<&'a str>) -> Self {
        Self {
            input_path,
            expected_html_outputs,
        }
    }
}
