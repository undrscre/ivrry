let totalSplashes = {
    "categories": [
        "me", // ME!! ME!! I WROTE THIS!!
        "shoutout",
        "nthclub",
        "other"
    ],
    "me": [
        "now written in rust!",
        "Deltarune tomorrow",
        "i stayed up until 4am to write this",
        "check out my website https://127.0.0.1:3030/index.html",
        "beep boop",
        "mrrow",
        "meow",
        "paws at you",
        "it's goop monday",
        "ears caressing gif",
        "Wasy!",
        ">w<",
    ],
    "shoutout": [
        // in no particular order;
        "shoutout natalie",
        "shoutout sundaedog",
        "shoutout mayflower",
        "shoutout oatmealine",
        "shoutout icosahedra",
        "shoutout ryoojiz",
        "shoutout celestialexe",
        "shoutout slime",
        "shoutout finn",
        "shoutout foggy",
        "shoutout air2",
        "shoutout kraafter",
        "shoutout xiliam",
        "shoutout wardo",
        "shoutout zabregah",
        "shoutout shadestaa",
        "shoutout ivyclub",
        "shoutout sky",
        "shoutout yuri",
        "shoutout b.6",
        "shoutout hama",
        "shoutout val",
        "shoutout fuac",
        "shoutout to those reading this right now",
        // and shoutout to you!
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
    let category = totalSplashes.categories[Math.floor(Math.random() * totalSplashes.categories.length)];
    console.log(category);
	let splash = totalSplashes[category][Math.floor(Math.random() * totalSplashes[category].length)];
    console.log(splash);
	document.getElementById("splash-text").innerHTML = splash;
});