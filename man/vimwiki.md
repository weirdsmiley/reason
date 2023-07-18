Usage:
1) alone: vimwiki [filter]
2) pipe:  [paper list] | vimwiki

Open paper notes as wiki in Vim. This relies on VimWiki
plugin. In your vimrc you need to allow
`g:vimwiki_global_ext` by setting it to 1 as shown below
`let g:vimwiki_global_ext = 1`

You also need to set `config.storage.wiki_dir` to tell
VimWiki where to store wikis.

The functionality remains same as of `ed` command.

The following might come in handy:
```
ls as Reason | open | vimwiki
```

