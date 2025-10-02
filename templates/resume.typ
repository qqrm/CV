// Resume template rendered through the `resume` function.
#import "@preview/cmarker:0.1.6"

#let resume(lang: "en", role: "", name: none, md_path: none, theme: "light") = [
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

    #let base = "https://qqrm.github.io/CV/"
    #let slug = if lang == "ru" { "ru/" } else { "" }
    #let cv_url = base + slug
    #let cv_path = if md_path == none {
        if lang == "ru" {
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
    #let replaced_md = ("- **CV:** [" + cv_url + "](" + cv_url + ")\n" + lines.join("\n"))
    #cmarker.render(replaced_md)
]
