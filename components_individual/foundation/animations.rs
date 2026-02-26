pub struct Animation {
    pub name: &'static str,
    pub duration: u32,
    pub easing: &'static str,
    pub iteration: &'static str,
    pub direction: &'static str,
    pub fill: &'static str,
    pub keyframes: &'static str,
}

impl Animation {
    pub fn new(name: &'static str, keyframes: &'static str) -> Self {
        Self {
            name,
            duration: 300,
            easing: "var(--sh-ease-bionic)",
            iteration: "1",
            direction: "normal",
            fill: "both",
            keyframes,
        }
    }

    pub fn duration(mut self, ms: u32) -> Self {
        self.duration = ms;
        self
    }

    pub fn infinite(mut self) -> Self {
        self.iteration = "infinite";
        self
    }

    pub fn ease_out(mut self) -> Self {
        self.easing = "var(--sh-ease-out)";
        self
    }

    pub fn ease_in(mut self) -> Self {
        self.easing = "var(--sh-ease-in)";
        self
    }

    pub fn ease_spring(mut self) -> Self {
        self.easing = "var(--sh-ease-spring)";
        self
    }

    pub fn linear(mut self) -> Self {
        self.easing = "linear";
        self
    }

    pub fn alternate(mut self) -> Self {
        self.direction = "alternate";
        self
    }

    pub fn to_css(&self) -> String {
        format!(
            "@keyframes {} {{ {} }}\n.{}-anim {{ animation: {} {}ms {} {} {}; }}",
            self.name,
            self.keyframes,
            self.name,
            self.name,
            self.duration,
            self.easing,
            self.iteration,
            self.direction,
            self.fill
        )
    }
}

pub fn all_animations() -> String {
    let mut css = String::new();

    // Fade animations
    css.push_str(
        &Animation::new("sh-fade-in", "from { opacity: 0; } to { opacity: 1; }")
            .duration(400)
            .to_css(),
    );
    css.push_str(
        &Animation::new("sh-fade-out", "from { opacity: 1; } to { opacity: 0; }")
            .duration(300)
            .to_css(),
    );

    // Slide animations
    css.push_str(&Animation::new("sh-slide-up", "from { opacity: 0; transform: translateY(20px); } to { opacity: 1; transform: translateY(0); }").duration(400).to_css());
    css.push_str(&Animation::new("sh-slide-down", "from { opacity: 0; transform: translateY(-20px); } to { opacity: 1; transform: translateY(0); }").duration(400).to_css());
    css.push_str(&Animation::new("sh-slide-left", "from { opacity: 0; transform: translateX(20px); } to { opacity: 1; transform: translateX(0); }").duration(400).to_css());
    css.push_str(&Animation::new("sh-slide-right", "from { opacity: 0; transform: translateX(-20px); } to { opacity: 1; transform: translateX(0); }").duration(400).to_css());

    // Scale animations
    css.push_str(
        &Animation::new(
            "sh-scale-in",
            "from { opacity: 0; transform: scale(0.9); } to { opacity: 1; transform: scale(1); }",
        )
        .duration(350)
        .to_css(),
    );
    css.push_str(
        &Animation::new(
            "sh-scale-out",
            "from { opacity: 0; transform: scale(1.1); } to { opacity: 1; transform: scale(1); }",
        )
        .duration(350)
        .to_css(),
    );
    css.push_str(&Animation::new("sh-bounce-in", "0% { opacity: 0; transform: scale(0.3); } 50% { transform: scale(1.05); } 70% { transform: scale(0.9); } 100% { opacity: 1; transform: scale(1); }").duration(600).ease_spring().to_css());

    // Special animations
    css.push_str(
        &Animation::new("sh-pulse", "0%, 100% { opacity: 1; } 50% { opacity: 0.5; }")
            .duration(2000)
            .infinite()
            .to_css(),
    );
    css.push_str(
        &Animation::new(
            "sh-spin",
            "from { transform: rotate(0deg); } to { transform: rotate(360deg); }",
        )
        .duration(1000)
        .linear()
        .infinite()
        .to_css(),
    );
    css.push_str(
        &Animation::new(
            "sh-float",
            "0%, 100% { transform: translateY(0); } 50% { transform: translateY(-10px); }",
        )
        .duration(3000)
        .ease_in()
        .alternate()
        .infinite()
        .to_css(),
    );
    css.push_str(&Animation::new("sh-glow", "0%, 100% { box-shadow: 0 0 5px var(--sh-primary), 0 0 10px var(--sh-primary); } 50% { box-shadow: 0 0 20px var(--sh-primary), 0 0 30px var(--sh-primary); }").duration(2000).infinite().to_css());

    // Grid drift for backgrounds
    css.push_str(
        &Animation::new(
            "sh-grid-drift",
            "from { background-position: 0 0; } to { background-position: 60px 60px; }",
        )
        .duration(20000)
        .linear()
        .infinite()
        .to_css(),
    );

    // Marquee
    css.push_str(
        &Animation::new(
            "sh-marquee",
            "from { transform: translateX(0); } to { transform: translateX(-50%); }",
        )
        .duration(15000)
        .linear()
        .infinite()
        .to_css(),
    );

    // Typewriter
    css.push_str(
        &Animation::new("sh-typewriter", "from { width: 0; } to { width: 100%; }")
            .duration(3000)
            .ease_out()
            .to_css(),
    );

    // Blinking cursor
    css.push_str(
        &Animation::new(
            "sh-caret-blink",
            "0%, 100% { opacity: 1; } 50% { opacity: 0; }",
        )
        .duration(1000)
        .infinite()
        .to_css(),
    );

    // Reveal
    css.push_str(&Animation::new("sh-reveal", "from { opacity: 0; filter: blur(10px); transform: scale(1.1); } to { opacity: 1; filter: blur(0); transform: scale(1); }").duration(800).ease_out().to_css());

    // Shimmer
    css.push_str(
        &Animation::new(
            "sh-shimmer",
            "0% { background-position: -200% 0; } 100% { background-position: 200% 0; }",
        )
        .duration(2000)
        .linear()
        .infinite()
        .to_css(),
    );

    // Shake
    css.push_str(&Animation::new("sh-shake", "0%, 100% { transform: translateX(0); } 25% { transform: translateX(-5px); } 75% { transform: translateX(5px); }").duration(500).to_css());

    // Zoom in up
    css.push_str(&Animation::new("sh-zoom-in-up", "from { opacity: 0; transform: translateY(20px) scale(0.9); } to { opacity: 1; transform: translateY(0) scale(1); }").duration(500).ease_spring().to_css());

    // Flip
    css.push_str(&Animation::new("sh-flip-in", "from { opacity: 0; transform: rotateY(90deg); } to { opacity: 1; transform: rotateY(0); }").duration(600).to_css());

    css
}

