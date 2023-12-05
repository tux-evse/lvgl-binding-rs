# lvgl-binding-rs
Embedded graphic lvgl Rust afb-v4 binding

## reference:
https://docs.lvgl.io/8/widgets/index.html

## testing without root privilege

1) In order to test without admin privileges, you need access to

* /dev/fb0 generally with 'video' group
* /dev/input0 usually with 'input' group

2) find which virtual console hold /dev/fb0 framebuffer

* switch virtual console with (Alt+Ctrl+F1) (Alt+Ctrl+F2) ...
* after login check with cat /dev/urandom >/dev/fb0

If your screen is repainted, then you are on /dev/fb0 other wise switch to next virtual console.

3) start display-binding

```
display-binding/afb-binding/etc/binding-test.sh
```

## Demo screen on framebuffer

![LVGL demo screen](Docs/lvgl-demo-screen.png)

## Demo afb-v4 API

![LVGL demo api](Docs/lvgl-demo-api.png)


4) Rust LVGL api sample

Check Display::draw_panel within display-lvgl.rs for more samples

Create a label
```Rust
LvglLabel::new("Label-1", "Tux EvSe UI", 50, 400)
            .set_info("Demo Label widget")
            .set_size(300, 100)
            .set_color(LvglColor::rvb(0, 0, 0))
            .set_background(LvglColor::rvb(0xFF, 0xFF, 0xFF))
            .set_border(3, LvglColor::rvb(0, 0xff, 0))
            .finalize();
```

Create a button
```Rust
LvglButton::new("Button-A", "Test-1", 100, 200)
    .set_info("Demo Button 1")
    .set_size(180, 100)
    .finalize();
```

5) Faire un screencast du framebuffer

Copy framebuffer and transform it yo PNG. If needed crop image to content with gimp.
```
cp /dev/fb0 > /tmp/screen.data
RESOLUTION="1920x1080"
ffmpeg -vcodec rawvideo -f rawvideo -pix_fmt rgb32 -s $RESOLUTION -i /tmp/screen.data -f image2 -vcodec png screenshot.png
```
