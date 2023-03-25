#[test]
fn test_new_command() -> favia::Result<()> {
    let tmpdir = mktemp::Temp::new_dir()?;
    let project_name = "test_favia_project";
    let project_dir = tmpdir.join(project_name);

    // ensure that starter project builds correctly
    favia::new(&tmpdir, project_name.into())?;
    favia::build(&project_dir)?;

    // ensure css file generated correctly
    let stylesheet = project_dir.join(".favia").join("styles.css");
    assert!(stylesheet.exists());

    // ensure stylesheet has been populated
    let stylesheet = std::fs::read_to_string(stylesheet)?;
    assert!(stylesheet.contains("flex"));
    assert!(stylesheet.contains("flex-col"));
    assert!(stylesheet.contains("content-center"));
    assert!(stylesheet.contains("bg-slate-500"));

    Ok(())
}
