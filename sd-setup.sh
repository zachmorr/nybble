#!/bin/bash

if [ -z "$1" ]
  then
    echo "No argument supplied"
    exit 1
fi
SD=/dev/$1; shift

if [ ! -e ${SD} ]
  then
    echo "${SD} not found!"
    exit 1
fi

erase=''
while getopts 'e' flag; do
  case "${flag}" in
    e) erase="true" ;;
  esac
done

umount ${SD}*

if [ -n "$erase" ]
  then
    echo "Erasing ${SD}"
    dd if=/dev/zero of=${SD} bs=8192 status=progress
fi

parted --script ${SD} mklabel msdos
parted --script ${SD} mkpart primary fat32 4M 512M
parted --script ${SD} mkpart primary ext4 512M 32G

mkfs.vfat -I ${SD}1
mkfs.ext4 -F ${SD}2

# blockdev --rereadpt ${SD}