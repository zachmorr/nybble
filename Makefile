include gmsl


#########################################################################
#																		#
#							GLOBAL VARS									#
#																		#
#########################################################################
.DEFAULT_GOAL:=all
SHELL=/bin/bash
PROJECT=$(shell realpath ./)
CONFIG_DIR=$(PROJECT)/config
BUILD_DIR=$(PROJECT)/build
IMAGE_DIR=$(BUILD_DIR)/images
SYSROOT_DIR=$(BUILD_DIR)/sysroot
TOOLCHAIN_DIR=$(BUILD_DIR)/toolchain
ROOTFS_DIR=$(BUILD_DIR)/rootfs

$(IMAGE_DIR):
	mkdir -pv $(IMAGE_DIR)

$(SYSROOT_DIR):
	mkdir -pv $(SYSROOT_DIR)

$(TOOLCHAIN_DIR):
	mkdir -pv $(TOOLCHAIN_DIR)

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

.PHONY: bsp-clean
bsp-clean:
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

.PHONY: all
all:: linux

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

.PHONY: all
all:: uboot

############################# Build Busybox ##############################
# .PHONY: busybox
# busybox:
# 	make -C $(BUILDROOT_SRC) busybox-rebuild 

# .PHONY: busybox-clean
# busybox-clean:
# 	make -C $(BUILDROOT_SRC) busybox-dirclean

# .PHONY: busybox-menuconfig
# busybox-menuconfig:
# 	make -C $(BUILDROOT_SRC) busybox-menuconfig


############################# Build Image ##################################
CPIO=$(IMAGE_DIR)/rootfs.cpio.uboot
SDK=$(TOOLCHAIN_DIR)/arm-buildroot-linux-musleabi_sdk-buildroot

$(CPIO) $(SDK): $(BUILDROOT_CONFIG) $(BSP)/nybble/overlay/init | $(IMAGE_DIR) $(TOOLCHAIN_DIR)
	make -C $(BUILDROOT_SRC) sdk
	cp $(BUILDROOT_IMAGES)/rootfs.cpio.uboot $(IMAGE_DIR)
	tar -xf $(BUILDROOT_IMAGES)/arm-buildroot-linux-musleabi_sdk-buildroot.tar.gz -C $(TOOLCHAIN_DIR)

.PHONY: sdk
sdk: $(SDK)

.PHONY: all
all:: sdk


#########################################################################
#																		#
#								RootFS									#
#																		#
#########################################################################
SKELETON_DIR= $(CONFIG_DIR)/target/skeleton/

$(ROOTFS_DIR):
	mkdir -pv $(ROOTFS_DIR)
	cp -r --preserve=mode,ownership,timestamps $(CONFIG_DIR)/target/skeleton/* $(ROOTFS_DIR)


#########################################################################
#																		#
#								Packages								#
#																		#
#########################################################################
ENV_SETUP=. $(SDK)/environment-setup
SDK_MAKE=$(ENV_SETUP) && $(MAKE)

include $(wildcard $(CONFIG_DIR)/target/**/*.mk)




#########################################################################
#																		#
#								Debugging								#
#																		#
#########################################################################
.PHONY: printvars
printvars:
ifndef VARS
	$(error Please pass a non-empty VARS to 'make printvars')
endif
	@:
	$(foreach V, \
		$(sort $(foreach X, $(.VARIABLES), $(filter $(VARS),$(X)))), \
		$(if $(filter-out environment% default automatic, \
				$(origin $V)), \
		$(if $(QUOTED_VARS),\
			$(info $V='$(subst ','\'',$(if $(RAW_VARS),$(value $V),$($V)))'), \
			$(info $V=$(if $(RAW_VARS),$(value $V),$($V))))))
