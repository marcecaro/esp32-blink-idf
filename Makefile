# Specify bash as the shell for make
SHELL := /bin/bash
ARDUINO_PACKAGES_PATH := ~/workbench/arduino-packages


clean-idf:
	-source ./project-export.sh && idf.py clean

clean-idf-full:
	-source ./project-export.sh && idf.py fullclean
	rm -rf build

clean-cargo:
	cargo clean

clean-full-idf-cargo: clean-idf-full clean-cargo
	rm -rf managed_components
	rm -rf target
	rm -rf build
	rm -rf .embuild

menuconfig-idf:
	-source ./project-export.sh && idf.py menuconfig
	echo "ESP-IDF Menuconfig completed."

reconfigure-idf:
	source ./project-export.sh && idf.py reconfigure && idf.py update-dependencies
	echo "ESP-IDF Reconfigured."

build-cargo:
	cargo build 

flash:build
	cargo espflash flash --monitor
	echo "Flash completed."

flash-jtag: build-cargo
	openocd -f esp32-jtag.cfg -c "program target/xtensa-esp32-espidf/debug/esp32-blink-idf verify reset exit"
	echo "JTAG Flash completed."

monitor:
	cargo espflash monitor
	echo "MOnitoring completed."
	
refresh-idf-deps:
	source ./project-export.sh && idf.py reconfigure && idf.py update-dependencies

install-idf-tools:
	source idf.env && ${IDF_PATH}/install.sh 
	cargo update

build: build-cargo
	echo "Build cargo finished"

clean: clean-idf clean-cargo 
	echo "Clean completed."

build-all: build-cargo
	echo "Build completed."

bootstrap: full-clean # uses system esp-idf, it needs to be installed
	-source $$HOME/export-esp.sh && cargo clean
	-source $$HOME/export-esp.sh && cargo update
	-source $$HOME/export-esp.sh && cargo build
	-source project-export.sh && ${IDF_PATH}/install.sh 
	-source project-export.sh && cargo update
	#-cargo install bindgen-cli
	@echo ""
	@echo ""
	@echo "Bootstrap completed."
	@echo "Close and open a new terminal."
	@echo ""

openicd:
	bash -c 'trap "echo \"OpenOCD stopped, restarting...\"" INT; while true; do openocd -f esp32-jtag.cfg || true; sleep 1; done'
	@echo "OpenOCD loop exited."


.PHONY: openicd build-idf build-cargo build-all flash flash-jtag monitor refresh-deps install-idf-tools find-arduino-h find-arduino-cxx clean full-clean
