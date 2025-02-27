
# Assets

## Chichen Itza Image License
Image: "Chichen itza.jpg" by Daniel Schwen, CC BY-SA 4.0. Available at: https://en.wikipedia.org/wiki/File:Chichen_Itza_3.jpg

## Commands used to generate the assets


### YUYV to I420
```
ffmpeg -f rawvideo -pix_fmt yuyv422 -s 1280x680 -i chichen_itza.yuyv -f rawvideo -pix_fmt yuv420p chichen_itza.yuyv_i420
```

### BGRA to I420
```
ffmpeg -f rawvideo -pix_fmt bgra -s 1280x680 -i chichen_itza.bgra -f rawvideo -pix_fmt yuv420p chichen_itza.bgra_i420
```

### JPEG to NV12
```
ffmpeg -i chichen_itza.jpg -f rawvideo -pix_fmt nv12 chichen_itza.nv12
```

### NV12 to I420
```
ffmpeg -f rawvideo -pix_fmt nv12 -s 1280x680 -i chichen_itza.nv12 -f rawvideo -pix_fmt yuv420p chichen_itza.nv12_i420
```

### JPEG to RGB24
```
ffmpeg -i chichen_itza.jpg -pix_fmt rgb24 chichen_itza.rgb
```

### BGRA to RGB24
```
ffmpeg -f rawvideo -pix_fmt bgra -s 1280x680 -i chichen_itza.bgra -f rawvideo -pix_fmt rgb24 chichen_itza.rgb
```