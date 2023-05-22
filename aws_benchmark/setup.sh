#!/bin/bash
sudo apt-get update
sudo apt-get install -y bison
sudo apt-get install -y build-essential
sudo apt-get install -y cmake
sudo apt-get install -y coinor-cbc 
sudo apt-get install -y coinor-libcbc-dev
sudo apt-get install -y cvc4
sudo apt-get install -y default-jre
sudo apt-get install -y default-jdk
sudo apt-get install -y flex
sudo apt-get install -y libboost-all-dev
sudo apt-get install -y libgmp-dev
sudo apt-get install -y libssl-dev
sudo apt-get install -y libwww-perl
sudo apt-get install -y m4
sudo apt-get install -y python3-pip
sudo apt-get install -y ufw
sudo apt-get install -y time
sudo apt-get install -y zsh

sudo ufw allow 7766

#build ABY
cd ~
if [[ ! -z "~/ABY" ]]; then 
    git clone https://github.com/edwjchen/ABY.git
    cd ~/ABY && git checkout functions && mkdir build && cd build
    cmake .. -DABY_BUILD_EXE=On -DCMAKE_BUILD_TYPE=Release
    make 
fi

#build HyCC
cd ~
if [[ ! -z "~/HyCC" ]]; then 
    git clone https://gitlab.com/edwjchen/HyCC.git
    cd ~/HyCC
    make minisat2-download
    make
fi
