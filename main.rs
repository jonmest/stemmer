use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;


#[derive(Clone)]
#[derive(Debug)]
struct StemInfo {
    name: String,
    description: String
}

#[derive(Clone)]
struct TrieNode {
    branches: HashMap<char, TrieNode>,
    leaf: Option<StemInfo>
}

fn read_csv<P: AsRef<Path>>(filename: P) -> Result<(), Box<dyn Error>> {
    let mut root_node: TrieNode = TrieNode {
        branches: HashMap::new(),
        leaf: None
    };
    let file = File::open(filename)?;
    let mut rdr = csv::ReaderBuilder::new().delimiter(b'\t').from_reader(file);

    for result in rdr.records() {
        let row = result.unwrap();
        let si = StemInfo {
            name: row[0].to_string(),
            description: row[1].to_string()
        };
        let mut current = root_node.clone();
        for c in si.name.chars() {
            if (current.branches.contains_key(&c)) {
                current = current.branches.get(&c).unwrap().clone();
            } else {
                current.branches.insert(c, TrieNode { branches: HashMap::new(), leaf: None });

            }
        }
        println!("{:?}", si);

        // for c in stem.chars() {
        //     if (currentNode.branches.contains_key(&c)) {
        //         let next = currentNode.branches.get(&c);
        //         if (next.is_none()) {
        //             currentNode.branches.insert(c, TrieNode { branches: HashMap::new(), leaf: None });
        //         }
                
        //     }
        // }      

    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = "/workspaces/stemmer/drug_name_stems.tsv";
    read_csv(filename)
}