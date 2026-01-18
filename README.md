# Rust Template Benchmark

This benchmark tries to compare different template engines in Rust.
We consider both precompiled and dynamic engines, which of course have wildly different runtime
performances.

**Beware:** One engine might be faster than the other.
But if the other engine is more ergonomical to work with,
e.g. because you have no re-compilation times, then it might still be the better fit for you.
Only you can decide which characteristics of a template engine
(runtime speed, simplicity of programming, …) are important to you.

Currently, we compare 17 different template engines.
All of them are well maintained, mature and perform HTML escaping automatically.

| Crate          | Docs                                            | Repo                               | Code         | Recent<br>Downloads                  | Github<br>Stars                              | Contrib-<br>utors                            | Recent<br>Commits                          |
| -------------- | ----------------------------------------------- | ---------------------------------- | -------------| ------------------------------------ | -------------------------------------------- | -------------------------------------------- | ------------------------------------------ |
| [askama]       | [![][docs-img-askama]][docs-askama]             | [![][img-repo]][repo-askama]       | pre-compiled | [![][dl-askama]][askama]             | [![][stars-askama]][repo-askama]             | [![][contr-askama]][repo-askama]             | [![][act-askama]][repo-askama]             |
| [dioxus]       | [![][docs-img-dioxus]][docs-dioxus]             | [![][img-repo]][repo-dioxus]       | ssr          | [![][dl-dioxus]][dioxus]             | [![][stars-dioxus]][repo-dioxus]             | [![][contr-dioxus]][repo-dioxus]             | [![][act-dioxus]][repo-dioxus]             |
| [handlebars]   | [![][docs-img-handlebars]][docs-handlebars]     | [![][img-repo]][repo-handlebars]   | interpreted  | [![][dl-handlebars]][handlebars]     | [![][stars-handlebars]][repo-handlebars]     | [![][contr-handlebars]][repo-handlebars]     | [![][act-handlebars]][repo-handlebars]     |
| [horrorshow]   | [![][docs-img-horrorshow]][docs-horrorshow]     | [![][img-repo]][repo-horrorshow]   | pre-compiled | [![][dl-horrorshow]][horrorshow]     | [![][stars-horrorshow]][repo-horrorshow]     | [![][contr-horrorshow]][repo-horrorshow]     | [![][act-horrorshow]][repo-horrorshow]     |
| [hypertext]    | [![][docs-img-hypertext]][docs-hypertext]       | [![][img-repo]][repo-hypertext]    | pre-compiled | [![][dl-hypertext]][hypertext]       | [![][stars-hypertext]][repo-hypertext]       | [![][contr-hypertext]][repo-hypertext]       | [![][act-hypertext]][repo-hypertext]       |
| [leptos]       | [![][docs-img-leptos]][docs-leptos]             | [![][img-repo]][repo-leptos]       | ssr          | [![][dl-leptos]][leptos]             | [![][stars-leptos]][repo-leptos]             | [![][contr-leptos]][repo-leptos]             | [![][act-leptos]][repo-leptos]             |
| [liquid]       | [![][docs-img-liquid]][docs-liquid]             | [![][img-repo]][repo-liquid]       | interpreted  | [![][dl-liquid]][liquid]             | [![][stars-liquid]][repo-liquid]             | [![][contr-liquid]][repo-liquid]             | [![][act-liquid]][repo-liquid]             |
| [markup]       | [![][docs-img-markup]][docs-markup]             | [![][img-repo]][repo-markup]       | pre-compiled | [![][dl-markup]][markup]             | [![][stars-markup]][repo-markup]             | [![][contr-markup]][repo-markup]             | [![][act-markup]][repo-markup]             |
| [maud]         | [![][docs-img-maud]][docs-maud]                 | [![][img-repo]][repo-maud]         | pre-compiled | [![][dl-maud]][maud]                 | [![][stars-maud]][repo-maud]                 | [![][contr-maud]][repo-maud]                 | [![][act-maud]][repo-maud]                 |
| [minijinja]    | [![][docs-img-minijinja]][docs-minijinja]       | [![][img-repo]][repo-minijinja]    | interpreted  | [![][dl-minijinja]][minijinja]       | [![][stars-minijinja]][repo-minijinja]       | [![][contr-minijinja]][repo-minijinja]       | [![][act-minijinja]][repo-minijinja]       |
| [ructe]        | [![][docs-img-ructe]][docs-ructe]               | [![][img-repo]][repo-ructe]        | pre-compiled | [![][dl-ructe]][ructe]               | [![][stars-ructe]][repo-ructe]               | [![][contr-ructe]][repo-ructe]               | [![][act-ructe]][repo-ructe]               |
| [sailfish]     | [![][docs-img-sailfish]][docs-sailfish]         | [![][img-repo]][repo-sailfish]     | pre-compiled | [![][dl-sailfish]][sailfish]         | [![][stars-sailfish]][repo-sailfish]         | [![][contr-sailfish]][repo-sailfish]         | [![][act-sailfish]][repo-sailfish]         |
| [sycamore]     | [![][docs-img-sycamore]][docs-sycamore]         | [![][img-repo]][repo-sycamore]     | ssr          | [![][dl-sycamore]][sycamore]         | [![][stars-sycamore]][repo-sycamore]         | [![][contr-sycamore]][repo-sycamore]         | [![][act-sycamore]][repo-sycamore]         |
| [tera]         | [![][docs-img-tera]][docs-tera]                 | [![][img-repo]][repo-tera]         | interpreted  | [![][dl-tera]][tera]                 | [![][stars-tera]][repo-tera]                 | [![][contr-tera]][repo-tera]                 | [![][act-tera]][repo-tera]                 |
| [tinytemplate] | [![][docs-img-tinytemplate]][docs-tinytemplate] | [![][img-repo]][repo-tinytemplate] | interpreted  | [![][dl-tinytemplate]][tinytemplate] | [![][stars-tinytemplate]][repo-tinytemplate] | [![][contr-tinytemplate]][repo-tinytemplate] | [![][act-tinytemplate]][repo-tinytemplate] |
| [vy]           | [![][docs-img-vy]][docs-vy]                     | [![][img-repo]][repo-vy]           | pre-compiled | [![][dl-vy]][vy]                     | [![][stars-vy]][repo-vy]                     | [![][contr-vy]][repo-vy]                     | [![][act-vy]][repo-vy]                     |
| [yew]          | [![][docs-img-yew]][docs-yew]                   | [![][img-repo]][repo-yew]          | ssr          | [![][dl-yew]][yew]                   | [![][stars-yew]][repo-yew]                   | [![][contr-yew]][repo-yew]                   | [![][act-yew]][repo-yew]                   |

