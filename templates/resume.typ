#import "@preview/markdown:0.3.1": markdown
#let lang = context().input("lang", "en")
#let role = context().input("role", "Rust Team Lead")
#let name = if lang == "ru" { "Алексей Леонидович Беляков" } else { "Alexey Leonidovich Belyakov" }

#align(center)[= {name}]
#align(center)[*{role}*]
#align(center)[#datetime.today().display()]

#align(center)[
  #box(width: 5cm, height: 5cm, radius: 2.5cm, clip: true)[
    #image("../content/avatar.jpg", width: 5cm, height: 5cm)
  ]
]

#markdown(file("../cv." + lang + ".md"))
