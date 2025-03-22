## Building bootloader, kernel, and ramfs image

```
wget http://buildroot.org/downloads/buildroot-2024.02.11.tar.gz
tar xf buildroot-2024.02.11.tar.gz 
cd buildroot-2024.02.11
make BR2_EXTERNAL=$PWD/../bsp nybble_defconfig
make
```

## Informative Links

Explainging Linux TTY Stack

https://www.linusakesson.net/programming/tty/

Blog Posts Covering Limitations of Terminal Emulators

https://arcan-fe.com/2022/04/02/the-day-of-a-new-command-line-interface-shell/ (best one)

https://arcan-fe.com/2022/10/15/whipping-up-a-new-shell-lashcat9/ 

https://arcan-fe.com/2016/12/29/chasing-the-dream-of-a-terminal-free-cli/

https://arcan-fe.com/2017/07/12/the-dawn-of-a-new-command-line-interface/

https://arcan-fe.com/2018/10/31/walkthrough-writing-a-kmscon-console-like-window-manager-using-arcan/

Reddit Post With Good Info

https://www.reddit.com/r/linux/comments/5k17em/terminal_forever_3_comicstrip/?sort=top

POSIX Shell Spec

https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html