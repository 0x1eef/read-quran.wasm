# read_quran.rs

read-quran.rs is a Rust library, and WebAssembly module that provides
an interface for both querying and reading the holy book, The Qur'an.
The library is available as a WebAssembly module; that means it can be
used from JavaScript in the web browser.

At compile time, the contents of The Qur'an are embedded into the source
using a compile-time Rust macro - by the name `include_str!`. Since the
WebAssembly module includes The Qur'an within itself, there's no need to
make additional web requests or use other methods to source the contents of The Qur'an.


Two languages are provided: the original Arabic, and an english translation.

## Demo

This library is being used on [0x1eef.github.io/read-quran](https://0x1eef.github.io/read-quran) - a simple web page that allows one to read a randomly selected chapter from The Qur'an.


## Examples

**1. Load the WebAssembly module**

Despite the library itself being written in Rust, the intention is for it
to be used from JavaScript. In order to do this, one has to import a
wasm module - although it relies on modern JavaScript features, it is straight forward:

```javascript
import init, { Quran, Language } from "read-quran.js";

async function run() {
  /* At this point, the web assembly module is not loaded. */
  await init()
  /* At this point, the web assembly module is loaded. */

  /* Create an instance of "Quran" (with Arabic as the language) */
  const quran = new Quran(Language.Arabic);

  /* Print every verse, from every chapter */
  quran.chapters.forEach((chapter) => {
    chapter.verses.forEach((verse) => {
      console.log(chapter.number, ":", verse.number, " ", verse.text);
    });
  });
};

run();
```

**2. Set a language**

An instance of "Quran" represents The Qur'an in a particular language. At
the moment there are two options: `Language.Arabic`, and `Language.English`. For example:

```javascript
/* The Qur'an, in the English language. */
let quran = new Quran(Language.English);

/* The Qur'an, in its original Arabic language */
quran = new Quran(Language.Arabic);
```

**3. Find a random chapter**

An instance of Quran provides a "randomChapter" method that can randomly
select a chapter. For example:


```javascript
const quran = new Quran(Language.Arabic);
const chapter = quran.randomChapter();
console.log("The random chapter number is: ", chapter.number);
```

**4. Find a random verse**

An instance of Chapter provides a "randomVerse" method that can randomly
select a verse from the associated chapter. For example:

```javascript
const quran = new Quran(Language.Arabic);
const chapter = quran.randomChapter();
const verse = chapter.randomVerse();
console.log("The random verse number is: ", chapter.number, ":", verse.number);
```

## LICENSE

The Rust code found within this repository is released according to
the MIT license. See [./LICENSE.txt](LICENSE.txt) for more details.
