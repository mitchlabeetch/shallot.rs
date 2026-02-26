//! VideoPlayer Component - Native Zero-JS Video Player UI
//!
//! A styled wrapper around the native HTML5 video element.
//! Provides custom controls styling using pure CSS.

use maud::{html, Markup, Render};

/// VideoPlayer size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VideoPlayerSize {
    Small,
    #[default]
    Medium,
    Large,
    Full,
}

impl VideoPlayerSize {
    fn css_class(&self) -> &'static str {
        match self {
            VideoPlayerSize::Small => "sh-videoplayer--sm",
            VideoPlayerSize::Medium => "sh-videoplayer--md",
            VideoPlayerSize::Large => "sh-videoplayer--lg",
            VideoPlayerSize::Full => "sh-videoplayer--full",
        }
    }
}

/// VideoPlayer component
pub struct VideoPlayer<'a> {
    src: &'a str,
    poster: Option<&'a str>,
    title: &'a str,
    size: VideoPlayerSize,
    autoplay: bool,
    loop_video: bool,
    muted: bool,
    controls: bool,
    class: Option<&'a str>,
}

impl<'a> VideoPlayer<'a> {
    /// Create a new VideoPlayer
    pub fn new(src: &'a str, title: &'a str) -> Self {
        Self {
            src,
            poster: None,
            title,
            size: VideoPlayerSize::default(),
            autoplay: false,
            loop_video: false,
            muted: false,
            controls: true,
            class: None,
        }
    }

    /// Set poster image
    pub fn poster(mut self, poster: &'a str) -> Self {
        self.poster = Some(poster);
        self
    }

    /// Set size
    pub fn size(mut self, size: VideoPlayerSize) -> Self {
        self.size = size;
        self
    }

    /// Enable autoplay
    pub fn autoplay(mut self, autoplay: bool) -> Self {
        self.autoplay = autoplay;
        self
    }

    /// Enable loop
    pub fn loop_video(mut self, loop_video: bool) -> Self {
        self.loop_video = loop_video;
        self
    }

    /// Set muted
    pub fn muted(mut self, muted: bool) -> Self {
        self.muted = muted;
        self
    }

    /// Show/hide native controls
    pub fn controls(mut self, controls: bool) -> Self {
        self.controls = controls;
        self
    }

    /// Add custom class
    pub fn class(mut self, class: &'a str) -> Self {
        self.class = Some(class);
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-videoplayer".to_string()];
        classes.push(self.size.css_class().to_string());
        if let Some(custom) = self.class {
            classes.push(custom.to_string());
        }
        classes.join(" ")
    }
}

impl<'a> Render for VideoPlayer<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();

        html! {
            figure class=(classes) {
                div class="sh-videoplayer__wrapper" {
                    video
                        class="sh-videoplayer__video"
                        src=(self.src)
                        poster=[self.poster]
                        autoplay?[self.autoplay]
                        loop?[self.loop_video]
                        muted?[self.muted]
                        controls?[self.controls]
                        playsinline
                        preload="metadata"
                        aria-label=(self.title)
                    {
                        "Your browser does not support the video tag."
                    }
                }
                @if let Some(_poster) = self.poster {
                    figcaption class="sh-videoplayer__caption" {
                        (self.title)
                    }
                } @else {
                    figcaption class="sh-videoplayer__caption" {
                        (self.title)
                    }
                }
            }
        }
    }
}

/// Generate CSS for VideoPlayer component
pub fn video_player_css() -> String {
    r#"
.sh-videoplayer {
    margin: 0;
    width: 100%;
}

.sh-videoplayer__wrapper {
    position: relative;
    background: #000;
    border-radius: var(--sh-radius-lg, 0.5rem);
    overflow: hidden;
    aspect-ratio: 16 / 9;
}

.sh-videoplayer__video {
    width: 100%;
    height: 100%;
    object-fit: contain;
}

/* Custom controls styling via CSS */
.sh-videoplayer__video::-webkit-media-controls {
    background: rgba(0, 0, 0, 0.7);
    border-radius: var(--sh-radius-md, 0.375rem);
}

.sh-videoplayer__video::-webkit-media-controls-panel {
    background: linear-gradient(transparent, rgba(0, 0, 0, 0.8));
}

.sh-videoplayer__video::-webkit-media-controls-play-button {
    background: var(--sh-primary, #3b82f6);
    border-radius: 50%;
}

.sh-videoplayer__video::-webkit-media-controls-current-time-display,
.sh-videoplayer__video::-webkit-media-controls-time-remaining-display {
    color: white;
    font-size: 0.75rem;
    font-weight: 500;
}

.sh-videoplayer__video::-webkit-media-controls-timeline {
    margin: 0 1rem;
}

.sh-videoplayer__video::-webkit-media-controls-timeline-container {
    margin: 0 1rem;
}

/* Size variants */
.sh-videoplayer--sm {
    max-width: 24rem;
}

.sh-videoplayer--md {
    max-width: 40rem;
}

.sh-videoplayer--lg {
    max-width: 64rem;
}

.sh-videoplayer--full {
    max-width: 100%;
}

.sh-videoplayer--full .sh-videoplayer__wrapper {
    border-radius: 0;
}

/* Caption */
.sh-videoplayer__caption {
    padding: 0.75rem 0.5rem;
    font-size: 0.875rem;
    color: var(--sh-text-secondary, #4b5563);
    text-align: center;
}

/* Focus state */
.sh-videoplayer__video:focus {
    outline: 2px solid var(--sh-primary, #3b82f6);
    outline-offset: 2px;
}

/* Theater mode hint */
.sh-videoplayer:has(.sh-videoplayer__video:fullscreen) {
    position: fixed;
    inset: 0;
    z-index: 9999;
    background: #000;
    display: flex;
    align-items: center;
    justify-content: center;
}

.sh-videoplayer:has(.sh-videoplayer__video:fullscreen) .sh-videoplayer__wrapper {
    max-width: 100%;
    max-height: 100%;
    border-radius: 0;
}

.sh-videoplayer:has(.sh-videoplayer__video:fullscreen) .sh-videoplayer__caption {
    display: none;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_videoplayer_creation() {
        let player = VideoPlayer::new("/video.mp4", "My Video");
        assert_eq!(player.src, "/video.mp4");
        assert_eq!(player.title, "My Video");
    }

    #[test]
    fn test_videoplayer_poster() {
        let player = VideoPlayer::new("/video.mp4", "Video").poster("/poster.jpg");
        assert_eq!(player.poster, Some("/poster.jpg"));
    }

    #[test]
    fn test_videoplayer_size() {
        let player = VideoPlayer::new("/video.mp4", "Video").size(VideoPlayerSize::Large);
        assert_eq!(player.size, VideoPlayerSize::Large);
    }

    #[test]
    fn test_videoplayer_autoplay() {
        let player = VideoPlayer::new("/video.mp4", "Video").autoplay(true);
        assert!(player.autoplay);
    }

    #[test]
    fn test_videoplayer_css() {
        let css = video_player_css();
        assert!(css.contains(".sh-videoplayer"));
        assert!(css.contains(".sh-videoplayer__video"));
    }
}
