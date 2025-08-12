// Resume template rendered through the `resume` function.
#import "@preview/cmarker:0.1.6"

#let resume(lang: "en", role: "", name: none) = [
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

  #align(center)[#link("https://qqrm.github.io/CV/")[https://qqrm.github.io/CV/]]

  #let cv_path = if lang == "ru" { "../CV_RU.MD" } else { "../CV.MD" }
  #let raw_md = read(cv_path)
  #let replaced_md = raw_md.replace("{NAME}", name)
  #let replaced_md = replaced_md.split("\n").slice(5).join("\n")
  #cmarker.render(replaced_md)
]
