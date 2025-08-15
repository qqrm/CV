// Resume template rendered through the `resume` function.
#import "@preview/cmarker:0.1.6"

#let resume(lang: "en", role: "", role_key: none, name: none, md_path: none) = [
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

  #let base_cv_url = "https://qqrm.github.io/CV/"
  #let lang_part = if lang == "ru" { "ru/" } else { "" }
  #let cv_url = if role_key == none {
      base_cv_url + lang_part
    } else {
      base_cv_url + "resume/" + role_key + "/" + lang_part
    }

  #let cv_path = if md_path == none {
      if lang == "ru" { "../CV_RU.MD" } else { "../CV.MD" }
    } else { md_path }
  #let raw_md = read(cv_path)
  #let replaced_md = raw_md.replace("{NAME}", name)
  #let replaced_md = replaced_md.split("\n").slice(5).join("\n")
  #let replaced_md = if role_key == none {
      replaced_md
    } else {
      "- **CV:** [" + cv_url + "](" + cv_url + ")\n" + replaced_md
    }
  #cmarker.render(replaced_md)
]
