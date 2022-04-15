use std::fs::File;
use std::io::Read;
use std::iter::FromIterator;

use d3rs::data_collections::{CategorisedValue, CategorisedValues};

#[derive(Debug, Default)]
struct MusicRevenue<'mr> {
    format: &'mr str,
    year: u16,
    revenue_inflation_adjusted: isize,
}

impl<'i, 'mr> FromIterator<&'i str> for MusicRevenue<'mr>
where
    'i: 'mr,
{
    fn from_iter<T: IntoIterator<Item = &'i str>>(iter: T) -> Self {
        let mut cells = iter.into_iter();
        let format = cells.next().unwrap();
        let year = cells.next().unwrap().parse::<u16>().unwrap();
        cells.next(); // units
        cells.next(); // revenue, not inflation adjusted
        let revenue = cells.next().unwrap().parse::<isize>().unwrap();

        Self {
            year,
            format,
            revenue_inflation_adjusted: revenue,
        }
    }
}

// TODO Make CategorisedValue a trait and implement it's three methods here
//      This also modifying requires add_data
impl<'mr> From<MusicRevenue<'mr>> for CategorisedValue<u16, &'mr str, isize> {
    fn from(music_revenue: MusicRevenue<'mr>) -> Self {
        Self::new(
            music_revenue.year,
            music_revenue.format,
            music_revenue.revenue_inflation_adjusted,
        )
    }
}

fn main() -> std::io::Result<()> {
    // let svg_file = File::create("revenue-by-music-format-css.svg")?;
    // source [RIAA](https://www.riaa.com/u-s-sales-database/)
    let mut file = File::open("./sources/music.csv")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let categorised_music_revenues = CategorisedValues::new()
        .with_secondary_categories(vec![
            "LP/EP",
            "Vinyl Single",
            "8 - Track",
            "Cassette",
            "Cassette Single",
            "Other Tapes",
            "Kiosk",
            "CD",
            "CD Single",
            "SACD",
            "DVD Audio",
            "Music Video (Physical)",
            "Download Album",
            "Download Single",
            "Ringtones and Ringbacks",
            "Download Music Video",
            "Other Digital",
            "Synchronization",
            "Paid Subscription",
            "On-Demand Streaming (Ad-Supported)",
            "Other Ad-Supported Streaming",
            "SoundExchange Distributions",
            "Limited Tier Paid Subscription",
        ])
        .add_data(
            contents
                .split('\n')
                .enumerate()
                .filter_map(|(i, row)| if i > 0 { Some(row) } else { None })
                .map(|row| MusicRevenue::from_iter(row.split(','))),
        );

    for primary in categorised_music_revenues.iter() {
        print!("{}: ({})\n\t", primary.key, primary.height());
        for secondary in primary.values().filter(|secondary| secondary.value > &0) {
            print!("{}: {}, ", secondary.key, secondary.value);
        }
        println!();
    }

    Ok(())
}
