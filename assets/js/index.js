const FEED_URL = "https://notes.nkko.workers.dev/";
const LASTFM_URL = "https://lastfm.nkko.workers.dev/?method=user.getRecentTracks&user=undrscr_";
const NEKOWEB_USER = "ivrry";

const POST_COUNT = 2;
const MAX_CHARS = 240; 
const FALLBACK_ART = "https://lastfm.freetls.fastly.net/i/u/174s/2a96cbd8b46e442fc41c2b86b821562f.png";

const $ = (sel, root = document) => root.querySelector(sel);

async function updateLastFM() {
    const art = $("#album-art");
    const title = $("#lastfm-title");
    const artist = $("#lastfm-artist");
    const album = $("#lastfm-album");
    
    try {
        const res = await fetch(LASTFM_URL);
        if (!res.ok) throw new Error(`last.fm ${res.status}`);
        const track = (await res.json()).recenttracks.track[0];
        const albumName = track.album?.["#text"] || track.name;
        
        art.src = track.image?.[2]?.["#text"] || FALLBACK_ART;
        art.alt = `album art for ${albumName}`;
        title.textContent = track.name;
        title.title = track.name;
        artist.textContent = `by ${track.artist?.["#text"] ?? "???"}`;
        album.textContent = `on ${albumName}`;
        
        $(".lastfm")?.classList.toggle("playing", track["@attr"]?.nowplaying === "true");
    } catch (err) {
        console.error(err);
        art.src = FALLBACK_ART;
        art.alt = "placeholder art";
        title.textContent = "error loading last.fm stats!";
        artist.textContent = "by ???";
        album.textContent = err.message;
    }
}

function getOrdinalSuffix(day) {
    if (day > 3 && day < 21) return "th";
    switch (day % 10) {
        case 1: return "st";
        case 2: return "nd";
        case 3: return "rd";
        default: return "th";
    }
}

function formatDate(d) {
    const month = d.toLocaleDateString("en-US", { month: "long" }); 
    const day = d.getDate();
    return `${month} ${day}${getOrdinalSuffix(day)}`; // "Nov 11th"
}

function renderPaddedStat(value, width = 13) {
    const raw = String(value ?? 0);
    const str = raw.padStart(width, "0");
    const leading = str.length - raw.length;
    return [...str]
    .map((digit, i) => `<span${i < leading ? ' class="subtext"' : ""}>${digit}</span>`)
    .join("");
}

async function updateSiteStats() {
    try {
        const res = await fetch(`https://nekoweb.org/api/site/info/${NEKOWEB_USER}.nekoweb.org`);
        const info = await res.json();
        
        $(".statistics-visitors h1").innerHTML = renderPaddedStat(info.views);
        $(".statistics-followers h1").innerHTML = renderPaddedStat(info.followers);
        $(".statistics-update h1").textContent = formatDate(new Date(info.updated_at));
    } catch (err) {
        console.error(err);
    }
}

async function getFirstEntry(url = FEED_URL) {
    const res = await fetch(url);
    if (!res.ok) throw new Error(`fetch failed: ${res.status}`);
    let json = await res.json();
    let content = json[0];

    $("#post-content").textContent = content["text"];
    $("#post-name").textContent = content["user"]["name"];
    $(".post-avatar").src = content["user"]["avatarUrl"];
    $("#post-link").textContent = `view on instance ↗`;
    $("#post-link").style.opacity = 1;
    $("#post-link").href = `https://booping.synth.download/notes/${content["threadId"]}`;
    if (content["reply"]) {
        $("#post-reply").style.display = "block";
        $("#post-reply-name").textContent = content["reply"]["user"]["name"];
    }
    if (content["files"]) {
        content["files"].forEach(file => {
            let span = document.createElement("span");
            let br = document.createElement("br");
            span.textContent = `+ attached: ${file["name"]}`
            $("#post-files").appendChild(span);
            $("#post-files").appendChild(br);
        });
    }
}


document.addEventListener("DOMContentLoaded", () => {
    updateLastFM();
    updateSiteStats();
    getFirstEntry();
});