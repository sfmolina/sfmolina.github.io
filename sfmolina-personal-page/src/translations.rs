//-------------------------------------------------------------------//
//  AUTHOR:    @sfmolina                                            //
//  Version:   v1                                                  //
//  Modified:  23oc24                                             //
//---------------------------------------------------------------//


#[derive(Clone, PartialEq)]
pub enum LanguageVariant {
    English,
    Spanish,
}


#[derive(Debug)]
pub struct Texts {
    pub lang: &'static str,
    pub author_username: &'static str,
    pub author_large: &'static str,

    pub nav: NavTexts,

    pub about_me: AboutMeTexts,

    pub super_tres_check: &'static str,
}

#[derive(Debug)]
pub struct AboutMeTexts {
    pub greeting: &'static str,
    pub my_name: &'static str,
    pub my_profession: &'static str,

    pub introduction1: &'static str,
    pub introduction2: &'static str,

    pub prof_pict_alt: &'static str,

    pub section_1_title: &'static str,

    pub fav_rust_text1: &'static str,
    pub fav_rust_text2: &'static str,
    pub fav_rust_alt: &'static str,

    pub fav_haskell_text1: &'static str,
    pub fav_haskell_text2: &'static str,
    pub fav_haskell_alt: &'static str,

    pub fav_linux_text1: &'static str,
    pub fav_linux_text2: &'static str,
    pub fav_linux_alt: &'static str,
}

#[derive(Debug)]
pub struct NavTexts {
    pub about_me: &'static str,
    pub projects: &'static str,
    pub super_tres: &'static str,

    pub language: &'static str,
    pub misc: &'static str,
    pub theme: &'static str,

    pub es: &'static str,
    pub en: &'static str,

    pub neumorphism: &'static str,
    pub brutalism: &'static str,
}


pub const SPANISH_TEXTS: Texts = Texts {
    lang: "es",
    author_username: "@sfmolina",
    author_large: "Serafín L. Molina",

    nav: NavTexts {
        about_me: "Sobre mí",
        projects: "Proyectos",
        super_tres: "Super Tres",
        language: "Idioma",
        misc: "Misc",
        theme: "Tema",
        es: "Español",
        en: "Inglés",
        neumorphism: "Neumorfismo",
        brutalism: "Brutalismo",
    },

    about_me: AboutMeTexts {
        greeting: "¡Hola!",
        my_name: "Me llamo ",
        my_profession: "y estudio ingeniería informática.",
        introduction1: "Siempre me he interesado por la ciencia y he sido una persona muy curiosa. Pero las matemáticas despiertan en mí una pasión especial. Por eso pensé que la informática me gustaría, y no me equivoqué.",
        introduction2: "A la hora de trabajar y estudiar, soy muy meticuloso y me gusta hacer las cosas bien. A veces soy demasiado purista a la hora de programar, y doy muchas vueltas a las cosas para que sean eficientes, modulares, fáciles de mantener... Me gusta seguir las buenas prácticas y siempre estoy aprendiendo cosas nuevas para mejorar.",
        prof_pict_alt: "Imagen de perfil de Serafín L. Molina",
        section_1_title: "Mis favoritos",
        
        fav_rust_text1: "Me gusta su sintaxis, su rendimiento y la seguridad que ofrece escribiendo código robusto. Estoy acostumbrado a trabajar con lenguajes estrictos y me encanta eliminar errores en tiempo de compilación.",
        fav_rust_text2: "He trabajado con Yew, Tauri, Rocket, Tokio y SurrealDB entre otros. Lo uso en la mayoría de mis proyectos personales. Y para el backend de uno de ellos estoy utilizando Rustler, para aprovechar Rust en Elixir.",
        fav_rust_alt: "Logo de Rust: https://www.rust-lang.org/static/images/rust-logo-blk.svg",
        
        fav_haskell_text1: "Es mi lenguaje favorito. Me encanta su sintaxis, su tipado fuerte, su sistema de tipos, la currificación, la evaluación perezosa, su pureza... Es un lenguaje muy elegante y me gusta mucho programar en él.",
        fav_haskell_text2: "He trabajado con Stack, QuickCheck y un poco con Yesod. Uso mucho este lenguaje para hacer scripts y estudiar las matemáticas que doy en la universidad. También desarrollo en mi tiempo libre mi propia librería de estructuras de datos.",
        fav_haskell_alt: "De Darrin A Thompson; J.M. Wheeler - Thompson-Wheeler logo on the haskell wiki, Dominio público, https://commons.wikimedia.org/w/index.php?curid=8479507",
        
        fav_linux_text1: "Para trabajar, es mi sistema operativo favorito. Me gusta su filosofía, su rendimiento y la facilidad que brinda para desarrollar software. Además, la terminal es una maravilla.",
        fav_linux_text2: "He probado Ubuntu, Fedora, Manjaro, Arch y EndeavourOS. Actualmente uso el último, y estoy muy contento. Nunca he tenido problemas, e instalar software es muy sencillo.\n\nbtw, i use Arch.",
        fav_linux_alt: "De Larry Ewing, Simon Budig, Garrett LeSage - https://isc.tamu.edu/~lewing/linux/, https://www.home.unix-ag.org/simon/penguin/, garrett/Tux on GitHub, CC0, https://commons.wikimedia.org/w/index.php?curid=753970",
    },

    super_tres_check: "Comprobar",
};


