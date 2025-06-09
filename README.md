# Tea
A collection of Arch Dockerfiles to build specialized Recon, OSINT & Pentesting containers, simplified through the teashop custom launcher built with Rust.

## Legal & Ethical Considerations
Using Tea containers to interact with unauthorized systems is illegal and unethical. This tool is meant solely for educational and research purposes within controlled environments where you have explicit permission. Acceptable use cases include:

- CTF Challenges
- Security Research
- Penetration Testing & Red Teaming Engagements (with proper authorization)
- Ethical Hacking scenarios where any parties involved consent to its use

Unauthorized use of Tea and its containers may violate cybercrime laws, including but not limited to the Computer Fraud and Abuse Act (CFAA) and General Data Protection Regulation (GDPR). Misuse can lead to legal consequences, including criminal charges and civil liability.

Additionally, improper use of Tea containers could pose serious security risks, including data breaches and full-system compromise. Deploying this tool on unsecured networks, personal devices, or any unauthorized environment could expose you to threats beyond your control.

By using Tea and any related Docker Image and Container, you agree to use them ethically and legally. You take full responsibility for how this tool is used. This tool must never be used for unauthorized access, military applications or unlawful financial gain.

## Using Tea
The tea framework is used through the teashop binary. It allows the user to instance and access their tea brews at any time using the ~/.tea directory as the flavors database.

## Brewing Tea
To brew specific tea flavors it's necessary to initialize tea's base brew, this is done automatically by the install.sh script. If you ever delete your tea:base image you will need to clone the repository again and run the init.sh script manually.

Installation is done via:

Debian:
``` bash
sudo apt install docker
git clone https://github.com/XoanOuteiro/tea && cd tea
chmod +x install.sh && ./install.sh
```

Arch:
``` bash
sudo pacman -Sy docker
git clone https://github.com/XoanOuteiro/tea && cd tea
chmod +x install.sh && ./install.sh
```

This process should not have any errors, if it does please open an issue.
You can check for tea's basic image using:

``` bash
docker images | grep tea
```

## Using teashop
When the install process is finished your system will have the teashop binary. You can access it at any time using:

``` bash
teashop
```

Teashop is extremely simple and has only 4 commands:

Seeing all available options:
``` bash
teashop help
```
Seeing all available flavors:
``` bash
teashop flavors
```

Seeing all available brews:
``` bash
teashop brews
```

Creating a brew:
``` bash
teashop brew <flavor>
```

Drinking a brew:
``` bash
teashop drink <flavor>
```

## Tea Flavors

| Flavor | Focus |
| --- | --- |
| sencha | Demonstrates Tea's usage, but doesn't install any tools apart from the ones that the base brew uses |
| genmaicha | General tools for DNS enumeration and subdomain discovery, mostly passive methodologies |
| gyokuro | Web-Recon, includes crawling tools, fuzzers, DNS recon and Subdomain enum tools |
| hojicha | General tools for host-pentesting on local networks |

## Creating flavors
Simply create a folder under ~/.tea/flavors/ with your desired flavor name, then, add a Dockerfile using Sencha's flavor dockerfile as a base and add tools and configurations as needed. You may also add a description.txt file to be displayed when using the flavors listing.
