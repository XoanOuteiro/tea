# Start from the tea:base image
FROM tea:base

# Set working directory (optional, can be changed later)
WORKDIR /pentest

# Default command (this will run the zsh shell)
CMD ["/bin/zsh"]

# Update system & Install tools
RUN pacman -Syu --noconfirm && \
	pacman -S  --noconfirm \
	metasploit \
	gnu-netcat \
	john \
	seclists \
	hydra \
	medusa \
	ettercap \
	socat \
	curl \
	wget \
	hashcat \
	nmap \
	yay -S --noconfirm searchsploit

