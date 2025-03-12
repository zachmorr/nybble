################################################################################
#
# foo
#
################################################################################

HELLO_VERSION = 0.0
# HELLO_SOURCE = foo-$(HELLO_VERSION).tar.gz
HELLO_SITE = /home/zach/projects/nybble/bsp/packages/hello
HELLO_SITE_METHOD=local
HELLO_LICENSE = GPL-3.0+
HELLO_LICENSE_FILES = COPYING

$(eval $(cargo-package))