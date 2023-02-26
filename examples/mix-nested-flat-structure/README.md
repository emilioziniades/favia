This example shows how you can either have flat or nested file structures for `content` and `templates` and it just works.

- flat content, flat templates: `content/about.md` and `templates/about.html` builds `.favia/about/index.html`
- nested content, flat templates: `content/contact/index.md` and `templates/contact.html` builds `.favia/contact/index.html`
- flat content, nested templates: `content/testimonials.md` and `templates/testimonials/index.html` builds `.favia/testimonials/index.html`
- nested content, nested templates: `content/projects/index.md` and `templates/projects/index.html` builds `.favia/projects/index.html`
