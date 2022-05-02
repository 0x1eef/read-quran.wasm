static MAX_SIZE: usize = 114;

use js_sys::Array;
use js_sys::JsString;
use json;
use rand::seq::SliceRandom;
use wasm_bindgen::prelude::*;

#[derive(Clone)]
#[wasm_bindgen]
pub struct Quran {
    _chapters: Vec<Chapter>,
    json_blob: json::JsonValue,
}

#[wasm_bindgen]
impl Quran {
    #[wasm_bindgen(constructor)]
    pub fn new(language: Language) -> Quran {
        let mut quran = Quran {
            _chapters: vec![],
            json_blob: json::parse(&read_json_blob(&language)).unwrap()
        };

        /* Fill "quran.chapters" */
        for number in 1..MAX_SIZE {
            let mut chapter = Chapter {
                number: number,
                _verses: vec![],
            };

            /* Fill "chapter.verses" */
            let verses = &quran.json_blob[number - 1];
            for verse in verses.members() {
                let number = verse[0].as_usize().unwrap();
                let text = JsString::from(verse[1].as_str().unwrap());
                chapter._verses.push(Verse { number, text });
            }
            quran._chapters.push(chapter);
        }

        quran
    }

    #[wasm_bindgen(getter, js_name = "chapters")]
    pub fn chapters(&self) -> Array {
        let chapters = self._chapters.clone();
        chapters.into_iter().map(JsValue::from).collect()
    }

    #[wasm_bindgen(js_name = "randomChapter")]
    pub fn random_chapter(&self) -> Chapter {
        self._chapters
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone()
    }
}

#[derive(Clone)]
#[wasm_bindgen]
pub struct Chapter {
    pub number: usize,
    _verses: Vec<Verse>,
}

#[wasm_bindgen]
impl Chapter {
    #[wasm_bindgen(getter, js_name = "verses")]
    pub fn verses(&self) -> Array {
        let verses = self._verses.clone();
        verses.into_iter().map(JsValue::from).collect()
    }

    #[wasm_bindgen(js_name = "randomVerse")]
    pub fn random_verse(&self) -> Verse {
        self._verses
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone()
    }
}

#[derive(Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct Verse {
    pub number: usize,
    pub text: JsString,
}

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Language {
    Arabic,
    English
}

impl Language {
    pub fn is_english(&self) -> bool {
        match self {
            Language::English => true,
            _ => false
        }
    }

    pub fn is_arabic(&self) -> bool {
        match self {
            Language::Arabic => true,
            _ => false
        }
    }
}

fn read_json_blob(language: &Language) -> String {
    if language == &Language::English {
        include_str!("../../static/quran_en.json").to_string()
    } else {
        include_str!("../../static/quran_ar.json").to_string()
    }
}
