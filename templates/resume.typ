// Resume template rendered through the `resume` function.
#import "@preview/cmarker:0.1.6"

#let resume(lang: "en", role: "", name: none, md_path: none) = [
  #set text(font: "Latin Modern Roman")
  #let default_name = if lang == "ru" { "Алексей Леонидович Беляков" } else { "Alexey Leonidovich Belyakov" }
  #let name = if name == none { default_name } else { name }

  #align(center)[= #name]
  #if role != "" [#align(center)[*#role*]]
  #align(center)[#datetime.today().display()]

  #align(center)[
    #box(width: 5cm, height: 5cm, radius: 2.5cm, clip: true)[
      #image("../content/avatar.jpg", width: 5cm, height: 5cm)
    ]
  ]

    #let slugs = (
      "Engineering Manager": "em",
      "Руководитель разработки": "em",
      "Инженерный менеджер": "em",
    )
    #let resume_paths = (
      "em": (
        en: "../profiles/resume/en/RESUME_EM.MD",
        ru: "../profiles/resume/ru/RESUME_EM_RU.MD",
      ),
    )
    #let base = "https://qqrm.github.io/CV/"
    #let slug_key = if role == "" { "" } else { slugs.at(role, default: role) }
    #let resume_entry = if slug_key == "" { none } else { resume_paths.at(slug_key, default: none) }
    #let using_resume = md_path == none and resume_entry != none
    #let base_slug = if using_resume { "resume/" } else { "" }
    #let slug = if role == "" {
        if lang == "ru" { base_slug + "ru/" } else { base_slug }
      } else {
        let s = slug_key + "/"
        if lang == "ru" { base_slug + s + "ru/" } else { base_slug + s }
      }
    #let cv_url = base + slug

    #let cv_path = if md_path == none {
        if using_resume {
          if lang == "ru" { resume_entry.at("ru") } else { resume_entry.at("en") }
        } else if lang == "ru" {
          "../profiles/cv/ru/CV_RU.MD"
        } else {
          "../profiles/cv/en/CV.MD"
        }
      } else {
        md_path
      }
    #let raw_md = read(cv_path)
    #let replaced_md = raw_md.replace("{NAME}", name)
    #let lines = replaced_md.split("\n").slice(5)
    #let link_label = if using_resume {
        if lang == "ru" { "Резюме" } else { "Resume" }
      } else {
        "CV"
      }
    #let replaced_md = ("- **" + link_label + ":** [" + cv_url + "](" + cv_url + ")\n" + lines.join("\n"))
    #cmarker.render(replaced_md)
]
