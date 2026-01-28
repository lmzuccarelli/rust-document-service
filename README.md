# Overview

A simple rust based markdown server


# Usage

clone the repo

```
cd rust-markdown-server

# compile

make build
```

execute 

```
./target/release/rust-markdown-server
```

## Storage notation

The document being stored muts have a notation as follows

```
<unique-name>-subject.<file-ext>  

example:

grok-results.md

the db_path will the be

db_path/unique-name/document.kv
```

## Markdown

Use a pluign i.e MarkDown Viewer for FireFox (install viewer apprropriate for your particular browser) 
