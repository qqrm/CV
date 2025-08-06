#import "@preview/markdown:0.2.0": markdown
#let lang = sys.inputs.at("lang", default: "en")
#let role = sys.inputs.at("role", default: "Rust Team Lead")

#let resume(lang, role) = [
  #align(center)[= Alexey Leonidovich Belyakov]
  #align(center)[*{role}*]
  #align(center)[#datetime.today().display()]
  #align(center)[
    #box(width: 5cm, height: 5cm, radius: 2.5cm, clip: true)[
      #image("avatar.jpg", width: 5cm, height: 5cm)
    ]
  ]
  #markdown(file("cv." + lang + ".md"))
]

#resume(lang, role)
