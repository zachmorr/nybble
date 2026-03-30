#!/bin/sh

mount -t proc proc /proc
mount -t devtmpfs devtmpfs /dev
mount -t sysfs sysfs /sys
mount -t debugfs debugfs /sys/kernel/debug
mount -t configfs configfs /sys/kernel/config

insmod /lib/modules/6.6.78/kernel/drivers/usb/gadget/libcomposite.ko
insmod /lib/modules/6.6.78/kernel/drivers/usb/gadget/function/u_serial.ko 
insmod /lib/modules/6.6.78/kernel/drivers/usb/gadget/function/usb_f_acm.ko 
insmod /lib/modules/6.6.78/kernel/drivers/usb/gadget/function/usb_f_fs.ko 

cd /
exec sh
