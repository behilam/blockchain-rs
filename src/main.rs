mod models;

fn main() {
    let difficulty = 1;
    let mut count = 5;
    let mut blockchain = models::blockchain::Blockchain::new(difficulty);

    while count > 0 {
        models::blockchain::Blockchain::add_block(&mut blockchain, "".to_owned());
        count -= 1;
    }

    println!(
        "\n\
==========
= RESULT =
=========="
    );
    for block in blockchain.chain {
        println!("{:?}", block)
    }
}
