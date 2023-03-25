use fs_extra::dir::{self, CopyOptions};
use mktemp::Temp;
use std::path::Path;

use favia::Result;

#[test]
fn test_basic() -> Result<()> {
    test_build_command(TestCase::new("./examples/basic", vec!["index.html"]))
}

#[test]
fn test_two_page() -> Result<()> {
    test_build_command(TestCase::new(
        "./examples/two-page",
        vec!["index.html", "about/index.html"],
    ))
}

#[test]
fn test_mix_nested_flat_structure() -> Result<()> {
    test_build_command(TestCase::new(
        "./examples/mix-nested-flat-structure",
        vec![
            "index.html",
            "about/index.html",
            "contact/index.html",
            "testimonials/index.html",
            "projects/index.html",
        ],
    ))
}

#[test]
fn test_blog() -> Result<()> {
    test_build_command(TestCase::new(
        "./examples/blog",
        vec![
            "index.html",
            "blog/index.html",
            "blog/post-one/index.html",
            "blog/post-two/index.html",
            "blog/post-three/index.html",
        ],
    ))
}

fn test_build_command(test_case: TestCase) -> Result<()> {
    let tmpdir = Temp::new_dir()?;
    let input_dir = Path::new(test_case.input_path).canonicalize()?;
    dir::copy(input_dir, &tmpdir, &CopyOptions::new().content_only(true)).unwrap();
    favia::build(&tmpdir)?;

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
