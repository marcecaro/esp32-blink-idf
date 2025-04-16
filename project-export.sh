. idf.env
. ${IDF_PATH}/export.sh
echo | xtensa-esp32-elf-g++ -v -x c++ -E - 2>&1 | sed -n '/#include <...> search starts here:/,/End of search list./p' | sed '1d;$d' | awk '{$1=$1;print}' > .system-includes.txt

echo | xtensa-esp32-elf-g++ -dM -E - | sed -E 's/#define ([^ ]+)( (.+))?/-D\1=\3/' | sed 's/=$//' > .system-flags.txt
