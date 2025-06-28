# Introduction

This project is used for analysing of many malicious IP addresses.

## Installation

Download all IP zones from https://www.ipdeny.com/ipblocks/ and extract to it to folder zones.

```bash
mkdir -p zones
curl -o zones/all-zones.tar.gz https://www.ipdeny.com/ipblocks/data/countries/all-zones.tar.gz
tar -xvzf zones/all-zones.tar.gz -C zones
```

## Usage

Pipe relevant IPs into this application and get the associated nets back.
```bash
echo "8.8.8.8"|./target/release/analyse-country-networks
```

Output

```
8.8.8.0/24
```
