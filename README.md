# Setup

1. Install rust 
    ```
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
2. Install espup (https://docs.esp-rs.org/book/installation/riscv-and-xtensa.html)
    ```
    cargo install espup
    cargo install cargo-generate
    cargo install cargo-espflash
    cargo install espflash

3. Install necessary toolchains: 
    ```
    espup install -v 1.74.0.1
4. Install the prerequisites for std Dev: https://docs.espressif.com/projects/esp-idf/en/latest/esp32/get-started/linux-macos-setup.html#for-linux-users
    
    1. Install prerequisite packages to compile ESP-IDF: 
        ```
        sudo apt-get install git wget flex bison gperf python3 python3-pip python3-venv cmake ninja-build ccache libffi-dev libssl-dev dfu-util libusb-1.0-0
    2. Clone ESP IDF
        ```
        mkdir -p ~/esp
        cd ~/esp
        git clone --recursive https://github.com/espressif/esp-idf.git
    3. Install ESP-IDF Tools
        ```
        cd ~/esp/esp-idf
        ./install.sh all
    4. Export env Vars. (Do this every time you open this project!)
        ```
        . $HOME/esp/esp-idf/export.sh
5. Install ldproxy for proxying linker args
    ```
    cargo install ldproxy
6. Install espflash
    1. Install libudev and libc6
        ```
        apt-get install libudev-dev libc6
    2. Install espflash itself
        ```
        cargo install cargo-espflash --locked
