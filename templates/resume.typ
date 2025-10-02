// Resume template rendered through the `resume` function.
#import "@preview/cmarker:0.1.6"

#let resume(
  lang: "en",
  role: "",
  name: none,
  md_path: none,
  theme: "light",
  link_label: none,
) = [
  #let themes = (
    light: (
      background: rgb("#ffffff"),
      text: rgb("#1f2933"),
      muted: rgb("#7b8794"),
      link: rgb("#0645ad"),
    ),
    dark: (
      background: rgb("#121212"),
      text: rgb("#e0e0e0"),
      muted: rgb("#7b8794"),
      link: rgb("#9cdcfe"),
    ),
  )
  #let palette = themes.at(theme, default: themes.light)

  #set page(fill: palette.background)
  #set text(font: "Latin Modern Roman", fill: palette.text)
  #show link: set text(fill: palette.link)
  #let default_name = if lang == "ru" { "Алексей Леонидович Беляков" } else { "Alexey Leonidovich Belyakov" }
  #let name = if name == none { default_name } else { name }

  #align(center)[= #name]
  #if role != "" [#align(center)[*#role*]]
  #align(center)[#datetime.today().display()]

  #align(center)[
    #box(width: 5cm, height: 5cm, radius: 2.5cm, clip: true, stroke: 0.75pt + palette.muted)[
      #image("../content/avatar.jpg", width: 5cm, height: 5cm)
    ]
  ]

    #let slugs = (
      "Engineering Manager": "em",
      "Руководитель разработки": "em",
      "Инженерный менеджер": "em",
    )
    #let base = "https://qqrm.github.io/CV/"
    #let slug_key = if role == "" { none } else { slugs.at(role, default: role) }
    #let slug = if slug_key == none {
        if lang == "ru" { "ru/" } else { "" }
      } else {
        let slug_str = slug_key + "/"
        if lang == "ru" { slug_str + "ru/" } else { slug_str }
      }
    #let cv_url = base + slug

    #let default_cv_path = if lang == "ru" {
        "../profiles/cv/ru/CV_RU.MD"
      } else {
        "../profiles/cv/en/CV.MD"
      }
    #let cv_path = if md_path == none { default_cv_path } else { md_path }
    #let raw_md = read(cv_path)
    #let replaced_md = raw_md.replace("{NAME}", name)
    #let lines = replaced_md.split("\n").slice(5)
    #let default_link = if lang == "ru" { "Резюме" } else { "CV" }
    #let resolved_label = if link_label == none { default_link } else { link_label }
    #let replaced_md = ("- **" + resolved_label + ":** [" + cv_url + "](" + cv_url + ")\n" + lines.join("\n"))
    #cmarker.render(replaced_md)
]
