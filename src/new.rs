use std::{
    fs::{self, File},
    io::Write,
    path,
};

use log::{debug, info};

use crate::Result;

pub fn new(directory: &path::Path, project_name: String) -> Result<()> {
    let project_folder = directory.join(&project_name);
    info!("creating new favia project: {}", project_folder.display());
    fs::create_dir(&project_folder)?;

    let templates_folder = project_folder.join("templates");
    fs::create_dir(&templates_folder)?;
    debug!("created folder {}", templates_folder.display());

    let base_template = templates_folder.join("base.html");
    let mut base_template_file = File::create(&base_template)?;
    base_template_file.write_all(
        b"
<html lang=\"en\">
  <head>
    <title>{% block title %}{% endblock title %} - My Favia Website</title>
  <link href=\"styles.css\" rel=\"stylesheet\" />
  </head>
  <body>
    <div class=\"flex flex-col content-center bg-slate-500 max-w-100\">{% block content %}{% endblock content %}</div>
  </body>
</html>",
    )?;
    debug!("created file {}", base_template.display());

    let index_template = templates_folder.join("index.html");
    let mut index_template_file = File::create(&index_template)?;
    index_template_file.write_all(
        b"
{% extends \"base.html\" %} 

{% block title %} {{ fm.page_title }} {% endblock title %}

{% block content %}
<h1>{{ fm.page_title }}</h1>
{{ content | safe }}
{% endblock content %}",
    )?;
    debug!("created file {}", index_template.display());

    let content_folder = project_folder.join("content");
    fs::create_dir(&content_folder)?;
    debug!("created folder {}", content_folder.display());

    let index_content = content_folder.join("index.md");
    let mut index_content_file = File::create(&index_content)?;
    index_content_file.write_all(
        b"
+++
page_title = \"Favia\"
+++
Welcome to your favia project!",
    )?;
    debug!("created file {}", index_content.display());

    info!("new project ready, run `cd {project_name} && favia develop`");

    Ok(())
}
