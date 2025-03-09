#!/bin/bash
root=$(dirname "$0")

if [ -z "$1" ]
  then
    echo "No argument supplied"
    exit 1
fi
SD=/dev/$1

if [ ! -e ${SD}1 ] || [ ! -e ${SD}2 ] 
  then
    echo "${SD} not found!"
    exit 1
fi

B=$root/buildroot-2024.02.11/output/images
if [ ! -e ${B} ] 
  then
    echo "Cant find $B!"
    exit 1
fi

mkdir -pv $root/mnt/boot $root/mnt/rootfs
mount /dev/$11 $root/mnt/boot
mount /dev/$12 $root/mnt/rootfs

cp $B/boot.scr $B/linux-nybble.dtb $B/zImage $B/rootfs.cpio.uboot $root/mnt/boot
cp -ar $root/rootfs/* $root/mnt/rootfs
chown root:root $root/mnt/rootfs/*

umount /dev/$11
umount /dev/$12
rm -rf $root/mnt/boot $root/mnt/rootfs

dd if=$B/u-boot-sunxi-with-spl.bin of=/dev/$1 bs=1024 seek=8 conv=notrunc
