# gsp

## dependencies

```bash
apt install libttspico-utils espeak mbrola mbrola-fr4 tesseract-ocr-fra xfce4-screenshooter paplay
```

## commands to be attached a keyboard shortcut

```bash
# selection playback with automatic language recognition
gsp --dev -s selection -y espeak --speed 2 --translation auto --engine-translation translate_locally

# read the selection
gsp --dev -s selection -y espeak --speed 2
gsp --dev -s selection -y espeak --speed 2 --translation en-US --engine-translation translate_locally
# read screen content with ocr
gsp -s ocr -y espeak --speed 2
gsp -s ocr -y espeak --speed 2 --translation en-US --engine-translation translate_locally
# stop
gsp --stop
```
