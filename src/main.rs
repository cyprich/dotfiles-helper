use dialoguer::{Confirm, MultiSelect, theme::ColorfulTheme};

fn main() {
    loop {
        let selected = handle_packages();

        println!("\nAll selected packages: ");
        selected.iter().for_each(|i| print!("{} ", i));
        println!();

        let confirm = handle_confirm();
        println!();

        if confirm {
            break;
        }
    }
}

fn handle_packages() -> Vec<&'static str> {
    let required = vec![
        "cargo",
        "eza",
        "fd",
        "fzf",
        "gcc",
        "git",
        "gzip",
        "lua",
        "luarocks",
        "neovim",
        "python3",
        "ripgrep",
        "rustc",
        "unzip",
        "wget",
        "wl-clipboard",
        "xclip",
    ];
    let cli = vec![
        "bat",
        "duf",
        "dust",
        "fastfetch",
        "lazygit",
        "lazydocker",
        "openvpn",
        "tailscale",
        "tealdeer",
        "traceroute",
        "tree",
        "uv",
        "yazi",
    ];
    let gui = vec![
        "chromium",
        "datagrip",
        "davinci-resolve",
        "discord",
        "drawio",
        "eog",
        "ghostty",
        "inkscape",
        "jellyfin-media-player",
        "krita",
        "libreoffice",
        "lollypop",
        "lunar-client",
        "obsidian",
        "onlyoffice",
        "orca-slicer",
        "parsec-bin",
        "rustrover",
        "qbittorrent",
        "spotify",
        "virtualbox",
        "vlc",
        "vscode",
    ];
    let useless = vec![
        "aalib",
        "asciiquarium",
        "astroterm",
        "cbonsai",
        "cmatrix",
        "cowsay",
        "figlet",
        "hollywood",
        "lolcat",
        "pfetch",
        "pipes.sh",
        "pokete",
        "presenterm",
        "sl",
    ];

    let mut selected: Vec<&str> = Vec::new();

    select_packages(required, "Required Packages".to_string(), true)
        .iter()
        .for_each(|i| selected.push(i));

    select_packages(cli, "CLI Packages".to_string(), false)
        .iter()
        .for_each(|i| selected.push(i));

    select_packages(gui, "GUI Packages".to_string(), false)
        .iter()
        .for_each(|i| selected.push(i));

    select_packages(useless, "Useless Packages".to_string(), false)
        .iter()
        .for_each(|i| selected.push(i));

    selected
}

fn handle_confirm() -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Is this correct?")
        .interact()
        .unwrap()
}

fn select_packages(packages: Vec<&str>, title: String, selected: bool) -> Vec<&str> {
    let defaults = vec![selected; packages.len()];

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("{} (SPACE to toggle, ENTER to submit)", title))
        .items(&packages)
        .defaults(&defaults)
        .interact()
        .unwrap();

    selections.iter().map(|i| packages[*i]).collect()
}
