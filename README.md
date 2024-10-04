# mijn.host DDNS

Een Rust applicatie om dynamisch je IP adres in mijn.host up-to-date te houden.
Dit is nuttig als je een dynamisch public IP adres hebt (zoals vaak voorkomt bij
niet-commerciële internetabonnementen). Het script gebruikt [icanhazip.com](https://icanhazip.com/)
om je public IP vast te stellen en de API van mijn.host om de records aan te passen.

## Features

- Ondersteunt IPv4 en IPv6 (A en AAAA records)
- Simpele console logging
- Ingebouwde timer (optioneel)
- Records aanmaken en verwijderen gebaseerd op beschikbare IPs (optioneel)

## Gebruik

Het programma is te compilen en installeren met Cargo met het volgende commando.

```
cargo install --git https://github.com/ldobbelsteen/mijn-host-ddns
```

Dan kan het als volgt gebruikt worden.

```
mijn-host-ddns <config-bestand-pad>
```

Het configuratiebestand is in TOML formaat. Een voorbeeld kan je vinden in het 
`example-config.toml` bestand met uitleg per instelling.

## Container

Een simpelere manier om te deployen is met een container. Er is een Containerfile
waarmee je een image kan bouwen. Voorgemaakte images worden automatisch naar GitHub
Packages geschreven in deze repo. Een voorbeeld van het gebruik van de image staat hieronder.

```
docker run \
    --detach \
    --restart on-failure \
    --volume /path/to/config:/config.toml \
    ghcr.io/ldobbelsteen/mijn-host-ddns
```
