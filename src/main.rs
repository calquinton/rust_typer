// imports
use std::str;

fn main() -> Result<(), csv::Error> {

    // create arrays
    let mut quotes_list : Vec<&str> = Vec::new();
    let mut authors_list : Vec<&str> = Vec::new();

    // create csv reader and read file quotes.csv
    let mut rdr = csv::Reader::from_path(
        r".\src\quotes.csv",
    )?;

    // for every line in quotes.csv
    for result in rdr.records() {

        // convert line to StringRecord
        let record = result?;

        let mut column : u8 = 0;

        // for every field in the line
        for field in &record {

            // if in first column (0)
            if column == 0 {
                // test
                println!("first column");

                // add author to authors_list
                // authors_list.push(field);

                // move to second column
                column = 1;

            } else {

                // test
                println!("second column");

                // add quote to quotes_list
                // quotes_list.push(field);
            }

        }
        //test
        // println!("{:?}", record);
        
    }

    Ok(())

}
