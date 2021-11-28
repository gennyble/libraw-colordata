mod database;

use std::{
    fmt,
    fs::{read_dir, DirEntry},
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

use csv::WriterBuilder;
use database::PixlsDatabase;
use libraw::{Colordata, Processor};
use tablatal::{AsRecord, Tablatal};

fn main() {
    let pixls_us_repo = std::env::args()
        .skip(1)
        .next()
        .ok_or("Please pass the path of the raw.pixls.us repository as the first argument")
        .unwrap();

    let possible_arg = std::env::args().skip(2).next();

    match possible_arg.as_deref() {
        Some("colordata_csv-to-tbtl") => {
            colordata_csvtbtl();
            std::process::exit(0);
        }
        _ => (),
    }

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

    make_colordata(pixls_us_repo);
}

fn make_colordata<P: AsRef<Path>>(pixls_us_repo: P) {
    let before_parse = Instant::now();
    println!("Parsing data...");

    let mut csv_writer = WriterBuilder::new().from_path("colordata.csv").unwrap();

    let mut data = vec![];
    let pixls_db = PixlsDatabase::from_path(pixls_us_repo.as_ref()).unwrap();
    for company in pixls_db.company_iter() {
        let company_start = Instant::now();
        println!("Starting on {}", company.name);

        for model in company.model_iter() {
            for image in model.image_iter() {
                let img_name = image.file_stem().unwrap().to_str().unwrap_or("");

                match std::fs::read(&image) {
                    Err(e) => {
                        eprintln!(
                            "Failed to read image '{}'. Model {} from {}.\n\t{}",
                            img_name, model.name, company.name, e
                        );
                        continue;
                    }
                    Ok(buf) => match Processor::new().decode(&buf) {
                        Err(e) => {
                            eprintln!(
                                "Failed to decode image '{}'. Model {} from {}.\n\t{}",
                                img_name, model.name, company.name, e
                            );
                            continue;
                        }
                        Ok(raw) => {
                            let camdata = CameraColordata {
                                company: company.name.as_str(),
                                model: model.name.as_str(),
                                image: img_name,
                                colordata: raw.color(),
                            };

                            csv_writer.write_record(camdata.as_record()).unwrap();

                            data.push(camdata);
                        }
                    },
                }
            }
        }

        csv_writer.flush().unwrap();
        println!(
            "Finished files for {}. Took {:.2}s",
            company.name,
            Instant::now().duration_since(company_start).as_secs_f32()
        );
    }

    println!(
        "Finished parsing! Took {:.2}s\nBuilding and saving Tablatal... ",
        Instant::now().duration_since(before_parse).as_secs_f32()
    );

    let before_tbtl = Instant::now();
    let mut tbtl = Tablatal::new();

    for point in data {
        tbtl.push(point);
    }

    tbtl.save_file("colordata.tbtl").unwrap();

    println!(
        "Finished! Took {:.2}s",
        Instant::now().duration_since(before_tbtl).as_secs_f32()
    );
}

struct CameraColordata<'a> {
    company: &'a str,
    model: &'a str,
    image: &'a str,
    colordata: Colordata,
}

struct CameraTbtl {
    records: Vec<String>,
}

impl AsRecord for CameraTbtl {
    fn as_record(&self) -> Vec<String> {
        self.records.clone()
    }

    fn headers() -> &'static [&'static str] {
        &[
            "COMPANY",
            "MODEL",
            "IMAGE",
            "BLCK",
            "CH_BLCK",
            "MAX",
            "CHLIN_MAX",
            "FL_USED",
            "CLR_SPACE",
        ]
    }
}

impl<'a> AsRecord for CameraColordata<'a> {
    fn as_record(&self) -> Vec<String> {
        fn stry<T: fmt::Display>(v: T) -> String {
            format!("{}", v)
        }

        fn sf32(v: f32, precision: usize) -> String {
            format!("{:.*}", precision, v)
        }

        fn quad<T: fmt::Display>(a: &[T]) -> String {
            format!("{},{},{},{}", a[0], a[1], a[2], a[3])
        }

        vec![
            self.company.to_owned(),
            self.model.to_owned(),
            self.image.to_owned(),
            stry(self.colordata.black),
            quad(&self.colordata.black_per_channel),
            stry(self.colordata.maximum),
            quad(&self.colordata.linear_max),
            sf32(self.colordata.flash_used, 1),
            stry(self.colordata.exif_color_space.as_sys()),
        ]
    }

    fn headers() -> &'static [&'static str] {
        &[
            "COMPANY",
            "MODEL",
            "IMAGE",
            "BLCK",
            "CH_BLCK",
            "MAX",
            "CHLIN_MAX",
            "FL_USED",
            "CLR_SPACE",
        ]
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

fn colordata_csvtbtl() {
    let mut reader = csv::ReaderBuilder::new()
        .from_path("colordata.csv")
        .unwrap();

    let mut tbtl = Tablatal::new();

    for rcd in reader.records() {
        let rcds: Vec<String> = rcd.unwrap().iter().map(|s| s.to_owned()).collect();
        tbtl.push(CameraTbtl { records: rcds });
    }

    tbtl.save_file("colordata_from_csv.tbtl").unwrap();
}
