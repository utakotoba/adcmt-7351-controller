### ADCMT 7351 DMM USB I/O Controller

The goal of this project is to create a cross-platform I/O controller and a complementary host application for the ADCMT 7351A/E+03 digital multimeter (originally made by Advantest).

To keep costs down, the focus is solely on the USB interface, omitting the need for the expensive, official GPIB USB HS adapter.

The resulting package includes a simple yet practical host application, allowing users to interact with the device, take measurements, and generate basic plots.

The project is targeting cross-platform compatibility, with an emphasis on non-Windows operating systems. Instead of relying on the legacy official C and VBA APIs, this project provides a modern interface built with Rust. It's important to note that this is an amateur-level tool and is not intended to offer the advanced, production-grade features found in professional software like LabView.

> WIP...
