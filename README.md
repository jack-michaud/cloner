# Cloner
`git clone` a repo into `~/Code/<origin>/<owner>/<repo>`.

For example,
```
cloner git@github.com:jack-michaud/cloner
```

clones into 
```
Code/github.com/jack-michaud/cloner/
```

# Installation

Check in the [Releases tab for the latest binary download](https://github.com/jack-michaud/cloner/releases).

If you need a different version for your machine, [make an issue](https://github.com/jack-michaud/cloner/issues/new).


# Practical Use

```
# Script to cd into a Code directory
alias coad='cd $(find ~/Code -maxdepth 3 | fzf)'
```
