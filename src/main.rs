use std::{
    fs::{read_dir, DirEntry},
    path::{Path, PathBuf},
};

use csv::WriterBuilder;

fn main() {
    let pixls_us_repo = std::env::args()
        .skip(1)
        .next()
        .ok_or("Please pass the path of the raw.pixls.us repository as the first argument")
        .unwrap();

    if !PathBuf::from("colorspace.csv").exists() {
        print!("Attempting to create colorspace csv... ");
        match make_colorspace_file("colorspaces.csv") {
            Err(e) => {
                println!("Failed! Quitting...\n\t{}", e);
                std::process::exit(1);
            }
            Ok(()) => {
                println!("Success!");
            }
        }
    } else {
        println!("Colorspace csv exists, skipping creation.")
    }

    //let db_read = read_dir(pixls_us_repo).unwrap();
    print_dir(0, pixls_us_repo);
}

pub fn print_dir<P: AsRef<Path>>(level: usize, path: P) {
    let indents: String = std::iter::repeat("    ").take(level).collect();
    let folder = '🗀';
    let file = '📄';

    for entry in read_dir(path.as_ref()).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let stem = path.file_stem().unwrap();
        let ftype = entry.file_type().unwrap();

        if ftype.is_dir() {
            println!("{}{} {}", indents, folder, &stem.to_string_lossy());
            print_dir(level + 1, &path);
        } else {
            println!("{}{} {}", indents, file, &stem.to_string_lossy());
        }
    }
}

// Make the "colorspaces.csv" file. The docs for ExifColorSpace on colordata_t
// say it can be unknown, sRGB, or Adobe. The thing is: Adobe doesn't exist,
// but AdobeRGB does. Also there are a bunch more defintions than that, so
// should I believe the docs? Probably not. It makes this csv so we can
// reference it later if we get a colorspace that doesn't map to the mentioned
// three.
pub fn make_colorspace_file<P: AsRef<Path>>(path: P) -> Result<(), csv::Error> {
    #[rustfmt::skip]
    let mut colorspaces = vec![
        ("ICC", libraw_sys::LibRaw_colorspace_LIBRAW_COLORSPACE_ICC),
        ("sRGB", libraw_sys::LibRaw_colorspace_LIBRAW_COLORSPACE_sRGB),
        ("Unknown", libraw_sys::LibRaw_colorspace_LIBRAW_COLORSPACE_Unknown),
        ("AdobeRGB", libraw_sys::LibRaw_colorspace_LIBRAW_COLORSPACE_AdobeRGB),
        ("NotFound", libraw_sys::LibRaw_colorspace_LIBRAW_COLORSPACE_NotFound),
        ("CameraGamma", libraw_sys::LibRaw_colorspace_LIBRAW_COLORSPACE_CameraGamma),
        ("ProPhotoRGB", libraw_sys::LibRaw_colorspace_LIBRAW_COLORSPACE_ProPhotoRGB),
        ("CameraLinear", libraw_sys::LibRaw_colorspace_LIBRAW_COLORSPACE_CameraLinear),
        ("Uncalibrated", libraw_sys::LibRaw_colorspace_LIBRAW_COLORSPACE_Uncalibrated),
        ("WideGammutRGB", libraw_sys::LibRaw_colorspace_LIBRAW_COLORSPACE_WideGamutRGB),
        ("MonochromeGamma", libraw_sys::LibRaw_colorspace_LIBRAW_COLORSPACE_MonochromeGamma),
        ("CameraGammaUniWB", libraw_sys::LibRaw_colorspace_LIBRAW_COLORSPACE_CameraGammaUniWB),
        ("MonohcromeLinear", libraw_sys::LibRaw_colorspace_LIBRAW_COLORSPACE_MonochromeLinear),
        ("CameraLinearUniWB", libraw_sys::LibRaw_colorspace_LIBRAW_COLORSPACE_CameraLinearUniWB)
    ];
    colorspaces.sort_by(|a, b| a.1.cmp(&b.1));

    let mut writer = WriterBuilder::new().from_path(path)?;

    for (name, number) in colorspaces {
        writer.write_record(&[&format!("{}", number), name])?;
    }

    writer.flush()?;

    Ok(())
}
