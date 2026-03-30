################################################################################
#
# NYB
#
################################################################################

NYB_VERSION = 0.0
NYB_SITE = /home/zach/projects/nybble/bsp/packages/nyb
NYB_SITE_METHOD=local
NYB_LICENSE = GPL-3.0+
NYB_LICENSE_FILES = COPYING

# Buildroot does not fetch dependencies for local SITE
define NYB_CARGO_FETCH                             
     cd $(NYB_SRCDIR) && PATH=$(HOST_DIR)/bin:$(PATH) $(PKG_CARGO_ENV) cargo fetch                        
endef
                                                                               
NYB_POST_RSYNC_HOOKS += NYB_CARGO_FETCH

define NYB_BUILD_CMDS
	cd $(NYB_SRCDIR) && \
	$(TARGET_MAKE_ENV) \
	$(TARGET_CONFIGURE_OPTS) \
	$(PKG_CARGO_ENV) \
	$(NYB_CARGO_ENV) \
	cargo build \
		--offline \
		--bin nybd \
		$(if $$(BR2_ENABLE_DEBUG),,--release) \
		--manifest-path Cargo.toml \
		--locked \
		$(NYB_CARGO_BUILD_OPTS)
endef

# modified from packages/pgk-cargo.mk to accept workplace
define NYB_INSTALL_TARGET_CMDS
	cd $(NYB_SRCDIR) && \
	$(TARGET_MAKE_ENV) \
		$(TARGET_CONFIGURE_OPTS) \
		$(PKG_CARGO_ENV) \
		$(NYB_CARGO_ENV) \
		cargo install \
			--offline \
			--root $(TARGET_DIR)/usr/ \
			--bins \
			--path ./nybd \
			--force \
			--locked \
			-Z target-applies-to-host \
			$(NYB_CARGO_INSTALL_OPTS)
endef

$(eval $(cargo-package))