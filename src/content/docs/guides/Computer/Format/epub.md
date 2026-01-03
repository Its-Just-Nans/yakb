---
title: Epub
description: epub format
---

## Inside the epub

epub files are zip files which contains specifics files

```txt
├── META-INF/
│   └── container.xml
├── EPUB/
│   ├── ... (content files/folders: xhtml, css, image, fonts)
│   └── package.opf
└── mimetype
```

Note that the folder `EPUB/` is optional, you can directly use a `package.opf` and your files

## Content of the zip

- `mimetype` it just contains the epub mimetype

  ```txt
  application/epub+zip
  ```

- `META-INF/container.xml`
  - an `xml` file which contains the link to the `package.opf`
- `EPUB/package.opf`
  - an `xml` file which contains four child:
    - `<metadata>`: informations about the epub (e.g. title, language, isbn, creator..)
    - `<manifest>`: list all the content files available in the epub
    - `<spine>`: all the content in reading order
    - `<guide>`: (optional) to identify fundamental structural components of the book

## Format and specification

- <https://www.w3.org/publishing/epub3/>

- EPUB 2 provides all the formatting and layout capabilities of XHTML 1.1 and CSS2.
- EPUB 3 officially became a standard in late October 2011.The recent EPUB 3 defines profiles of HTML5, SVG, and CSS for use for enhanced Publications.
