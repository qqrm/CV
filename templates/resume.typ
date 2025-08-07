#let file = read
#let markdown = (src) => raw(src)
#let resume = (lang: str, role: str) => {
  let name = if lang == "ru" { "Алексей Леонидович Беляков" } else { "Alexey Leonidovich Belyakov" }
  align(center)[= name]
  align(center)[*role*]
  align(center)[#datetime.today().display()]

  align(center)[
    box(width: 5cm, height: 5cm, radius: 2.5cm, clip: true)[
      image("content/avatar.jpg", width: 5cm, height: 5cm)
    ]
  ]

  markdown(file("/cv." + lang + ".md"))
}
