// Resume template rendered through the `resume` function.

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

  #let cv_path = if lang == "ru" { "../cv.ru.md" } else { "../cv.md" }
  #raw(read(cv_path))
]
