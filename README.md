# libraw-colordata
Libraw does not seem very well documented. Instead of reading the code, which could be a good idea probably, this repository is an attempt at analysing it's behavior using the data it provides. Specifically it's focused on the `libraw_colordata_t` struct.

### Cloning the dataset
The dataset is from [raw.pixls.us](https://raw.pixls.us). You can clone it with this command

`rsync -avL rsync://raw.pixls.us/data/ raw-pixls-us-data/`

## Current Data
This repository currently hosts two data files. They contain the same data in different formats. These files are *incomplete* due to a crash while processing images from the Raspberry Pi.

**`colordata.csv`** is a csv file and **`colordata_from_csv.tbtl`** is a file in the [Tablatal][tbtl] format. The Tablatal file has headers, which are explained here.

*COMPANY, MODEL, IMAGE*: The Company that procuded that Model and which Image file it is. This is enough data for you to locate the image in the dataset.

*BLCK, CH_BLCK*: The global black level correct and per-channel black level correction values.

*MAX, CHLIN_MAX*: The maximum value as seen by the value or hardcoded and the per-channel linear maximum values, or zero if the file contained none.

*FL_USED*: The level of flash used

*CLR_SPACE*: Colorspace enum ID. Corresponds to a value in `colorspaces.csv`

[tbtl]: https://wiki.xxiivv.com/site/tablatal.html