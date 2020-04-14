## Rss Torrent Reader

rtr is a program to assist in parsing and making use of torrent rss feeds.

### Usage

`$ rtr manifest.toml`

Output is printed in the form:

group	title	link

seperated by tabs. Group refers to the name field grouped with a url. Title and link
refer to the rss values of a matching entry.

### Manifest

Ex.

```
[[targets]]
url = "some-url"
name = "music"

[[targets.filters]]
key = "category"
regexp = "pop|rock"

[[targets]]
url = "some-other-url"
name = "ISO"

[[targets.filters]]
key = "title"
regexp = "Linux"
```

The key value for each filter can be one of:

* title
* category
* description
* author

and refers to the rss value to apply the regex filter to.
