#!/bin/sh

mount -t proc proc /nybble/processes
mount -t devtmpfs devtmpfs /nybble/devices
mount -t sysfs sysfs /nybble/system
mount -t debugfs debugfs /nybble/debug
mount -t configfs configfs /nybble/config

insmod /lib/modules/6.6.78/kernel/drivers/usb/gadget/libcomposite.ko
insmod /lib/modules/6.6.78/kernel/drivers/usb/gadget/function/u_serial.ko 
insmod /lib/modules/6.6.78/kernel/drivers/usb/gadget/function/usb_f_acm.ko 
insmod /lib/modules/6.6.78/kernel/drivers/usb/gadget/function/usb_f_fs.ko 

cd /
exec sh
