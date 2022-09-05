env:
	# sudo apt update
	sudo apt install autoconf automake autotools-dev curl libmpc-dev libmpfr-dev libgmp-dev \
              gawk build-essential bison flex texinfo gperf libtool patchutils bc \
              zlib1g-dev libexpat-dev pkg-config  libglib2.0-dev libpixman-1-dev git tmux python3 ninja-build zsh -y
	cd ${HOME} && wget https://download.qemu.org/qemu-7.0.0.tar.xz
	cd ${HOME} && tar xvJf qemu-7.0.0.tar.xz
	cd ${HOME}/qemu-7.0.0 && ./configure --target-list=riscv64-softmmu,riscv64-linux-user
	cd ${HOME}/qemu-7.0.0 && make -j$(nproc)
	cd ${HOME}/qemu-7.0.0 && sudo make install
	qemu-system-riscv64 --version
	qemu-riscv64 --version
	curl https://sh.rustup.rs -sSf | sh -s -- -y
	source ${HOME}/.cargo/env
	rustc --version