pub fn animation_classes() -> String {
    r#"
.sh-animate-fade-in { animation: sh-fade-in 400ms var(--sh-ease-bionic) both; }
.sh-animate-fade-out { animation: sh-fade-out 300ms var(--sh-ease-bionic) both; }
.sh-animate-slide-up { animation: sh-slide-up 400ms var(--sh-ease-bionic) both; }
.sh-animate-slide-down { animation: sh-slide-down 400ms var(--sh-ease-bionic) both; }
.sh-animate-slide-left { animation: sh-slide-left 400ms var(--sh-ease-bionic) both; }
.sh-animate-slide-right { animation: sh-slide-right 400ms var(--sh-ease-bionic) both; }
.sh-animate-scale-in { animation: sh-scale-in 350ms var(--sh-ease-bionic) both; }
.sh-animate-bounce-in { animation: sh-bounce-in 600ms var(--sh-ease-spring) both; }
.sh-animate-float { animation: sh-float 3000ms ease-in-out infinite alternate; }
.sh-animate-pulse { animation: sh-pulse 2000ms ease-in-out infinite; }
.sh-animate-spin { animation: sh-spin 1000ms linear infinite; }
.sh-animate-glow { animation: sh-glow 2000ms ease-in-out infinite; }
.sh-animate-shimmer { animation: sh-shimmer 2000ms linear infinite; }
.sh-animate-shake { animation: sh-shake 500ms var(--sh-ease-bionic) both; }
.sh-animate-zoom-in-up { animation: sh-zoom-in-up 500ms var(--sh-ease-spring) both; }
.sh-animate-reveal { animation: sh-reveal 800ms var(--sh-ease-out) both; }
.sh-animate-flip-in { animation: sh-flip-in 600ms var(--sh-ease-bionic) both; }
"#
    .to_string()
}
