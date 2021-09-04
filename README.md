# Instructions to run

## Clone htmldsl

```
cd ~/storage/projects/
git clone git@github.com:quintenpalmer/htmldsl
```

## Clone target quintenpalmer.github.io

```
cd ~/storage/projects/
git clone git@github.com:quintenpalmer/quintenpalmer.github.io
```

## Run script

```
cd ~/storage/projects/personal_site_gen/
cargo run > ~/storage/projects/quintenpalmer.github.io/index.html
```

## Commit changes to github.io project

```
cd ~/storage/projects/quintenpalmer.github.io
git add -p
git commit
git push
```
