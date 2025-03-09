define init_package
name=$(call uc,$(1))
$$(name)_NAME?=$(1)
$$(name)_VERSION?=$(2)
$$(name)_FULLNAME?=$$($$(name)_NAME)-$$($$(name)_VERSION)
$$(name)_WORK_DIR?=$(BUILD_DIR)/target/$$($$(name)_NAME)/$$($$(name)_VERSION)
$$(name)_SRC_DIR?=$$($$(name)_WORK_DIR)/src
$$(name)_BUILD_DIR?=$$($$(name)_WORK_DIR)/build
$$(name)_INSTALL_DIR?=$$($$(name)_WORK_DIR)/install

$$(name)_FETCH_STAMP=$$($$(name)_WORK_DIR)/.stamp_fetch
$$(name)_CONFIGURE_STAMP=$$($$(name)_WORK_DIR)/.stamp_configure
$$(name)_BUILD_STAMP=$$($$(name)_WORK_DIR)/.stamp_build
$$(name)_INSTALL_STAMP=$$($$(name)_WORK_DIR)/.stamp_install
$$(name)_STAGE_STAMP=$$($$(name)_WORK_DIR)/.stamp_stage


$$($$(name)_WORK_DIR):
	mkdir -pv $$($$(name)_WORK_DIR)

$$($$(name)_SRC_DIR): | $$($$(name)_WORK_DIR)
	mkdir -pv $$($$(name)_SRC_DIR)

$$($$(name)_BUILD_DIR): | $$($$(name)_WORK_DIR)
	mkdir -pv $$($$(name)_BUILD_DIR)

$$($$(name)_INSTALL_DIR): | $$($$(name)_WORK_DIR)
	mkdir -pv $$($$(name)_INSTALL_DIR)

.PHONY: $$($$(name)_NAME)-init
$$($$(name)_NAME)-init: | $$($$(name)_WORK_DIR) $$($$(name)_SRC_DIR) $$($$(name)_BUILD_DIR) $$($$(name)_INSTALL_DIR)

.PHONY: $$($$(name)_NAME)-clean
$$($$(name)_NAME)-clean:
	rm -rf $$($$(name)_WORK_DIR)

.PHONY: $$($$(name)_NAME)
$$($$(name)_NAME):: $$($$(name)_NAME)-init
endef


define fetch_source
name=$(call uc,$(1))
$$(name)_LOCATION?=$(2)
$$(name)_PACKED_SRC?=$$($$(name)_WORK_DIR)/$$($$(name)_FULLNAME).tar.xz
$$(name)_UNPACK_CMD?=tar xf $$($$(name)_PACKED_SRC) -C $$($$(name)_SRC_DIR)
$$(name)_UNPACKED_DIR_NAME?=$$($$(name)_FULLNAME)
$$(name)_SRC?=$$($$(name)_SRC_DIR)/$$($$(name)_UNPACKED_DIR_NAME)

$$($$(name)_PACKED_SRC): | $$($$(name)_WORK_DIR)
	wget $$($$(name)_LOCATION) -O $$($$(name)_PACKED_SRC)
	touch $$@

$$($$(name)_SRC): $$($$(name)_PACKED_SRC) | $$($$(name)_SRC_DIR)
	$$($$(name)_UNPACK_CMD)
	touch $$@

.PHONY: $$($$(name)_NAME)-fetch
$$($$(name)_NAME)-fetch: $$($$(name)_SRC)

.PHONY: $$($$(name)_NAME)
$$($$(name)_NAME):: $$($$(name)_NAME)-fetch

.PHONY: $$($$(name)_NAME)
$$($$(name)_NAME):: $$($$(name)_NAME)-fetch
endef


define make_defconfig
name=$(call uc,$(1))
$$(name)_DEFCONFIG?=$(2)
$$(name)_CONFIG_DIR?=$(CONFIG_DIR)/target/$(1)

$$($$(name)_CONFIGURE_STAMP): $$($$(name)_CONFIG_DIR)/$$($$(name)_DEFCONFIG) $$($$(name)_SRC) | $$($$(name)_BUILD_DIR) 
	cp $$($$(name)_CONFIG_DIR)/$$($$(name)_DEFCONFIG) $$($$(name)_SRC)/$(3)
	$(SDK_MAKE) -C $$($$(name)_SRC) O=$$($$(name)_BUILD_DIR) $$($$(name)_DEFCONFIG)
	touch $$@


.PHONY: $$($$(name)_NAME)-defconfig
$$($$(name)_NAME)-defconfig: $$($$(name)_CONFIGURE_STAMP)

.PHONY: $$($$(name)_NAME)-menuconfig
$$($$(name)_NAME)-menuconfig: 
	$(SDK_MAKE) -C $$($$(name)_SRC) O=$$($$(name)_BUILD_DIR) menuconfig

.PHONY: $$($$(name)_NAME)
$$($$(name)_NAME):: $$($$(name)_NAME)-defconfig
endef


define make_build
name=$(call uc,$(1))
$$($$(name)_BUILD_STAMP): $$($$(name)_CONFIGURE_STAMP) | $$($$(name)_BUILD_DIR) 
	$(SDK_MAKE) -C $$($$(name)_SRC) O=$$($$(name)_BUILD_DIR)
	touch $$@

.PHONY: $$($$(name)_NAME)-build
$$($$(name)_NAME)-build: $$($$(name)_BUILD_STAMP)

.PHONY: $$($$(name)_NAME)
$$($$(name)_NAME):: $$($$(name)_NAME)-build
endef

define make_install
name=$(call uc,$(1))
$$($$(name)_INSTALL_STAMP): $$($$(name)_BUILD_STAMP) | $$($$(name)_INSTALL_DIR) 
	$(SDK_MAKE) -C $$($$(name)_SRC) O=$$($$(name)_BUILD_DIR) install
	cp -r $$($$(name)_BUILD_DIR)/$(2)/* $$($$(name)_INSTALL_DIR)
	touch $$@

.PHONY: $$($$(name)_NAME)-install
$$($$(name)_NAME)-install: $$($$(name)_INSTALL_STAMP)

.PHONY: $$($$(name)_NAME)
$$($$(name)_NAME):: $$($$(name)_NAME)-install
endef

define stage_app
name=$(call uc,$(1))
$$(name)_APP_DIR=$(ROOTFS_DIR)/apps/$(1)

$$($$(name)_STAGE_STAMP): $$($$(name)_INSTALL_STAMP) | $(ROOTFS_DIR)
	mkdir -pv $$($$(name)_APP_DIR)
	cp -r $$($$(name)_INSTALL_DIR)/* $$($$(name)_APP_DIR)
	touch $$@

.PHONY: $$($$(name)_NAME)-stage
$$($$(name)_NAME)-stage: $$($$(name)_STAGE_STAMP)

.PHONY: $$($$(name)_NAME)
$$($$(name)_NAME):: $$($$(name)_NAME)-stage
endef


$(eval $(call init_package,busybox,1.34.0))
$(eval $(call fetch_source,busybox,https://www.busybox.net/downloads/$(BUSYBOX_FULLNAME).tar.bz2))
$(eval $(call make_defconfig,busybox,nybble_defconfig,configs/))
$(eval $(call make_build,busybox))
$(eval $(call make_install,busybox,_install))
$(eval $(call stage_app,busybox))

.PHONY: all
all:: busybox
