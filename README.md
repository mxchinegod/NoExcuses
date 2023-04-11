<h1 align="center">

🙅 NoExcuses!

</h1>
<p align="center">
A super-fast way to delete unwanted tokens in text training data.

</p>

<center>

![build](https://github.com/DylanAlloy/NoExcuses/actions/workflows/build.yml/badge.svg)

</center>

This is a Rust program that removes phrases from all JSON files in a given directory. NoExcuses reads a list of phrases from a file, then iterates through each JSON file in a given directory and removes any occurrence of those phrases. NoExcuses writes back the modified JSON files to their original location.

## 🚀 Getting Started

To get started, clone the repository and run cargo build to compile the project. Then, you can run NoExcuses with the following command:

``` shell 
./target/debug/no_excuses [DIRECTORY_PATH] [PHRASES_FILE_PATH]
```

<sub>Replace `[DIRECTORY_PATH]` with the path to the directory that contains the JSON files you want to modify, and `[PHRASES_FILE_PATH]` with the path to the file containing the list of phrases you want to remove. If `[PHRASES_FILE_PATH]` is not provided, NoExcuses will use the default path "./phrases.txt"</sub>

## 👌🏼 Usage

NoExcuses reads each JSON file in the specified directory and removes any occurrence of the phrases provided in the phrases file. The modified JSON files are then written back to their original location.

## 🔮 Philosophy

NoExcuses is designed to be used in early stages of token data processing. When considering if I should try doing similar work with vector representations, it doesn't make as much sense. Maybe I will change my mind about this in the future but discriminate token processing like will probably happen prior to storing language vectors in your workflow 😌

## 🔌 Implementation Details

NoExcuses uses the `serde_json` library to parse and modify the JSON files.

The `remove_phrases_from_json` function recursively traverses the JSON data structure and removes any occurrence of the phrases provided in the phrases file.

## 🙋 Contributing

##### Proper commit message format is required for automated changelog generation. Examples:

    [<emoji>] [revert: ?]<type>[(scope)?]: <message>

    💥 feat(compiler): add 'comments' option
    🐛 fix(compiler): fix some bug
    📝 docs(compiler): add some docs
    🌷 UI(compiler): better styles
    🏰 chore(compiler): Made some changes to the scaffolding
    🌐 locale(compiler): Made a small contribution to internationalization

    Other commit types: refactor, perf, workflow, build, CI, typos, tests, types, wip, release, dep

### ✅ To-Do

- [x] this README
- [ ] add CSV logic

### 📑 License

This program is licensed under the MIT License. See the LICENSE file for more information.

