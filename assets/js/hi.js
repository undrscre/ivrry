var lines = []
var ua = navigator.userAgent;

var browser = ua.includes("Firefox") ? "firefox"
    : ua.includes("Edg") ? "edge"
    : ua.includes("Chrome") ? "chrome"
    : ua.includes("Safari") ? "safari"
    : "unknown";

var os = ua.includes("Windows") ? "windows"
    : ua.includes("Mac") ? "mac"
    : ua.includes("Android") ? "android"
    : (ua.includes("iPhone") || ua.includes("iPad")) ? "ios"
    : ua.includes("Linux") ? "linux"
    : "unknown";

var cores = navigator.hardwareConcurrency;

async function createLines() {
    lines.push("let's find out together shall we?")
    if (browser != "unknown" || os != "unknown") { 
        lines.push(`you're currently reading this with <span class="evil-censored-bar">${browser}</span> on a <span class="evil-censored-bar">${os}</span> system.`)
        switch (os) {
            case "linux":
                switch (browser) {
                    case "firefox":
                        lines.push("hey! that's an awesome combo!<br>are you actually that privacy focused or did it just come with your system?")
                        break;
                    case "chrome":
                        lines.push("hey! that's an.. interesting combo..<br>are you running this through electron or something?")
                        break;
                    case "edge":
                        lines.push("what are you doing.")
                        break;
                    case "safari":
                        lines.push("what are you doing.")
                        break;
                }
                break;
            case "windows":
                switch (browser) {
                    case "firefox":
                        lines.push("hey! that's an awesome combo.<br>are you <i>really</i> that privacy focused?")
                        break;
                    case "chrome":
                        lines.push("hey! you're just like every windows user ever! that's interesting,")
                        break;
                    case "edge":
                        lines.push("you better not sell my data to microsoft!!")
                        break;
                    case "safari":
                        lines.push("what are you doing.")
                        break;
                }
                break;
            default:
                lines.push("i suppose that's pretty neat, i got nothing to say for that")
        }
    } else {
        lines.push("you're running on something i can't seem to recognize, props to you for being unique i suppose?")
    }

    if(!document.referrer) {
        lines.push("anywho, you seem pretty new here! welcome to my space, i hope you like it")
    } else {
        lines.push("anywho, you don't seem to be new around here, welcome back!")
    }

    lines.push("you're probably wondering;");
    lines.push("<i><span style='text-align:right; width: 100%; display: inline-block;'>\"noelle, how do you know any of this?\"</span></i>")
    lines.push("simple answer is: hahaha i just do!");
    try {
        let res = await fetch("https://ipapi.co/json/");
        let geo = await res.json();
        if (geo && geo.city) {
            lines.push(`it's okay though, you're probably safe in <span class="evil-censored-bar">${geo.city}</span>`);
        }
    } catch (e) {
        lines.push("it's okay, i don't really know that much about you.");
    }

    lines.push("<hr><br>this bit is inspired by <a href='https://yhvr.me'>yhvr</a>'s website, check them out! they're really cool.")
}

document.addEventListener('DOMContentLoaded', function () {
    createLines()
    let i = 0;
    let container = document.getElementById('sentences');
    let button = document.getElementById('who');
    button.addEventListener('click', function () {
        var p = document.createElement("p");
        p.classList.add("fadein");
        button.disabled = true;
        p.innerHTML = lines[i];
        container.appendChild(p);
        i++;

        setTimeout(function () {
            button.disabled = false;
        }, 1500)

        if (i >= lines.length) {
            button.remove()
        } else {
            button.textContent = "..."
        }
    });
});