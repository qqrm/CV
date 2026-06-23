// Resume template rendered through the `resume` function.
#import "@preview/cmarker:0.1.6"
#import "contact_config.typ": CONTACT_URLS

#let resume(
  lang: "en",
  name: none,
  title: none,
  subtitle: none,
  md_path: none,
  theme: "light",
  github_url: CONTACT_URLS.github,
  email_url: CONTACT_URLS.email,
  telegram_url: CONTACT_URLS.telegram,
  linkedin_url: CONTACT_URLS.linkedin,
  site_path: "",
) = [
  #let themes = (
    light: (
      background: rgb("#ffffff"),
      text: rgb("#1f2933"),
      muted: rgb("#6b7280"),
      link: rgb("#0645ad"),
    ),
    dark: (
      background: rgb("#121212"),
      text: rgb("#e5e7eb"),
      muted: rgb("#9ca3af"),
      link: rgb("#9cdcfe"),
    ),
  )
  #let palette = themes.at(theme, default: themes.light)

  #set page(fill: palette.background, margin: (x: 1.45cm, y: 1.35cm))
  #set text(font: ("Libertinus Serif", "Noto Sans"), size: 9.8pt, fill: palette.text)
  #set par(leading: if lang == "ru" { 0.78em } else { 0.65em })
  #show heading.where(level: 3): it => [
    #v(0.85em)
    #it
  ]
  #show link: set text(fill: palette.link)
  #let default_name = if lang == "ru" { "Алексей Беляков" } else { "Alexey Belyakov" }
  #let name = if name == none { default_name } else { name }
  #let default_title = if lang == "ru" {
    "CTO / Head of Engineering / Head of Delivery"
  } else {
    "CTO / Head of Engineering / Head of Delivery"
  }
  #let default_subtitle = if lang == "ru" {
    "Rust/C++ • backend/platform/systems engineering • AI-агенты в разработке"
  } else {
    "Rust/C++ • backend/platform/systems engineering • AI agents in software development"
  }
  #let title = if title == none { default_title } else { title }
  #let subtitle = if subtitle == none { default_subtitle } else { subtitle }
  #let contact_labels = if lang == "ru" {
    (
      github: "GitHub",
      email: "Email",
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
  #let contact_button(label, url) = link(url)[
    #box(
      inset: (x: 10pt, y: 4pt),
      radius: 8pt,
      stroke: 0.75pt + palette.link,
      fill: if theme == "dark" { rgb("#0f1a2a") } else { rgb("#e6f6fc") },
    )[
      #set text(weight: "semibold", size: 10pt)
      #label
    ]
  ]

  #align(center)[
    #set text(size: 20pt, weight: "bold", fill: palette.text)
    #name
  ]
  #v(0.22em)
  #align(center)[
    #set text(size: 10.8pt, weight: "semibold", fill: palette.text)
    #title
  ]
  #v(0.12em)
  #align(center)[
    #set text(size: 8.8pt, fill: palette.muted)
    #subtitle
  ]
  #v(0.5em)
  #align(center)[
    #box(width: 5cm, height: 5cm, radius: 2.5cm, clip: true, stroke: 0.75pt + palette.muted)[
      #image("../content/avatar.jpg", width: 5cm, height: 5cm)
    ]
  ]
  #v(0.35em)
  #align(center)[
    #set text(size: 11pt, fill: palette.link)
    #h(0.3em)
    #contact_button(contact_labels.github, github_url)
    #h(0.35em)
    #contact_button(contact_labels.email, email_url)
    #h(0.35em)
    #contact_button(contact_labels.telegram, telegram_url)
    #h(0.35em)
    #contact_button(contact_labels.linkedin, linkedin_url)
  ]
  #v(0.35em)

  #let find_first_section(lines, i: 0) = if i >= lines.len() {
    none
  } else if lines.at(i).starts-with("## ") {
    i
  } else {
    find_first_section(lines, i: i + 1)
  }

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
  #let replaced_md = trimmed_lines.join("\n")
  #let render_pages(pages, i: 0) = if i >= pages.len() {
    []
  } else [
    #cmarker.render(pages.at(i))
    #if i + 1 < pages.len() [
      #pagebreak()
      #render_pages(pages, i: i + 1)
    ]
  ]
  #render_pages(replaced_md.split("<!-- pdf-pagebreak -->"))
  #v(1.2em)
  #align(center)[
    #set text(size: 10pt, fill: palette.muted)
    #(if lang == "ru" { "Последнее редактирование: " } else { "Last updated: " })
    #datetime.today().display()
  ]
]
