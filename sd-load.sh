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

boot=/mnt/boot
rootfs=/mnt/rootfs
image=/mnt/image

mkdir -pv $boot $rootfs $image
mount /dev/$11 $boot && rm -rf $boot/*
mount /dev/$12 $rootfs && rm -rf $rootfs/*
mount -o loop $B/rootfs.ext4 $image 

cp $B/boot.scr $B/linux-nybble.dtb $B/zImage $boot
cp -r $image/* $rootfs
rm -r $rootfs/lost+found
chown -R root:root $rootfs/*
sync

umount /dev/$11
umount /dev/$12
umount $B/rootfs.ext4


dd if=$B/u-boot-sunxi-with-spl.bin  of=/dev/$1 bs=1024 seek=8 conv=notrunc
