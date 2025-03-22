#!/bin/sh

exec setsid sh
exec </nybble/devices/ttyS0 >/nybble/devices/ttyS0 2>&1
exec sh

gadget=g1
configfs=/nybble/config
mkdir -v $configfs/usb_gadget/$gadget
cd $configfs/usb_gadget/$gadget

echo 0xABCD > idVendor
echo 0x1234 > idProduct

str=strings/0x409
mkdir $str
echo "serial" > $str/serialnumber
echo "zach" > $str/manufacturer
echo "nybble" > $str/product

conf=configs/usb.1
mkdir $conf
mkdir $conf/$str
echo "usb conf" > $conf/$str/configuration
echo 120 > $conf/MaxPower

# mkdir functions/acm.GS0
# ln -s functions/acm.GS0 $conf

mkdir functions/ffs.usb
ln -s functions/ffs.usb $conf

cd /nybble/devices 
mkdir ffs-usb
mount -t functionfs usb /nybble/devices/ffs-usb
cd ffs-usb

# echo "musb-hdrc.1.auto" > $configfs/usb_gadget/$gadget/UDC