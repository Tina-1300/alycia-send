use arboard::Clipboard;
use clap::Parser;
use enigo::{
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Settings,
};
use rand::seq::SliceRandom;
use std::{process, thread, time::Duration};

mod languages;

#[derive(Parser, Debug)]
#[command(
    name = "alycia-send",
    version,
    about = "A lightweight CLI tool for automated message sending.",
    long_about = "Alycia Send automates message sending through keyboard input and the system clipboard."
)]
struct Args {
    /// Number of messages to send
    #[arg(long = "nbm", default_value_t = 30, value_name = "NUMBER")]
    nb_messages: usize,

    /// Message dictionary language
    #[arg(long, default_value = "fr", value_name = "LANG")]
    lang: String,

    /// Target sex used to select the appropriate message dictionary
    #[arg(long = "ts", default_value = "neutral", value_name = "SEX")]
    target_sex: String,
    
    /// Message mode love or haite 
    #[arg(long, default_value = "love", value_name = "MODE")]
    mode: String,

    /// Minimum delay between messages in milliseconds
    #[arg(long, default_value_t = 1000, value_name = "MS")]
    delay_min: u64,

    /// Maximum delay between messages in milliseconds
    #[arg(long, default_value_t = 2000, value_name = "MS")]
    delay_max: u64,

    /// Delay before starting the program in seconds
    #[arg(long, default_value_t = 10, value_name = "SECONDS")]
    start_delay: u64,

    /// List all available languages and exit
    #[arg(long)]
    list_languages: bool,

    /// List all available target sexes and exit
    #[arg(long)]
    list_sexes: bool,

    /// List all available mode
    #[arg(long)]
    list_mode: bool,
}

fn main() {
    let args = Args::parse();

    if args.list_languages {
        println!("Available languages:");

        for language in languages::available_languages() {
            println!("  - {}", language);
        }

        return;
    }

    if args.list_sexes {
        println!("Available target sexes:");

        for sex in languages::available_target_sexes() {
            println!("  - {}", sex);
        }

        return;
    }

    if args.list_mode {
        println!("Available mode:");
        for mode in languages::available_modes() {
            println!("  - {}", mode);
        }
        return;
    }

    if args.nb_messages == 0 {
        eprintln!("Error: --nbm must be greater than 0.");
        process::exit(1);
    }

    if args.delay_min > args.delay_max {
        eprintln!("Error: --delay-min cannot be greater than --delay-max.");

        process::exit(1);
    }

    

    // validate target sex BEFORE retrieving the dictionary
    if !languages::available_target_sexes().contains(&args.target_sex.as_str()) {
        eprintln!(
            "Error: unknown target sex '{}'.",
            args.target_sex
        );

        eprintln!();
        eprintln!(
            "Available target sexes: {}",
            languages::available_target_sexes().join(", ")
        );

        process::exit(1);
    }

    // validate mode
    if !languages::available_modes().contains(&args.mode.as_str()) {
        eprintln!(
            "Error: unknown mode '{}'.",
            args.mode
        );

        eprintln!();

        eprintln!(
            "Available modes: {}",
            languages::available_modes().join(", ")
        );

        process::exit(1);
    }

    // retrieving the dictionary
    let messages = match languages::get_messages(&args.lang, &args.target_sex, &args.mode) {
        Some(messages) => messages,

        None => {
            eprintln!(
                "Error: no message dictionary found for language '{}' and target sex '{}' and mode '{}'.",
                args.lang,
                args.target_sex,
                args.mode
            );

            eprintln!();
            eprintln!(
                "Available languages: {}",
                languages::available_languages().join(", ")
            );

            eprintln!(
                "Available target sexes: {}",
                languages::available_target_sexes().join(", ")
            );

            eprintln!(
                "Available mode: {}",
                languages::available_modes().join(", ")
            );

            process::exit(1);
        }
    };

    if messages.is_empty() {
        eprintln!(
            "Error: the '{}' dictionary for target sex '{}' and mode {} is empty.",
            args.lang,
            args.target_sex, 
            args.mode
        );

        process::exit(1);
    }

    println!();
    println!("==============================================");
    println!("                 Alycia Send");
    println!("==============================================");
    println!("Language       : {}", args.lang);
    println!("Target sex     : {}", args.target_sex);
    println!("Mode           : {}", args.mode);
    println!("Messages       : {}", args.nb_messages);
    println!(
        "Delay          : {} - {} ms",
        args.delay_min, args.delay_max
    );
    println!("Start delay    : {} seconds", args.start_delay);
    println!("Dictionary     : {} messages", messages.len());
    println!("==============================================");
    println!();

    println!("Open your messaging application.");
    println!("Open the conversation where you want to send messages.");
    println!("Click on the message input field.");
    println!();

    println!("The program starts in {} seconds...", args.start_delay);

    // Countdown

    thread::sleep(Duration::from_secs(args.start_delay));

    println!();
    println!("Starting...");
    println!();

    // Random number generator
    let mut rng = rand::rng();

    // Creation of the initial mixture
    let mut messages_melanges: Vec<&str> = messages.to_vec();

    messages_melanges.shuffle(&mut rng);

    // Clipboard
    let mut clipboard = match Clipboard::new() {
        Ok(clipboard) => clipboard,

        Err(error) => {
            eprintln!("Error: unable to initialize clipboard: {}", error);
            process::exit(1);
        }
    };

    let mut enigo = match Enigo::new(&Settings::default()) {
        Ok(enigo) => enigo,

        Err(error) => {
            eprintln!(
                "Error: unable to initialize keyboard automation: {:?}",
                error
            );
            process::exit(1);
        }
    };

    // main loop
    for i in 0..args.nb_messages {
        // New cycle once all messages have been used
        if messages_melanges.is_empty() {
            messages_melanges = messages.to_vec();

            messages_melanges.shuffle(&mut rng);

            println!();
            println!("--- New message cycle ---");
            println!();
        }

        // Retrieving the message

        let message = messages_melanges.pop().unwrap();

        // Copy the message

        if let Err(error) = clipboard.set_text(message) {
            eprintln!("Error: unable to copy message: {}", error);

            process::exit(1);
        }

        // CTRL + V

        if let Err(error) = enigo.key(Key::Control, Press) {
            eprintln!("Error pressing CTRL: {:?}", error);

            process::exit(1);
        }

        if let Err(error) = enigo.key(Key::Unicode('v'), Click) {
            eprintln!("Error pressing V: {:?}", error);

            process::exit(1);
        }

        if let Err(error) = enigo.key(Key::Control, Release) {
            eprintln!("Error releasing CTRL: {:?}", error);

            process::exit(1);
        }

        thread::sleep(Duration::from_millis(200));

        if let Err(error) = enigo.key(Key::Return, Click) {
            eprintln!("Error pressing Enter: {:?}", error);

            process::exit(1);
        }

        println!("[{}/{}] {}", i + 1, args.nb_messages, message);

        let delay = rand::random_range(args.delay_min..=args.delay_max);

        thread::sleep(Duration::from_millis(delay));
    }

    println!();
    println!("==============================================");
    println!("Finished: {} messages sent.", args.nb_messages);
    println!("==============================================");
}
