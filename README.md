# xcap
Capture X window

# requirements

- xwininfo
- ffmpeg

# usage

```console
# Usage: xcap [ffmpeg options[

# capture as an animated gif with framerate of 16
$ xcap -framerate 16 command-line.gif

# capture as a loopback video
$ sudo modprobe v4l2loopback
$ cargo run -- -f v4l2 /dev/video0
```

Licence
-------

MIT
