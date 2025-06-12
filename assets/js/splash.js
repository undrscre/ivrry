let totalSplashes = {
    "categories": [
        "nthclub",
        "other"
    ],
    "nthclub": [
        "Splashes splashes... where's the water?",
        "the lion, the witch, and the audacity of this bitch!",
        // "actually, why DID joe have so many liberals???",
        "Help ive been trapped in these texts for 7 years let me out LET ME OU",
        "\"it's dot org!\"",
        "don't talk to me until u a farm merge valley sweat 🔥🔥🔥🔥",
        "whuh- how? why?                                             ..nevermind",
        "dont ever buy no gas from the gas station bro",
        "pfft, who told you that? splash texts are myths, bud",
        "Also try Terraria!",
        "Also try Minecraft!",
        "..chat, is this real?!",
        "and by it.. let's just say my website",
        "i am the angry pumpkin.",
        "39 buried, 0 found.",
        "THEY FUCKING BROKE MY MARQUEE ELEMENT."
    ],
    "other": [
        "hiii squidwaaardd :3",
        "gurple >>>>> graple",
        "gay",
        "it's sludge saturday baby!",
        "trans rights are human rights!",
        "gay rights are human rights!",
        "swocket",
        "Dude please help i'm stuck outside of my house"
    ]
};

document.addEventListener('DOMContentLoaded', () => {
    // the beast
	let splash = totalSplashes[totalSplashes.categories[Math.floor(Math.random() * totalSplashes.categories.length)]][Math.floor(Math.random() * totalSplashes[totalSplashes.categories[Math.floor(Math.random() * totalSplashes.categories.length)]].length)];
    console.log(splash);
	document.getElementById("splash-text").innerHTML = splash;
});