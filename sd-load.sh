#!/bin/bash
set -x
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

B=$root/build/images
if [ ! -e ${B} ] 
  then
    echo "Cant find $B!"
    exit 1
fi

boot=$root/build/mnt/boot
rootfs=$root/build/mnt/rootfs

mkdir -pv $boot $rootfs
mount /dev/$11 $boot && rm -rf $boot/*
mount /dev/$12 $rootfs && rm -rf $rootfs/*

cp $B/boot.scr $B/linux-nybble.dtb $B/zImage $B/rootfs.cpio.uboot $boot
cp -r $root/build/rootfs/* $rootfs
chown -R root:root $rootfs/*
sync

umount /dev/$11
umount /dev/$12
rm -rf $root/mnt

dd if=$B/u-boot-sunxi-with-spl.bin of=/dev/$1 bs=1024 seek=8 conv=notrunc