Please see also [*Rust web framework comparison*].

## Benchmark Results

The benchmarking was done on Github's action runners, on an AMD-64 Linux system.
The absolute performance can vary, but the relative performance of crate *X* to *Y* should be stable.

[![](https://askama-rs.github.io/template-benchmark/results.svg)](https://askama-rs.github.io/template-benchmark/)

More information can be found in: <https://askama-rs.github.io/template-benchmark/>.

The "big table" benchmark generates a 100×100 cell HTML table.
The "teams" benchmark contains a list of four sport teams and their scores.
The former test contains only raw data, but a lot of it.
The latter test includes texts that (might) need escaping, and "if" conditions.

To run the benchmarks on your machine, clone the repo, and execute `cargo bench`.
The console will contain performance numbers,
and the file `target/criterion/report/index.html` will contain more information.
Run `./generate-table.py` to generate the table you see above.

[*Rust web framework comparison*]: <https://github.com/flosse/rust-web-framework-comparison>
[img-repo]: <https://img.shields.io/badge/-repo-f8f8f8?style=flat-square&logo=github&logoColor=black>

[askama]: <https://crates.io/crates/askama/>
[dioxus]: <https://crates.io/crates/dioxus/>
[handlebars]: <https://crates.io/crates/handlebars/>
[horrorshow]: <https://crates.io/crates/horrorshow/>
[hypertext]: <https://crates.io/crates/hypertext/>
[leptos]: <https://crates.io/crates/leptos/>
[liquid]: <https://crates.io/crates/liquid/>
[markup]: <https://crates.io/crates/markup/>
[maud]: <https://crates.io/crates/maud/>
[minijinja]: <https://crates.io/crates/minijinja/>
[ructe]: <https://crates.io/crates/ructe/>
[sailfish]: <https://crates.io/crates/sailfish/>
[sycamore]: <https://crates.io/crates/sycamore/>
[tera]: <https://crates.io/crates/tera/>
[tinytemplate]: <https://crates.io/crates/tinytemplate/>
[vy]: <https://crates.io/crates/vy/>
[yew]: <https://crates.io/crates/yew/>

[docs-askama]: <https://docs.rs/askama/latest/askama>
[docs-dioxus]: <https://docs.rs/dioxus/latest/dioxus>
[docs-handlebars]: <https://docs.rs/handlebars/latest/handlebars>
[docs-horrorshow]: <https://docs.rs/horrorshow/latest/horrorshow>
[docs-hypertext]: <https://docs.rs/hypertext/latest/hypertext>
[docs-leptos]: <https://docs.rs/leptos/latest/leptos>
[docs-liquid]: <https://docs.rs/liquid/latest/liquid>
[docs-markup]: <https://docs.rs/markup/latest/markup>
[docs-maud]: <https://docs.rs/maud/latest/maud>
[docs-minijinja]: <https://docs.rs/minijinja/latest/minijinja>
[docs-ructe]: <https://docs.rs/ructe/latest/ructe>
[docs-sailfish]: <https://docs.rs/sailfish/latest/sailfish>
[docs-sycamore]: <https://docs.rs/sycamore/latest/sycamore>
[docs-tera]: <https://docs.rs/tera/latest/tera>
[docs-tinytemplate]: <https://docs.rs/tinytemplate/latest/tinytemplate>
[docs-vy]: <https://docs.rs/vy/latest/vy>
[docs-yew]: <https://docs.rs/yew/latest/yew>

[docs-img-askama]: <https://img.shields.io/docsrs/askama?label=&style=flat-square>
[docs-img-dioxus]: <https://img.shields.io/docsrs/dioxus?label=&style=flat-square>
[docs-img-handlebars]: <https://img.shields.io/docsrs/handlebars?label=&style=flat-square>
[docs-img-horrorshow]: <https://img.shields.io/docsrs/horrorshow?label=&style=flat-square>
[docs-img-hypertext]: <https://img.shields.io/docsrs/hypertext?label=&style=flat-square>
[docs-img-leptos]: <https://img.shields.io/docsrs/leptos?label=&style=flat-square>
[docs-img-liquid]: <https://img.shields.io/docsrs/liquid?label=&style=flat-square>
[docs-img-markup]: <https://img.shields.io/docsrs/markup?label=&style=flat-square>
[docs-img-maud]: <https://img.shields.io/docsrs/maud?label=&style=flat-square>
[docs-img-minijinja]: <https://img.shields.io/docsrs/minijinja?label=&style=flat-square>
[docs-img-ructe]: <https://img.shields.io/docsrs/ructe?label=&style=flat-square>
[docs-img-sailfish]: <https://img.shields.io/docsrs/sailfish?label=&style=flat-square>
[docs-img-sycamore]: <https://img.shields.io/docsrs/sycamore?label=&style=flat-square>
[docs-img-tera]: <https://img.shields.io/docsrs/tera?label=&style=flat-square>
[docs-img-tinytemplate]: <https://img.shields.io/docsrs/tinytemplate?label=&style=flat-square>
[docs-img-vy]: <https://img.shields.io/docsrs/vy?label=&style=flat-square>
[docs-img-yew]: <https://img.shields.io/docsrs/yew?label=&style=flat-square>

[dl-askama]: <https://img.shields.io/crates/dr/askama?label=&color=f8f8f8&style=flat-square>
[dl-dioxus]: <https://img.shields.io/crates/dr/dioxus?label=&color=f8f8f8&style=flat-square>
[dl-handlebars]: <https://img.shields.io/crates/dr/handlebars?label=&color=f8f8f8&style=flat-square>
[dl-horrorshow]: <https://img.shields.io/crates/dr/horrorshow?label=&color=f8f8f8&style=flat-square>
[dl-hypertext]: <https://img.shields.io/crates/dr/hypertext?label=&color=f8f8f8&style=flat-square>
[dl-leptos]: <https://img.shields.io/crates/dr/leptos?label=&color=f8f8f8&style=flat-square>
[dl-liquid]: <https://img.shields.io/crates/dr/liquid?label=&color=f8f8f8&style=flat-square>
[dl-markup]: <https://img.shields.io/crates/dr/markup?label=&color=f8f8f8&style=flat-square>
[dl-maud]: <https://img.shields.io/crates/dr/maud?label=&color=f8f8f8&style=flat-square>
[dl-minijinja]: <https://img.shields.io/crates/dr/minijinja?label=&color=f8f8f8&style=flat-square>
[dl-ructe]: <https://img.shields.io/crates/dr/ructe?label=&color=f8f8f8&style=flat-square>
[dl-sailfish]: <https://img.shields.io/crates/dr/sailfish?label=&color=f8f8f8&style=flat-square>
[dl-sycamore]: <https://img.shields.io/crates/dr/sycamore?label=&color=f8f8f8&style=flat-square>
[dl-tera]: <https://img.shields.io/crates/dr/tera?label=&color=f8f8f8&style=flat-square>
[dl-tinytemplate]: <https://img.shields.io/crates/dr/tinytemplate?label=&color=f8f8f8&style=flat-square>
[dl-vy]: <https://img.shields.io/crates/dr/vy?label=&color=f8f8f8&style=flat-square>
[dl-yew]: <https://img.shields.io/crates/dr/yew?label=&color=f8f8f8&style=flat-square>

[stars-askama]: <https://img.shields.io/github/stars/askama-rs/askama?label=&color=f8f8f8&style=flat-square>
[stars-dioxus]: <https://img.shields.io/github/stars/DioxusLabs/dioxus?label=&color=f8f8f8&style=flat-square>
[stars-handlebars]: <https://img.shields.io/github/stars/sunng87/handlebars-rust?label=&color=f8f8f8&style=flat-square>
[stars-horrorshow]: <https://img.shields.io/github/stars/Stebalien/horrorshow-rs?label=&color=f8f8f8&style=flat-square>
[stars-leptos]: <https://img.shields.io/github/stars/leptos-rs/leptos?label=&color=f8f8f8&style=flat-square>
[stars-liquid]: <https://img.shields.io/github/stars/cobalt-org/liquid-rust?label=&color=f8f8f8&style=flat-square>
[stars-hypertext]: <https://img.shields.io/github/stars/vidhanio/hypertext?label=&color=f8f8f8&style=flat-square>
[stars-markup]: <https://img.shields.io/github/stars/utkarshkukreti/markup.rs?label=&color=f8f8f8&style=flat-square>
[stars-maud]: <https://img.shields.io/github/stars/lambda-fairy/maud?label=&color=f8f8f8&style=flat-square>
[stars-minijinja]: <https://img.shields.io/github/stars/mitsuhiko/minijinja?label=&color=f8f8f8&style=flat-square>
[stars-ructe]: <https://img.shields.io/github/stars/kaj/ructe?label=&color=f8f8f8&style=flat-square>
[stars-sailfish]: <https://img.shields.io/github/stars/rust-sailfish/sailfish?label=&color=f8f8f8&style=flat-square>
[stars-sycamore]: <https://img.shields.io/github/stars/sycamore-rs/sycamore?label=&color=f8f8f8&style=flat-square>
[stars-tera]: <https://img.shields.io/github/stars/Keats/tera?label=&color=f8f8f8&style=flat-square>
[stars-tinytemplate]: <https://img.shields.io/github/stars/bheisler/TinyTemplate?label=&color=f8f8f8&style=flat-square>
[stars-vy]: <https://img.shields.io/github/stars/JonahLund/vy?label=&color=f8f8f8&style=flat-square>
[stars-yew]: <https://img.shields.io/github/stars/yewstack/yew?label=&color=f8f8f8&style=flat-square>

[contr-askama]: <https://img.shields.io/github/contributors/askama-rs/askama?label=&color=f8f8f8&style=flat-square>
[contr-dioxus]: <https://img.shields.io/github/contributors/DioxusLabs/dioxus?label=&color=f8f8f8&style=flat-square>
[contr-handlebars]: <https://img.shields.io/github/contributors/sunng87/handlebars-rust?label=&color=f8f8f8&style=flat-square>
[contr-horrorshow]: <https://img.shields.io/github/contributors/Stebalien/horrorshow-rs?label=&color=f8f8f8&style=flat-square>
[contr-hypertext]: <https://img.shields.io/github/contributors/vidhanio/hypertext?label=&color=f8f8f8&style=flat-square>
[contr-leptos]: <https://img.shields.io/github/contributors/leptos-rs/leptos?label=&color=f8f8f8&style=flat-square>
[contr-liquid]: <https://img.shields.io/github/contributors/cobalt-org/liquid-rust?label=&color=f8f8f8&style=flat-square>
[contr-markup]: <https://img.shields.io/github/contributors/utkarshkukreti/markup.rs?label=&color=f8f8f8&style=flat-square>
[contr-maud]: <https://img.shields.io/github/contributors/lambda-fairy/maud?label=&color=f8f8f8&style=flat-square>
[contr-minijinja]: <https://img.shields.io/github/contributors/mitsuhiko/minijinja?label=&color=f8f8f8&style=flat-square>
[contr-ructe]: <https://img.shields.io/github/contributors/kaj/ructe?label=&color=f8f8f8&style=flat-square>
[contr-sailfish]: <https://img.shields.io/github/contributors/rust-sailfish/sailfish?label=&color=f8f8f8&style=flat-square>
[contr-sycamore]: <https://img.shields.io/github/contributors/sycamore-rs/sycamore?label=&color=f8f8f8&style=flat-square>
[contr-tera]: <https://img.shields.io/github/contributors/Keats/tera?label=&color=f8f8f8&style=flat-square>
[contr-tinytemplate]: <https://img.shields.io/github/contributors/bheisler/TinyTemplate?label=&color=f8f8f8&style=flat-square>
[contr-vy]: <https://img.shields.io/github/contributors/JonahLund/vy?label=&color=f8f8f8&style=flat-square>
[contr-yew]: <https://img.shields.io/github/contributors/yewstack/yew?label=&color=f8f8f8&style=flat-square>

[act-askama]: <https://img.shields.io/github/commit-activity/y/askama-rs/askama?label=&color=f8f8f8&style=flat-square>
[act-dioxus]: <https://img.shields.io/github/commit-activity/y/DioxusLabs/dioxus?label=&color=f8f8f8&style=flat-square>
[act-handlebars]: <https://img.shields.io/github/commit-activity/y/sunng87/handlebars-rust?label=&color=f8f8f8&style=flat-square>
[act-horrorshow]: <https://img.shields.io/github/commit-activity/y/Stebalien/horrorshow-rs?label=&color=f8f8f8&style=flat-square>
[act-hypertext]: <https://img.shields.io/github/commit-activity/y/vidhanio/hypertext?label=&color=f8f8f8&style=flat-square>
[act-leptos]: <https://img.shields.io/github/commit-activity/y/leptos-rs/leptos?label=&color=f8f8f8&style=flat-square>
[act-liquid]: <https://img.shields.io/github/commit-activity/y/cobalt-org/liquid-rust?label=&color=f8f8f8&style=flat-square>
[act-markup]: <https://img.shields.io/github/commit-activity/y/utkarshkukreti/markup.rs?label=&color=f8f8f8&style=flat-square>
[act-maud]: <https://img.shields.io/github/commit-activity/y/lambda-fairy/maud?label=&color=f8f8f8&style=flat-square>
[act-minijinja]: <https://img.shields.io/github/commit-activity/y/mitsuhiko/minijinja?label=&color=f8f8f8&style=flat-square>
[act-ructe]: <https://img.shields.io/github/commit-activity/y/kaj/ructe?label=&color=f8f8f8&style=flat-square>
[act-sailfish]: <https://img.shields.io/github/commit-activity/y/rust-sailfish/sailfish?label=&color=f8f8f8&style=flat-square>
[act-sycamore]: <https://img.shields.io/github/commit-activity/y/sycamore-rs/sycamore?label=&color=f8f8f8&style=flat-square>
[act-tera]: <https://img.shields.io/github/commit-activity/y/Keats/tera?label=&color=f8f8f8&style=flat-square>
[act-tinytemplate]: <https://img.shields.io/github/commit-activity/y/bheisler/TinyTemplate?label=&color=f8f8f8&style=flat-square>
[act-vy]: <https://img.shields.io/github/commit-activity/y/JonahLund/vy?label=&color=f8f8f8&style=flat-square>
[act-yew]: <https://img.shields.io/github/commit-activity/y/yewstack/yew?label=&color=f8f8f8&style=flat-square>

[repo-askama]: <https://github.com/askama-rs/askama>
[repo-dioxus]: <https://github.com/DioxusLabs/dioxus>
[repo-handlebars]: <https://github.com/sunng87/handlebars-rust>
[repo-horrorshow]: <https://github.com/Stebalien/horrorshow-rs>
[repo-hypertext]: <https://github.com/vidhanio/hypertext>
[repo-leptos]: <https://github.com/leptos-rs/leptos>
[repo-liquid]: <https://github.com/cobalt-org/liquid-rust>
[repo-markup]: <https://github.com/utkarshkukreti/markup.rs>
[repo-maud]: <https://github.com/lambda-fairy/maud>
[repo-minijinja]: <https://github.com/mitsuhiko/minijinja>
[repo-ructe]: <https://github.com/kaj/ructe>
[repo-sailfish]: <https://github.com/rust-sailfish/sailfish>
[repo-sycamore]: <https://github.com/sycamore-rs/sycamore>
[repo-tera]: <https://github.com/Keats/tera>
[repo-tinytemplate]: <https://github.com/bheisler/TinyTemplate>
[repo-vy]: <https://github.com/JonahLund/vy>
[repo-yew]: <https://github.com/yewstack/yew>
