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


# Practical Use

```
# Script to cd into a Code directory
alias coad="cd $(find ~/Code -maxdepth 3 | fzf)"
```
