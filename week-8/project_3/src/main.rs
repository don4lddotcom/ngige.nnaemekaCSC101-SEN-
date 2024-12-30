use std::io::Write;
fn main() {
    
    let commissioners = [
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbonna",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etiye", ];
    let ministries = [
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];
    let geopolitical_zones = [
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    let mut file = std::fs::File::create("merger.txt").unwrap();
    let columns = format!("{:<5} {:<30} {:<20} {:<15}\n", "S/N", "Name of Commissioner", "Ministry", "Geopolitical Zone");
    file.write_all(columns.as_bytes()).unwrap();

    for i in 0..commissioners.len() {
        let row = format!(
            "{:<5} {:<30} {:<20} {:<15}\n",
            i + 1,
            commissioners[i],
            ministries[i],
            geopolitical_zones[i]
        );
        file.write_all(row.as_bytes()).unwrap();
    }
}