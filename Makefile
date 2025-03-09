#########################################################################
#																		#
#							GLOBAL VARS									#
#																		#
#########################################################################
.DEFAULT_GOAL:=all
PROJECT=$(shell realpath ./)
CONFIG_DIR=$(PROJECT)/config
BUILD_DIR=$(PROJECT)/build
IMAGE_DIR=$(BUILD_DIR)/images

$(IMAGE_DIR):
	mkdir -pv $(IMAGE_DIR)


#########################################################################
#																		#
#								BSP										#
#																		#
#########################################################################
########################## Create Work Dir ##############################
BUILDROOT_VERSION=2024.02.11
BUILDROOT_WORK_DIR=$(BUILD_DIR)/buildroot/$(BUILDROOT_VERSION)
BUILDROOT_SRC=$(BUILDROOT_WORK_DIR)/buildroot-$(BUILDROOT_VERSION)

$(BUILDROOT_SRC): | $(BUILDROOT_WORK_DIR)
	wget http://buildroot.org/downloads/buildroot-$(BUILDROOT_VERSION).tar.gz -O $(BUILDROOT_SRC).tar.gz
	tar xf $(BUILDROOT_SRC).tar.gz -C $(BUILDROOT_WORK_DIR)

$(BUILDROOT_WORK_DIR):
	mkdir -pv $(BUILDROOT_WORK_DIR)

.PHONY: buildroot-clean
buildroot-clean:
	rm -rf $(BUILDROOT_WORK_DIR)


########################### Configure BSP ###############################
BSP=$(CONFIG_DIR)/bsp
BUILDROOT_DEFCONFIG=$(BSP)/configs/nybble_defconfig
BUILDROOT_CONFIG=$(BUILDROOT_SRC)/.config

$(BUILDROOT_CONFIG): $(BUILDROOT_DEFCONFIG) | $(BUILDROOT_SRC)
	make -C $(BUILDROOT_SRC) BR2_EXTERNAL=$(BSP) nybble_defconfig

.PHONY: buildroot-menuconfig
buildroot-menuconfig: $(BUILDROOT_CONFIG)
	make -C $(BUILDROOT_SRC) menuconfig

.PHONY: buildroot-savedefconfig
buildroot-savedefconfig: $(BUILDROOT_CONFIG)
	make -C $(BUILDROOT_SRC) savedefconfig


############################# Build Linux ################################
BUILDROOT_BUILD=$(BUILDROOT_SRC)/output
BUILDROOT_IMAGES=$(BUILDROOT_BUILD)/images
ZIMAGE=$(IMAGE_DIR)/zImage
LINUX_DTB=$(IMAGE_DIR)/linux-nybble.dtb
 
$(ZIMAGE) $(LINUX_DTB): $(BUILDROOT_CONFIG) $(BSP)/nybble/linux_defconfig $(BSP)/nybble/linux-nybble.dts | $(IMAGE_DIR)
	make -C $(BUILDROOT_SRC) linux-rebuild 
	cp $(BUILDROOT_IMAGES)/zImage $(BUILDROOT_IMAGES)/linux-nybble.dtb $(IMAGE_DIR)

.PHONY: linux
linux: $(ZIMAGE) $(LINUX_DTB)

.PHONY: linux-clean
linux-clean:
	make -C $(BUILDROOT_SRC) linux-dirclean

.PHONY: linux-menuconfig
linux-menuconfig:
	make -C $(BUILDROOT_SRC) linux-menuconfig


############################# Build U-Boot ###############################
SPL=$(IMAGE_DIR)/u-boot-sunxi-with-spl.bin
SCR=$(IMAGE_DIR)/boot.scr

$(SPL) $(SCR): $(BUILDROOT_CONFIG) $(BSP)/nybble/uboot_defconfig $(BSP)/nybble/uboot-nybble.dts $(BSP)/nybble/boot.cmd | $(IMAGE_DIR)
	make -C $(BUILDROOT_SRC) uboot-rebuild 
	make -C $(BUILDROOT_SRC) host-uboot-tools-rebuild
	cp $(BUILDROOT_IMAGES)/boot.scr $(BUILDROOT_IMAGES)/u-boot-sunxi-with-spl.bin $(IMAGE_DIR)

.PHONY: uboot
uboot: $(SPL) $(SCR)

.PHONY: uboot-clean
uboot-clean:
	make -C $(BUILDROOT_SRC) uboot-dirclean

.PHONY: uboot-menuconfig
uboot-menuconfig:
	make -C $(BUILDROOT_SRC) uboot-menuconfig


############################# Build Busybox ##############################
.PHONY: busybox
busybox:
	make -C $(BUILDROOT_SRC) busybox-rebuild 

.PHONY: busybox-clean
busybox-clean:
	make -C $(BUILDROOT_SRC) busybox-dirclean

.PHONY: busybox-menuconfig
busybox-menuconfig:
	make -C $(BUILDROOT_SRC) busybox-menuconfig


############################# Build Image ##################################
CPIO=$(IMAGE_DIR)/rootfs.cpio.uboot
SDK=$(IMAGE_DIR)/arm-buildroot-linux-musleabi_sdk-buildroot.tar.gz

$(CPIO) $(SDK): $(BUILDROOT_CONFIG) $(BSP)/nybble/overlay/init | $(IMAGE_DIR)
	make -C $(BUILDROOT_SRC) sdk
	cp $(BUILDROOT_IMAGES)/arm-buildroot-linux-musleabi_sdk-buildroot.tar.gz $(BUILDROOT_IMAGES)/rootfs.cpio.uboot $(IMAGE_DIR)



#########################################################################
#																		#
#								Packages								#
#																		#
#########################################################################



.PHONY: all
all: $(ZIMAGE) $(SPL) $(SDK)

