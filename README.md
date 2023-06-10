# DHT11 driver

This is a [DHT11](https://www.mouser.com/datasheet/2/758/DHT11-Technical-Data-Sheet-Translated-Version-1143054.pdf) driver for rust.

This library uses `embedded_hal` to sleep micro order sec.
And the struct of DHT11 defined in this library takes embedded_hal's `Delay` struct.
This means you need to add an embedded_hal to your project.
