fn main() {
    #[cfg(not(feature = "debug"))]
    bevy_app::run();

    #[cfg(feature = "debug")]
    console_app::run();
}
