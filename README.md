# libraw-colordata
Libraw does not seem very well documented. Instead of reading the code, which could be a good idea probably, this repository is an attempt at analysing it's behavior using the data it provides. Specifically it's focused on the `libraw_colordata_t` struct.

### Cloning the dataset
The dataset is from [raw.pixls.us](https://raw.pixls.us). You can clone it with this command

`rsync -avL rsync://raw.pixls.us/data/ raw-pixls-us-data/`