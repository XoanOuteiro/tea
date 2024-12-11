# Start from the tea:base image
FROM tea:base

# Set working directory (optional, can be changed later)
WORKDIR /pentest

# Default command (this will run the zsh shell)
CMD ["/bin/zsh"]

# Update system & Install tools
RUN pacman -Syu --noconfirm && \
	yay -S --noconfirm sherlock \
	perl-image-exiftool \
	subfinder \
	sublist3r \
	dnsenum \
	searchsploit \
	nmap \
	rustscan \
	dig \
	nslookup \
	whois \
	ldns

