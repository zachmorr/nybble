################################################################################
#
# foo
#
################################################################################

USB_VERSION = 0.0
USB_SITE = /home/zach/projects/nybble/bsp/packages/usb
USB_SITE_METHOD=local
USB_LICENSE = GPL-3.0+
USB_LICENSE_FILES = COPYING

# Buildroot does not fetch dependencies for local SITE
define USB_CARGO_FETCH                             
     cd $(USB_SRCDIR) && PATH=$(HOST_DIR)/bin:$(PATH) $(PKG_CARGO_ENV) cargo fetch                        
endef
                                                                               
USB_POST_RSYNC_HOOKS += USB_CARGO_FETCH

$(eval $(cargo-package))