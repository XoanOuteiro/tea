# Tea
A collection of Arch Dockerfiles to build specialized Recon, OSINT & Pentesting containers, simplified through a custom launcher built with Bash Scripts.

## Using Tea
Using a tea requires 2 simplified steps:

- Brewing tea (Building an specific flavor's Docker Images)
- Drinking tea (Creating, starting and attaching to a container)

## Brewing Tea
To brew specific tea flavors it's necessary to initialize tea's base brew (don't worry, this only needs to be done once), this can be done through:

Debian:
``` bash
sudo apt install docker
git clone https://github.com/XoanOuteiro/tea && cd tea
chmod +x init.sh && ./init.sh
```

Arch:
``` bash
sudo pacman -Sy docker
git clone https://github.com/XoanOuteiro/tea && cd tea
chmod +x init.sh && ./init.sh
```

This process should not have any errors, if it does please open an issue.
You can check for tea's basic image using:

``` bash
docker images | grep tea
```

If you have tea's base image, you can start brewing specific flavors:

``` bash
chmod +x brew.sh
./brew.sh [flavor]
```

## Drinking Tea
Once your flavor has been brewed, you can drink it using:

``` bash
chmod +x drink.sh
./drink.sh [brew]
```

As long as you don't delete your container, you can start and attach it again just by doing the exact same command:

``` bash
./drink.sh [brew]
```
## Tea Flavors

| Flavor | Focus |
| --- | --- |
| sencha | Demonstrates Tea's usage, but doesn't install any tools apart from the ones that the base brew uses |
| genmaicha | General tools for DNS enumeration and subdomain discovery, mostly passive methodologies |
| kyoguro | Web-Recon, includes crawling tools, fuzzers, DNS recon and Subdomain enum tools |
| hojicha | General tools for host-pentesting on local networks |
