## Building bootloader, kernel, and ramfs image
```
wget http://buildroot.org/downloads/buildroot-2024.02.11.tar.gz
tar xf buildroot-2024.02.11.tar.gz 
cd buildroot-2024.02.11
make BR2_EXTERNAL=$PWD/../bsp nybble_defconfig
make sdk
make
```

## Informative Links
Blog: https://www.linusakesson.net/programming/tty/
Blog: https://arcan-fe.com/2016/12/29/chasing-the-dream-of-a-terminal-free-cli/
Blog: https://arcan-fe.com/2017/07/12/the-dawn-of-a-new-command-line-interface/
Blog: https://arcan-fe.com/2022/10/15/whipping-up-a-new-shell-lashcat9/
Examples: https://www.reddit.com/r/linux/comments/5k17em/terminal_forever_3_comicstrip/?sort=top