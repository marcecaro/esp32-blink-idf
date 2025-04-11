Based on: [ESP-IDF-TEMPLASTE](https://github.com/esp-rs/esp-idf-template/blob/master/README-cmake.md)

# Layout

The project is structured as follows:

```
.
├── components
│   ├── esp32servoserver  # cpp dependencies
│   |     ├── idf_component.yml # IDF component manifest, downloads them, needs to apear in CMakeLists
│   |     ├── CMakeLists.txt    # CMake configuration for CPP dependencies
│   |     └── include
│   |  
│   └── lx16a-servo
├── Cargo.toml            # Rust dependencies   
├── Makefile              # High level Makefile
├── README.md
├── SETUP-ENVIRONMENT.md  # Setup environment
├── target
├── wokwi.toml              # Wokwi configuration, emulator
├── src                     # Rust source code
├── CMakeLists.txt          # CMake configuration for CPP dependencies
└── project-export.sh       # Environment variables
```


# Setup environment

sudo apt install clang

Install:
    - ROS JAZZY
    - Rust

First run the setup script, to install the local ESP-IDF, to build the components directory:

```bash
make bootstrap
```
Then open a new terminal.

**bash.rc:**
```shell

## ROS2 Jazzy
source /opt/ros/jazzy/setup.bash && echo "ROS2 Jazzy Environment loaded successfully"
export PATH=$HOME/.local/bin:$PATH

#source $HOME/esp/esp-idf/export.sh 
source $HOME/.local/bin/env echo "~/local/bin/env  Environment loaded successfully"
source $HOME/.cargo/env && echo "Cargo Environment loaded successfully"


# Auto-source project-export.sh if it exists in the current directory
function check_project_export() {
  if [ -f "./project-export.sh" ]; then
    echo "Found project-export.sh in current directory, sourcing it..."
    source ./project-export.sh
  fi
}

check_project_export;
```


install the dependencies:

```bash
cargo update
make refresh-idf-deps
```

### Add idf-dependencies

In the component that needs the dependency, check the idf_component.yml file
Run `make` to download the dependency

### Add cpp-dependencies

- Check components/esp32servoserver

### Add cpp-dependencies from arduino registry

- Point the Arduino IDE to the ~/workbench/arduino-packages directory
- Insatall the package there
- is needed to add the idf dependency: espressif/arduino-esp32


### Generates Binding From .h to rust

```bash
mkdir -p src/lx16a
bindgen components/lx16a-servo/src/lx16a-servo.h -o src/lx16a/mod.rs
```

# Build

```bash
make build-idf
make build-cargo
```

# Flash

```bash
cargo flash
```

# Monitor

```bash
cargo monitor
```