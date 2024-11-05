# Astromech API

## Installation
 On a clean raspberry pi running raspian os lite..

 1. Setup SSH
    
 3. Setup Rust
    - curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
      
 4. Add armv7-unknown-linux-gnueabihf (for Raspberry Pi 3/4) or aarch64-unknown-linux-gnu (for 64-bit Raspberry Pi OS):
    - rustup target add armv7-unknown-linux-gnueabihf
    - rustup target add aarch64-unknown-linux-gnu
      
 6.   Install Build Tools
      - sudo apt update && sudo apt install build-essential git cmake pkg-config
      - sudo apt install gdb-multiarch

7. Install Audio library
   - sudo apt-get install libasound2-dev (May not be necessary)
   - sudo apt install mp3info

 8.  Install Git
     - sudo apt install git
       
 10.  Clone the project
     - git clone https://github.com/ZachRich/AstromechAPI.git
