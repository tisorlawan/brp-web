pub mod button;
pub mod datepicker;
pub mod input;
pub mod modal;
pub mod theme_toggle;
pub mod toast;

#[derive(Debug, Default, Clone, Copy)]
pub enum Color {
    #[default]
    Default,
    Alternative,
    Blue,
    Green,
    Red,
    Yellow,
    Purple,
    PurpleToBlue,
    CyanToBlue,
    GreenToBlue,
    PurpleToPink,
    PinkToOrange,
    TealToLime,
    RedToYellow,
}

impl Color {
    fn cn_btn(&self) -> &'static str {
        match self {
            Color::Default => "text-background bg-gray-800 hover:bg-gray-700 focus:ring-gray-300 dark:text-gray-900 dark:bg-white dark:border-gray-300 dark:hover:bg-gray-200 dark:focus:ring-gray-100",
            Color::Alternative => "bg-background/10 focus:ring-border-focus/20 focus:ring-1 border border-border-focus/80 hover:bg-foreground/5",
            Color::Blue => "text-background bg-blue-700 hover:bg-blue-800 focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800",
            Color::Green => "text-background bg-green-700 hover:bg-green-800 focus:ring-green-300 dark:bg-green-600 dark:hover:bg-green-700 dark:focus:ring-green-800",
            Color::Red => "text-background bg-red-700 hover:bg-red-800 ocus:ring-red-300 dark:bg-red-600 dark:hover:bg-red-700 dark:focus:ring-red-900",
            Color::Yellow => "text-background bg-yellow-400 hover:bg-yellow-500 focus:ring-yellow-300 dark:focus:ring-yellow-900",
            Color::Purple => "text-background bg-purple-700 hover:bg-purple-800 focus:ring-purple-300 dark:bg-purple-600 dark:hover:bg-purple-700 dark:focus:ring-purple-900",
            Color::PurpleToBlue => "text-background bg-gradient-to-br from-purple-600 to-blue-500 hover:bg-gradient-to-bl focus:ring-4 focus:outline-none focus:ring-blue-300 dark:focus:ring-blue-800 font-medium text-sm px-5 py-2.5 text-center",
            Color::CyanToBlue => "text-background bg-gradient-to-r from-cyan-500 to-blue-500 hover:bg-gradient-to-bl focus:ring-4 focus:outline-none focus:ring-cyan-300 dark:focus:ring-cyan-800 font-medium text-sm px-5 py-2.5 text-center",
            Color::GreenToBlue => "text-background bg-gradient-to-br from-green-400 to-blue-600 hover:bg-gradient-to-bl focus:ring-4 focus:outline-none focus:ring-green-200 dark:focus:ring-green-800 font-medium text-sm px-5 py-2.5 text-center",
            Color::PurpleToPink => "text-background bg-gradient-to-r from-purple-500 to-pink-500 hover:bg-gradient-to-l focus:ring-4 focus:outline-none focus:ring-purple-200 dark:focus:ring-purple-800 font-medium text-sm px-5 py-2.5 text-center",
            Color::PinkToOrange => "text-background bg-gradient-to-br from-pink-500 to-orange-400 hover:bg-gradient-to-bl focus:ring-4 focus:outline-none focus:ring-pink-200 dark:focus:ring-pink-800 font-medium text-sm px-5 py-2.5 text-center",
            Color::TealToLime => "text-gray-900 bg-gradient-to-r from-teal-200 to-lime-200 hover:bg-gradient-to-l hover:from-teal-200 hover:to-lime-200 focus:ring-4 focus:outline-none focus:ring-lime-200 dark:focus:ring-teal-700 font-medium text-sm px-5 py-2.5 text-center",
            Color::RedToYellow => "text-gray-900 bg-gradient-to-r from-red-200 via-red-300 to-yellow-200 hover:bg-gradient-to-bl focus:ring-4 focus:outline-none focus:ring-red-100 dark:focus:ring-red-400 font-medium text-sm px-5 py-2.5 text-center",
        }
    }
}
