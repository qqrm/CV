// Resume template rendered through the `resume` function.
#import "@preview/cmarker:0.1.6"
#import "contact_config.typ": CONTACT_URLS

#let resume(
  lang: "en",
  name: none,
  md_path: none,
  theme: "light",
  github_url: CONTACT_URLS.github,
  email_url: CONTACT_URLS.email,
  telegram_url: CONTACT_URLS.telegram,
  linkedin_url: CONTACT_URLS.linkedin,
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
  #set text(font: "New Computer Modern", fill: palette.text)
  #show link: set text(fill: palette.link)
  #let default_name = if lang == "ru" { "Алексей Беляков" } else { "Alexey Belyakov" }
  #let name = if name == none { default_name } else { name }
  #let contact_labels = if lang == "ru" {
    (
      github: "GitHub",
      email: "Почта",
      telegram: "Telegram",
      linkedin: "LinkedIn",
    )
  } else {
    (
      github: "GitHub",
      email: "Email",
      telegram: "Telegram",
      linkedin: "LinkedIn",
    )
  }
  #let contact_separator = [
    #h(0.35em)
    #text(fill: palette.muted)[•]
    #h(0.35em)
  ]

  #align(center)[= #name]
  #v(0.25em)
  #align(center)[
    #set text(size: 11pt, fill: palette.link)
    #link(github_url)[contact_labels.github]
    #contact_separator
    #link(email_url)[contact_labels.email]
    #contact_separator
    #link(telegram_url)[contact_labels.telegram]
    #contact_separator
    #link(linkedin_url)[contact_labels.linkedin]
  ]
  #v(0.5em)
  #align(center)[
    #box(width: 5cm, height: 5cm, radius: 2.5cm, clip: true, stroke: 0.75pt + palette.muted)[
      #image("../content/avatar.jpg", width: 5cm, height: 5cm)
    ]
  ]

  #let find_first_section(lines, i: 0) = if i >= lines.len() {
    none
  } else if lines.at(i).starts-with("## ") {
    i
  } else {
    find_first_section(lines, i: i + 1)
  }

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
  #let lines = replaced_md.split("\n")
  #let summary_start = find_first_section(lines)
  #let trimmed_lines = if summary_start == none {
    lines
  } else {
    lines.slice(summary_start)
  }
  #let replaced_md = ("- **CV:** [" + cv_url + "](" + cv_url + ")\n" + trimmed_lines.join("\n"))
  #cmarker.render(replaced_md)
  #v(1.2em)
  #align(center)[
    #set text(size: 10pt, fill: palette.muted)
    #(if lang == "ru" { "Последнее редактирование: " } else { "Last updated: " })
    #datetime.today().display()
  ]
]
