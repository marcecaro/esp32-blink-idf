
# Setup environment

First run the setup script, to install the local ESP-IDF, to build the components directory:
```bash
./setup.sh
```

After that, source the local export script:

```bash
. local-export.sh
```

install the dependencies:

```bash
. ./project-export.sh && idf.py reconfigure
```



```bash
. project-export.sh && idf.py add-dependency "espressif/arduin
o-esp32" 
```

# Build

```bash
cargo build
```

# Flash

```bash
cargo flash
```

# Monitor

```bash
cargo monitor
```