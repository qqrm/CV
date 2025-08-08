// Resume template rendered through the `resume` function.
#import "@preview/cmarker:0.1.6"

#let resume(lang: "en", role: "") = [
  #let name = if lang == "ru" { "Алексей Леонидович Беляков" } else { "Alexey Leonidovich Belyakov" }

  #align(center)[= {name}]
  #if role != "" [#align(center)[*{role}*]]
  #align(center)[#datetime.today().display()]

  #align(center)[
    #box(width: 5cm, height: 5cm, radius: 2.5cm, clip: true)[
      #image("../content/avatar.jpg", width: 5cm, height: 5cm)
    ]
  ]

  #let cv_path = if lang == "ru" { "../CV_RU.MD" } else { "../CV.MD" }
  #cmarker.render(read(cv_path))
]
