setenv bootargs console=ttyS0,115200 earlyprintk debug
fatload mmc 0 ${kernel_addr_r} zImage
fatload mmc 0 ${fdt_addr_r} linux-nybble.dtb
fatload mmc 0 ${ramdisk_addr_r} rootfs.cpio.uboot
bootz ${kernel_addr_r} ${ramdisk_addr_r} ${fdt_addr_r}
