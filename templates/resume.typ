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
      "Team Lead": "tl",
      "Engineering Manager": "em",
      "Head of Development": "hod",
      "Product Manager": "pm",
      "Tech Lead": "tech",
    )
    #let base = "https://qqrm.github.io/CV/"
    #let slug = if role == "" {
        if lang == "ru" { "ru/" } else { "" }
      } else {
        let s = slugs.at(role, default: role) + "/"
        if lang == "ru" { s + "ru/" } else { s }
      }
    #let cv_url = base + slug

    #let cv_path = if md_path == none {
        if lang == "ru" { "../CV_RU.MD" } else { "../CV.MD" }
      } else { md_path }
    #let raw_md = read(cv_path)
    #let replaced_md = raw_md.replace("{NAME}", name)
    #let lines = replaced_md.split("\n").slice(5)
    #let replaced_md = ("- **CV:** [" + cv_url + "](" + cv_url + ")\n" + lines.join("\n"))
    #cmarker.render(replaced_md)
]
