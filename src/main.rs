use rand::Rng;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(short, long, default_value = "4")]
    num_words: i32,

    #[structopt(short="s", long, default_value = " ")]
    separator: String,
}

fn main() {
    let wordlist1 = include_str!("eff_short_wordlist_1.txt");
    let wordlist2 = include_str!("eff_short_wordlist_2_0.txt");

    let mut words = vec![];
    for w in get_words(wordlist1) {
        words.push(w);
    }
    for w in get_words(wordlist2) {
        words.push(w);
    }

    let mut rng = rand::thread_rng();
    let opt = Opt::from_args();

    let mut pwords: Vec<String> = vec![];
    for _ in 0..opt.num_words {
        pwords.push(words[rng.gen_range(0..words.len())].to_string())
    }

    let output = pwords.join(&opt.separator);
    println!("{}", output);
}

fn get_words(input: &'static str) -> Vec<String> {
    let mut output = vec![];
    for line in input.split("\n") {
        let words = line.split("\t").collect::<Vec<&str>>();
        if words.len() == 2 {
            output.push(words[1].into())
        }
    }
    output
}
