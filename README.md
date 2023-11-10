```
          $$\ $$\          $$\                     $$\           
          $$ |\__|         $$ |                    $$ |          
 $$$$$$$\ $$ |$$\        $$$$$$\    $$$$$$\   $$$$$$$ | $$$$$$\  
$$  _____|$$ |$$ |$$$$$$\\_$$  _|  $$  __$$\ $$  __$$ |$$  __$$\ 
$$ /      $$ |$$ |\______| $$ |    $$ /  $$ |$$ /  $$ |$$ /  $$ |
$$ |      $$ |$$ |         $$ |$$\ $$ |  $$ |$$ |  $$ |$$ |  $$ |
\$$$$$$$\ $$ |$$ |         \$$$$  |\$$$$$$  |\$$$$$$$ |\$$$$$$  |
 \_______|\__|\__|          \____/  \______/  \_______| \______/ 
``` 
<div align="center">

[![Rust](https://github.com/mackeper/cli-todo/actions/workflows/rust.yml/badge.svg)](https://github.com/mackeper/cli-todo/actions/workflows/rust.yml)
![GitHub all releases](https://img.shields.io/github/downloads/mackeper/cli-todo/total)
![GitHub](https://img.shields.io/github/license/mackeper/cli-todo)

Simple todo list in your command-line :white_check_mark:

[Usage](#usage-heart_eyes) •
[Installation](#installation-coffee) •
[Development](#development-sparkles) •
[Roadmap](#roadmap-world_map) •
[FAQ](#faq-question) •
[Support](#support-love_letter)  



</div>

## Usage :heart_eyes:
```
Usage: todo.exe [operation] [arguments]
Operations:
    list (l)             : Display all current items.
    add (a) [item]       : Add a new item.
    remove (r) [id]      : Remove an item by its ID.
    edit (e) [id] [item] : Edit an item by its ID.
    done (d) [id]        : Toggle its done state by ID.
    clear                : Remove all items.
```

## Installation :coffee:
### Windows :computer:
Download the release

### Linux :penguin:
Coming soon :hourglass_flowing_sand:

## Development :sparkles:
### Build :hammer:
```
cargo build --release
```
    
## Roadmap :world_map:
- Maybe support multiple lists :floppy_disk:

## FAQ :question:

#### Where are the todo lists stored?
- On Windows: `%localappdata%\cli-todo`
- On Linux: Coming soon

#### Question 2

Answer 2

## Support :love_letter:

Submit an [issue!](https://github.com/mackeper/cli-todo/issues/new?assignees=&labels=question&projects=&template=question.yaml&title=%5BQUESTION%5D+%3Ctitle%3E)
