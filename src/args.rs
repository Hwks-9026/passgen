use clap::Parser;

const DEFAULT_CHARS: &str = "ABDEFGHJLMNQRTYabdefghijmnqrty123456789";
const DEFAULT_SPECIAL_CHARS: &str = "!@#$%&*";

#[derive(Parser, Debug)]
#[command(
    name = "passgen",
    about = "Generate a random password consisting of blocks of characters.",
    after_help = "Example: passgen -b 3 -s 5 -p"
)]
pub struct Args {
    /// Number of blocks to generate
    #[arg(short = 'b', default_value_t = 5)]
    pub blocks: usize,

    /// The size (length) of each block
    #[arg(short = 's', default_value_t = 4)]
    pub block_size: usize,

    /// The delimiter used between blocks
    #[arg(short = 'd', default_value = "-")]
    pub delimiter: String,

    /// The set of characters used for generation
    #[arg(short = 'c', default_value = DEFAULT_CHARS)]
    pub characters: String,

    /// Appends special characters to the set. 
    /// If the flag is used without a value, it uses the default special characters.
    #[arg(
        short = 'p', 
        num_args = 0..=1, 
        default_missing_value = DEFAULT_SPECIAL_CHARS
    )]
    pub special_chars: Option<String>,

    /// If set, provide extra information about the security of chosen password settings.
    #[arg(short = 'i')]
    pub extra_info: bool,

    /// If set, colorize the output.
    #[arg(short = 'o')]
    pub colorize: bool,
}
