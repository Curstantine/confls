FROM archlinux:latest

# Configure hostname and users
RUN echo archie >> /etc/hostname
RUN useradd -m -G wheel -s /usr/bin/fish test

# Install base-devel (without sudo), git, fis, doas and rustup .
RUN pacman -Syu --needed base-devel --assume-installed sudo --noconfirm
RUN pacman -S git rustup fish doas --noconfirm

# Configure fish shell
RUN chsh -s /usr/bin/fish
RUN fish

# Configure doas
RUN echo "permit nopass :wheel" >> /etc/doas.conf
RUN chown root:root /etc/doas.conf && chmod 0400 /etc/doas.conf
RUN ln -s $(which doas) /usr/bin/sudo


# Set user and userhome as CWD
USER test
WORKDIR /home/test

# Configure rustup and cargo
RUN rustup default stable

# Install paru
RUN git clone https://aur.archlinux.org/paru.git
RUN cd paru && makepkg -si --noconfirm
