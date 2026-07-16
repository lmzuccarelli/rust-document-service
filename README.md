# Overview

A simple rust based markdown server


# Usage

clone the repo

```
cd rust-document-service

# compile

make build-nonstatic
```

execute 

```
./target/release/rust-document-service
```

To compile using static linked libraires

```bash
make build

# if this fails try installing glibc-static
# i.e for fedoar execute the following (other distros will differ)
sudo dnf install glibc-static
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
