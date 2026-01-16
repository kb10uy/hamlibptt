# HamlibPTT - Use Hamlib with MMSSTV/MMTTY/MMVARI

Control PTT via Hamlib from MM softwares.

## Usage

1. Download `hamlibptt.fsk` file from [Releases](https://github.com/kb10uy/hamlibptt/releases).
2. Place two files along with MMSSTV/MMTTY/MMVARI:
    - Downloaded `hamlibptt.fsk`
    - Config file `hamlibptt.toml` (See [config.example.toml](./config.example.toml))
3. Launch the application and set PTT to **hamlibptt** in *Settings*.
    - *Radio Command* should be set to **NONE**.
4. If you configured correctly, a dialog will pop that says `Initialized successfully!`!
    - Unlike other EXTFSK, no additional window will be shown while running.
