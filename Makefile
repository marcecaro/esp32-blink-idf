# Specify bash as the shell for make
SHELL := /bin/bash
ARDUINO_PACKAGES_PATH := ~/workbench/arduino-packages

clean:
	source ./project-export.sh && idf.py fullclean
	rm -rf build

full-clean: clean
	rm -rf managed_components
	cargo clean
	rm -rf target

build-idf:
	source ./project-export.sh && idf.py build
	echo "ESP-IDF Build completed."

flash:
	source ./project-export.sh && idf.py flash
	echo "Flash completed."

monitor:
	source ./project-export.sh && idf.py monitor
	
refresh-deps:
	source ./project-export.sh && idf.py reconfigure && idf.py update-dependencies

install-idf-tools:
	source idf.env && ${IDF_PATH}/install.sh 

find-arduino-h:
	find ${ARDUINO_PACKAGES_PATH} -name '*.h' | xargs dirname 2>/dev/null | sort | uniq

find-arduino-cxx:
	find ${ARDUINO_PACKAGES_PATH} -name '*.cpp' -o -name '*.c' | xargs dirname 2>/dev/null | sort | uniq

build-cargo:
	cargo build

build-all: build-idf build-cargo
	echo "Build completed."

.PHONY: build-idf build-cargo build-all flash monitor refresh-deps install-idf-tools find-arduino-h find-arduino-cxx clean full-clean

