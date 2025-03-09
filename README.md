## Building bootloader, kernel, and ramfs image
```
wget http://buildroot.org/downloads/buildroot-2024.02.11.tar.gz
tar xf buildroot-2024.02.11.tar.gz 
cd buildroot-2024.02.11
make BR2_EXTERNAL=$PWD/../bsp nybble_defconfig
make sdk
make
```