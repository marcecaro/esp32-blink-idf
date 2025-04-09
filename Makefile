# Specify bash as the shell for make
SHELL := /bin/bash
ARDUINO_PACKAGES_PATH := ~/workbench/arduino-packages

clean:
	source ./project-export.sh && idf.py fullclean
	rm -rf build

full-clean: clean
	rm -rf managed_components

build:
	source ./project-export.sh && idf.py build
	echo "Build completed."

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
	
.PHONY: clean full-clean build flash monitor refresh-deps