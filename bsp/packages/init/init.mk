################################################################################
#
# Init
#
################################################################################

INIT_VERSION = 0.0
INIT_SITE = /home/zach/projects/nybble/bsp/packages/init
INIT_SITE_METHOD=local

define INIT_INSTALL_TARGET_CMDS
     cp $(@D)/init $(TARGET_DIR)/nybble/boot/init
endef

$(eval $(generic-package))