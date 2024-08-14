use std::io::Write;

macro_rules! parse {
    ($lines: ident, $x: ty) => {
        $lines.next().unwrap().parse::<$x>()?
    };
    ($lines: ident, $($x: ty), +) => {{
        let line = $lines.next().unwrap();
        let mut line = line.split_whitespace();
        ($(line.next().unwrap().parse::<$x>()?,)+)
    }};
    ($lines: ident => $x: ty) => {
        $lines.next().unwrap().split_whitespace().map(str::parse).map(Result::unwrap).collect::<$x>()
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = std::io::stdout().lock();
    let mut lines = std::io::stdin().lines().map(Result::unwrap);

    Ok(())
}
