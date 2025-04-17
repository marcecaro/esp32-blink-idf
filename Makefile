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

build-idf:
	source ./project-export.sh && idf.py -v build 
	echo "ESP-IDF Build completed."

build-cargo:
	cargo build 

flash:
	idf.py flash
	echo "Flash completed."

monitor:
	espflash monitor
	echo "Flash completed."
	
refresh-idf-deps:
	source ./project-export.sh && idf.py reconfigure && idf.py update-dependencies

install-idf-tools:
	source idf.env && ${IDF_PATH}/install.sh 
	cargo update

build: build-cargo
	echo "Build cargo finished"


build-all: build-idf
	echo "Build completed."

bootstrap: full-clean
	-unset IDF_PATH && cargo clean
	-cargo update
	-unset IDF_PATH && cargo build
	-source idf.env && ${IDF_PATH}/install.sh 
	-cargo update
	-cargo install bindgen-cli
	@echo ""
	@echo ""
	@echo "Bootstrap completed."
	@echo "Close and open a new terminal."
	@echo ""

extract-includes:
	idf.py reconfigure
	cmake -P extract_includes.cmake

.PHONY: build-idf build-cargo build-all flash monitor refresh-deps install-idf-tools find-arduino-h find-arduino-cxx clean full-clean

