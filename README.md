# GSP : Lecteur d'écran avec reconnaissance automatique de langue

## Description

GSP est un outil polyvalent de lecture d'écran et de reconnaissance de texte qui offre :

- Lecture de texte à partir de différentes sources
- Reconnaissance optique de caractères (OCR)
- Traduction automatique
- Support multilingue
- Personnalisation des moteurs de synthèse vocale et de traduction

## Table des matières

1. [Prérequis](#prérequis)
2. [Installation](#installation)
3. [Commandes et Raccourcis Clavier](#commandes-et-raccourcis-clavier)
4. [Utilisation](#utilisation)
5. [Construction du Projet](#construction-du-projet)
6. [Options de Ligne de Commande](#options-de-ligne-de-commande)

## Prérequis

### Dépendances système

Avant d'utiliser GSP, installez les dépendances suivantes :

```bash
apt install libttspico-utils \
            espeak \
            mbrola \
            mbrola-fr4 \
            tesseract-ocr-fra \
            xfce4-screenshooter \
            paplay
```

```bash
apt install espeak-ng \
            libttspico-utils
```

### Langages et Outils

- Rust (dernière version stable)
- Cargo
- Docker (optionnel)

## Installation

### Installation standard

```bash
# Construire le projet
cargo build --release

# Déplacer l'exécutable
cp ./target/release/gsp ~/.local/bin/
```

### Installation Docker

```bash
# Construire l'image Docker
docker build -t gsp .

# Créer un conteneur
docker create --name gsp gsp

# Extraire l'exécutable
docker cp gsp:gsp .

# Déplacer l'exécutable
cp ./gsp ~/.local/bin/
```

## Commandes et Raccourcis Clavier

les raccourcis clavier sont à gérer par le système d'exploitation.
des exemples de raccourcis clavier sont fournis au-dessus des commandes pour un clavier azerty.

### Lecture de sélection avec reconnaissance automatique de langue

```bash
# attaché aux raccourcis clavier : win + ²
gsp --dev -s selection -y espeak --speed 2 --translation auto --engine-translation translate_locally
```

### Lecture de la sélection en anglais

```bash
gsp --dev -s selection -y espeak --speed 2 --translation en-US --engine-translation translate_locally
```

### Lecture du contenu écran avec OCR

```bash
# attaché aux raccourcis clavier avec: win + é
# Sans traduction
gsp -s ocr -y espeak --speed 2

# attaché aux raccourcis clavier avec: win + "
# Avec traduction  avec la source en anglais
gsp -s ocr -y espeak --speed 2 --translation en-US --engine-translation translate_locally
```

### Arrêt du lecteur

```bash
gsp --stop
```

## Utilisation

Exécutez simplement l'une des commandes mentionnées dans la section précédente.

## Construction du Projet

### Construction standard

```shell
cargo build --release
cp ./target/release/gsp ~/.local/bin/
```

### Construction Docker

```shell
docker build -t gsp .
docker create --name gsp gsp
docker cp gsp:gsp .
cp ./gsp ~/.local/bin/
```

## Options de Ligne de Commande

### Sources de texte

- `selection` : Lire le texte sélectionné
- `clipboard` : Lire le contenu du presse-papiers
- `file` : Lire un fichier
- `ocr` : Reconnaissance optique de caractères
- `stdin` : Lire depuis l'entrée standard

### Moteurs de Synthèse Vocale (TTS)

- `pico`
- `espeak`

### Moteurs de Traduction

- `libretranslate`
- `argos_translate`
- `translate_locally`

### Vitesses Supportées

- `0.6`, `0.8`, `1`, `1.2`, `1.4`, `1.6`, `1.8`, `2`, `2.2`

## Contribution

Les contributions sont les bienvenues ! Veuillez ouvrir une issue ou soumettre une pull request.
