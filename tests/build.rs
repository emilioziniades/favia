use fs_extra::dir::{self, CopyOptions};
use mktemp::Temp;
use std::env;
use std::path::Path;

macro_rules! build_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() -> anyhow::Result<()> {
            let test_case = $value;
            let tmpdir = Temp::new_dir()?;
            let input_dir = Path::new(test_case.input_path).canonicalize()?;
            dir::copy(input_dir, &tmpdir, &CopyOptions::new().content_only(true))?;
            env::set_current_dir(&tmpdir)?;
            favia::build(tmpdir.to_path_buf())?;

            for output_html in test_case.expected_html_outputs {
                let output_path = Path::new(&tmpdir.to_path_buf()).join(".favia").join(output_html);
                assert!(output_path.exists(), "{output_path:?} should exist");
            }

            Ok(())
        }
    )*
    }
}

#[derive(Debug)]
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
build_tests! {
    basic: TestCase::new(
            "./examples/basic",
            vec!["index.html"],
       ),
    two_page: TestCase::new(
            "./examples/two-page",
            vec!["index.html", "about.html"]
        ),
    blog: TestCase::new(
            "./examples/blog",
            vec![
                "index.html",
                "blog.html",
                "blog/post-one.html",
                "blog/post-two.html",
                "blog/post-three.html"
            ]
        ),
}
