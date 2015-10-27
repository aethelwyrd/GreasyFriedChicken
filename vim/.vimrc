"vimrc
"Copyright 2015 David Huddle
"
:set nocompatible
:set cursorline
:set colorcolumn=80,120
:set spell
:set spellsuggest=best,10
:set hidden

"setup helpful navigation
:set relativenumber
:set ruler

":colorscheme desert
:syntax on
:set hlsearch
:set vb
:set noerrorbells
:set showcmd
:set laststatus=2
:set statusline=%F%m%r%h%w\ [FORMAT=%{&ff}]\ [TYPE=%Y]\[POS=%l,%v][%p%%]\ %{strftime(\"%d/%m/%y\ -\ %H:%M\")}
:set scrolloff=5
"set tags
:set tags=./tags;
:set autoread
:set showmatch
" Configure backspace so it acts as it should act
:set backspace=eol,start,indent
:set whichwrap+=<,>,h,l

"setup case sensitivity
:set ignorecase
:set smartcase

"use alt + jkhl for window navigation
:tnoremap <A-h> <C-\><C-n><C-w>h
:tnoremap <A-j> <C-\><C-n><C-w>j
:tnoremap <A-k> <C-\><C-n><C-w>k
:tnoremap <A-l> <C-\><C-n><C-w>l
:nnoremap <A-h> <C-w>h
:nnoremap <A-j> <C-w>j
:nnoremap <A-k> <C-w>k
:nnoremap <A-l> <C-w>l
:tnoremap <Leader>e <C-\><C-n>

"* and # are case sensitive
:nnoremap * /\<<C-R>=expand('<cword>')<CR>\><CR>
:nnoremap # ?\<<C-R>=expand('<cword>')<CR>\><CR>

"setup tabs
:set listchars=tab:>-,trail:.,eol:$
:set tabstop=4
:set sw=4
:set expandtab
:set cindent

"font
"set guifont=Courier_New:h8:b:cANSI
set guifont=Hack

"ignore files
set wildignore+=*\\tmp\\*,*\\Debug\\*,*\\Release\\*,*\\_*\\*,*\\PrebuiltUnits\*,*.swp,*.zip,*.exe  " Windows
"set wildignore+=*/tmp/*,*.so,*.swp,*.zip     " MacOSX/Linux

"key mappings
nmap <leader>cd :lcd %:p:h<CR>:pwd<CR>
nmap <leader>pc :!p4 edit %<CR>
nmap <leader>pr :!p4 revert %<CR>
nmap <leader>ps :!p4 set P4CLIENT=dhuddle_Scaleform_Upgrade_Dev<CR>
nmap <leader>tt :TagbarToggle <CR>
nmap <leader>ft :ts <c-r><c-w><CR>
nmap <leader>bs :ls<CR>:e #
nmap <leader>, :vertical resize -5<CR>
nmap <leader>. :vertical resize +5<CR> 
imap :imap <TAB> <C-N> 
"nmap <F7> :%g/^\s*$/d <CR>

"Find a buffer function
function! BufSel(pattern)
  let bufcount = bufnr("$")
  let currbufnr = 1
  let nummatches = 0
  let firstmatchingbufnr = 0
  while currbufnr <= bufcount
    if(bufexists(currbufnr))
      let currbufname = bufname(currbufnr)
      if(match(currbufname, a:pattern) > -1)
        echo currbufnr . ": ". bufname(currbufnr)
        let nummatches += 1
        let firstmatchingbufnr = currbufnr
      endif
    endif
    let currbufnr = currbufnr + 1
  endwhile
  if(nummatches == 1)
    execute ":buffer ". firstmatchingbufnr
  elseif(nummatches > 1)
    let desiredbufnr = input("Enter buffer number: ")
    if(strlen(desiredbufnr) != 0)
      execute ":buffer ". desiredbufnr
    endif
  else
    echo "No matching buffers"
  endif
endfunction

"Find macro
command! -nargs=1 Findall :vimgrep/<args>/gj **/*.*
command! -nargs=1 Findmost :vimgrep/<args>/gj **/*.{cpp,h,hpp,cs,c,as,sln,props,proj,xml} | copen
command! -nargs=1 Findcpp :vimgrep/<args>/gj **/*.{cpp,h,hpp} | copen
command! -nargs=1 Buf :call BufSel("<args>")

if has("gui_running")
  " GUI is running or is about to start.
  " Maximize gvim window.
  set lines=999 columns=99
else
  " This is console Vim.
  if exists("+lines")
    set lines=70
  endif
  if exists("+columns")
    set columns=200
  endif
endif

"vim-plug
if has("unix") 
    if !filereadable(expand("~/.vim/autoload/plug.vim"))
        silent !mkdir -p ~/.vim/{autoload,undo,backups}
        silent !curl -fLo ~/.vim/autoload/plug.vim https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim
    endif
else
    "TODO fix for winders
endif
call plug#begin('~/.vim/plugged')
" Make sure you use single quotes
Plug 'junegunn/seoul256.vim'
Plug 'kien/ctrlp.vim'
Plug 'rking/ag.vim'
Plug 'majutsushi/tagbar'
Plug 'vim-scripts/SearchComplete'
Plug 'vim-scripts/ShowMarks'
Plug 'vim-scripts/vimwiki'
"Plug 'scrooloose/nerdtree', { on : 'NERDTreeToggle' }
"Plug 'Valloric/YouCompleteMe', { 'for': 'cpp' } "mabee someday
"autocmd! User YouCompleteMe call youcompleteme#Enable()
"on pause
"Plug 'scrooloose/syntastic'
"Plug 'vim-scripts/taglist.vim' "trying alt
call plug#end()

"unlimited undo
set undodir=$HOME/.vim/undo
set backupdir=$HOME/.vim/backups
set undofile
set backup
set writebackup

"showmarks config
let g:showmarks_hlline_lower = 1
let g:showmarks_include = "abcdefghijklmnopqrstuvwxyz"

"custom colors from seoul
:let g:seoul256_background = 233
:colo seoul256

"setup for ctrlp
let g:ctrlp_map = '<c-p>'
let g:ctrlp_cmd = 'CtrlP'
let g:ctrlp_working_path_mode = 'ra'
let g:ctrlp_clear_cache_on_exit = 0

"setup for NerdTree
"map <C-n> :NERDTreeToggle<CR>

" The Silver Searcher
if executable('ag')
  " Use ag over grep
  set grepprg=ag\ --nogroup\ --nocolor
endif
" bind K to grep word under cursor
nmap <C-k> :grep! "\b<C-R><C-W>\b"<CR>:cw<CR>

