`rfsx` is a simple XMODEM sender.

It sidesteps the OS serial device (`/dev/tty.usbserial...`) in favor of using
[`libftdi`](https://www.intra2net.com/en/developer/libftdi/).  This lets us
work around an apparent bug in the macOS serial driver, which cuts off midway
through a large file transfer.

For more details, see [this blog post](https://www.mattkeeter.com/blog/2022-05-31-xmodem/).
