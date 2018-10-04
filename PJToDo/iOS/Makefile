ARCHS = RustMake

.PHONY: $(ARCHS)
$(ARCHS): %:
	echo "$@"

	sh RustBuildScript.sh

.PHONY : clean
clean :
	sh RustCargoClean.sh
