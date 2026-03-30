#!/bin/sh

gadget=g1
configfs=/sys/kernel/config
mkdir -v $configfs/usb_gadget/$gadget
cd $configfs/usb_gadget/$gadget

echo 0xABCD > idVendor
echo 0x1234 > idProduct

str=strings/0x409
mkdir $str
echo serial > $str/serialnumber
echo zach > $str/manufacturer
echo nybble > $str/product

conf=configs/usb.1
mkdir $conf
mkdir $conf/$str
echo conf > $conf/$str/configuration
echo 120 > $conf/MaxPower

mkdir functions/acm.GS0
ln -s functions/acm.GS0 $conf
echo "musb-hdrc.1.auto" > $configfs/usb_gadget/$gadget/UDC

RUST_LOG=trace RUST_BACKTRACE=1 nybd /dev/ttyGS0 &
cd /