pub const ENGLISH_TEXTS: Texts = Texts {
    lang: "en",
    author_username: "@sfmolina",
    author_large: "Serafín L. Molina",

    nav: NavTexts {
        about_me: "About me",
        projects: "Projects",
        super_tres: "Super Tres",
        language: "Language",
        misc: "Misc",
        theme: "Theme",
        es: "Spanish",
        en: "English",
        neumorphism: "Neumorphism",
        brutalism: "Brutalism",
    },

    about_me: AboutMeTexts {
        greeting: "Hello!",
        my_name: "My name is ",
        my_profession: "and I study computer engineering.",
        introduction1: "I have always been interested in science and have been a very curious person. But mathematics arouse a special passion in me. That is why I thought I would like computer science, and I was not mistaken.",
        introduction2: "When it comes to working and studying, I am very meticulous and I like to do things right. Sometimes I am too much of a purist when it comes to programming, and I go around things a lot to make them efficient, modular, easy to maintain... I like to follow best practices and I am always learning new things to improve.",
        prof_pict_alt: "Profile picture of Serafín L. Molina",
        section_1_title: "My favorites",
        
        fav_rust_text1: "I like its syntax, its performance and the security it offers by writing robust code. I am used to working with strict languages and I love eliminating errors at compile time.",
        fav_rust_text2: "I have worked with Yew, Tauri, Rocket, Tokio and SurrealDB among others. I use it in most of my personal projects. Moreover, for the backend of one of them I am using Rustler, to take advantage of Rust in Elixir.",
        fav_rust_alt: "Rust logo: https://www.rust-lang.org/static/images/rust-logo-blk.svg",
        
        fav_haskell_text1: "It is my favorite language. I love its syntax, its strong typing, its type system, currying, lazy evaluation, its purity... It is a very elegant language and I really enjoy programming in it.",
        fav_haskell_text2: "I have worked with Stack, QuickCheck and a little with Yesod. I use this language a lot to write scripts and study the mathematics I study at university. I also develop in my free time my own data structures library.",
        fav_haskell_alt: "By Darrin A Thompson; J.M. Wheeler - Thompson-Wheeler logo on the haskell wiki, Public Domain, https://commons.wikimedia.org/w/index.php?curid=8479507",
        
        fav_linux_text1: "For work, it is my favorite operating system. I like its philosophy, its performance and the ease it provides for developing software. Also, the terminal is wonderful.",
        fav_linux_text2: "I have tried Ubuntu, Fedora, Manjaro, Arch and EndeavourOS. I currently use the last one, and I am very happy. I have never had any problems, and installing software is very simple.\n\nbtw, i use Arch.",
        fav_linux_alt: "By Larry Ewing, Simon Budig, Garrett LeSage - https://isc.tamu.edu/~lewing/linux/, https://www.home.unix-ag.org/simon/penguin/, garrett/Tux on GitHub, CC0, https://commons.wikimedia.org/w/index.php?curid=753970",
    },

    super_tres_check: "Check",
};


impl PartialEq for Texts {
    fn eq(&self, other: &Self) -> bool {
        self.lang == other.lang
    }
}

impl Eq for Texts {}
