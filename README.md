# Junk Tools

This is a bunch of CLI "tools" made out of sheer boredom.

# Junk available:

## Joke

Fetches jokes from [JokeAPI](https://jokeapi.dev/). The API contains about 1300 jokes,
and they range from Programming Jokes, to Puns, to Dark jokes.

The CLI includes a way to select what category of jokes to get and a way to blacklist topics

### Usage

```bash
junk joke --type <all | custom> --category <category> --blacklist <blacklist>
```

## Manga Downloader

Downloads Manga from Manga4Life. Just open the manga and put the link in the args, along with chapters you want and prefered title for the manga, it will download the chapter in `*.cbz` format in the directory of the executable.

You can download multiple chapters using a Comma Seperated List. Furthermore, to specify a range, you can use `..`.

For Example, `1..100,12.5,18.5` will download all chapters from 1 to 100 along with 12.5 and 18.5 chapters. You can download "point chapters", but they are not included in ranges, and you cannot use them at the beginning or the end. Lastly don't put spaces between chapters, it's a bug, it's annoying but it's more annoying to fix it for me.

### Usage

```bash
junk md -t <title> -c <chapters> -l <link>
```

## WIP Scroller

A extremely knock-off version of [CMatrix](https://github.com/abishekvashok/cmatrix). Currently it can do nothing
