# Start from the tea:base image
FROM tea:base

# Set working directory (optional, can be changed later)
WORKDIR /pentest

# Default command (this will run the zsh shell)
CMD ["/bin/zsh"]

# Update system
RUN pacman -Syu --noconfirm

# Install tools
RUN pacman -S --noconfirm \
    wfuzz \
    ffuf \
    feroxbuster \
    wafw00f \
    whatweb \
    yay

# Install additional tools via yay
RUN yay -S --noconfirm sherlock \
    subfinder \
    sublist3r \
    dnsenum \
    nmap \
    rustscan \
    dig \
    nslookup \
    whois \
    ldns

# Install Golang
RUN pacman -S --noconfirm go

# Add Go binaries to PATH
ENV PATH="/root/go/bin:${PATH}"

# Install Go tools
RUN CGO_ENABLED=1 go install github.com/projectdiscovery/katana/cmd/katana@latest
RUN go install -v github.com/projectdiscovery/httpx/cmd/httpx@latest

