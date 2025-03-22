################################################################################
#
# USB Gadget
#
################################################################################

GADGET_VERSION = 0.0
GADGET_SITE = /home/zach/projects/nybble/bsp/packages/gadget
GADGET_SITE_METHOD=local
GADGET_LICENSE = GPL-3.0+
GADGET_LICENSE_FILES = COPYING

# Buildroot does not fetch dependencies for local SITE
define GADGET_CARGO_FETCH                             
     cd $(GADGET_SRCDIR) && PATH=$(HOST_DIR)/bin:$(PATH) $(PKG_CARGO_ENV) cargo fetch                        
endef
                                                                               
GADGET_POST_RSYNC_HOOKS += GADGET_CARGO_FETCH

$(eval $(cargo-package))