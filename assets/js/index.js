function connectToHeader(container, cardElem, headerElem) {
    if (!container || !cardElem || !headerElem) return;

    const cRect = container.getBoundingClientRect();
    const hRect = headerElem.getBoundingClientRect();
    const rRect = cardElem.getBoundingClientRect();

    // chaos origins: pick any point along the header's edge
    const originX = (hRect.left + getRandomInt(hRect.width)) - cRect.left + getRandomInt(5);
    const originY = (hRect.top + getRandomInt(hRect.height * 3)) - cRect.top;
    
    const targetX = (rRect.left + getRandomInt(rRect.width)) - cRect.left;
    const targetY = (rRect.top + getRandomInt(rRect.height)) - cRect.top;

    // vertical segment with a bit of horizontal jitter
    const vLine = document.createElement('div');
    vLine.style.cssText = `
        position: absolute; border-left: 1px solid black; 
        opacity: 0.4;
        left: ${originX}px; 
        top: ${Math.min(originY, targetY)}px; 
        width: 1px; 
        height: ${Math.abs(targetY - originY)}px; 
        pointer-events: none;
    `;

    // horizontal segment
    const hLine = document.createElement('div');
    hLine.style.cssText = `
        position: absolute; border-top: 1px solid black; 
        opacity: 0.4;
        left: ${Math.min(originX, targetX)}px; 
        top: ${targetY}px; 
        width: ${Math.abs(targetX - originX)}px; 
        height: 1px; 
        pointer-events: none;
    `;

    container.append(vLine, hLine);
}

function getRandomInt(max) {
    return Math.floor(Math.random() * max);
}

async function fetchRepos() {
    try {
        const response = await fetch(`fuck github`);
        const repos = await response.json();
        return repos.filter(repo => !repo.fork).slice(0, 10);
    } catch (error) {
        console.error(error);
    }
}

fetchRepos().then(latestRepos => {
    if (!latestRepos) {
        latestRepos = []
        for (let _ = 0; _ < 6; _++) {
            latestRepos.push({
                "name": "OOPS!!! networkerror!",
                "html_url":"/",
                "description":"uh oh",
                "stargazers_count":"9999",
                "language":"?!?!",
            })
        }
    };

    document.getElementById("repo-container").innerHTML = latestRepos.map((repo, i) => `
        <div class="repo-card" id="repo-${i}" style="position: absolute; top: ${(50 + getRandomInt(10)) * i}px; left: ${30 + getRandomInt(400)}px; z-index: 10;">
            <div class="evil-class-name"></div>
            <a href="${repo.html_url}"><h2>${repo.name}</h2></a>
            <p>${repo.description || 'no description provided'}</p>
            <span>${repo.stargazers_count} stars • ${repo.language || 'code'}</span>
        </div>
    `).join('');

    requestAnimationFrame(() => {
        const header = document.getElementById("repo-header");
        const connectors = document.getElementById("connectors");
        latestRepos.forEach((_, i) => {
            const card = document.getElementById(`repo-${i}`);
            connectToHeader(connectors, card, header);
        });
    });
});

const header2 = document.getElementById("friends-header");

document.querySelectorAll(".button-item").forEach(el => {
    el.style.position = "absolute";
    el.style.top = `${70 + getRandomInt(300)}px`;
    el.style.left = `${getRandomInt(600)}px`;
    
    connectToHeader(document.getElementById("connectors2"), el, header2);
});

async function updateLastFM() {
	const template = document.getElementById('lastfm-template');
	const clone = template.content.cloneNode(true);
	
	const legend = document.querySelector('#lastfm-status');
	const artImg = clone.querySelector('.listening-art');
	const title = clone.querySelector('.listening-title');
	const album = clone.querySelector('.album-name');
	const artist = clone.querySelector('.artist-name');
	const link = clone.querySelector('.lastfm-link');

	try {
		let request = await fetch('https://lastfm.nkko.workers.dev/?method=user.getRecentTracks&user=undrscr_').then(r => r.json());
		let response = request.recenttracks;
		let data = response.track[0]; // fuckup with my implementation
		
		legend.textContent = data["@attr"] ? "currently listening to:" : "last listened to:";
		
		const albumArt = data.image?.[2]?.["#text"] || 
			"https://lastfm.freetls.fastly.net/i/u/174s/2a96cbd8b46e442fc41c2b86b821562f.png";
		
		artImg.src = albumArt;
		// bgImg.src = albumArt;
		artImg.alt = `album art for ${data.album?.["#text"] || data.name}`
		title.textContent = data.name;
		title.setAttribute("data-title", data.name);
		album.textContent = data.album?.["#text"] || data.name;
		artist.textContent = data.artist?.["#text"];
        clone.querySelector("#umm-haha").href = data.url;
		
		if (data.url) {
			link.innerHTML = `<span class="splash">${response["@attr"].total}</span> total scrobbles`;
		}
		
		document.querySelector('.lastfm').appendChild(clone);
		
	} catch (err) {

		console.log(err)
		legend.textContent = "oops!";
		
		const albumArt = "https://lastfm.freetls.fastly.net/i/u/174s/2a96cbd8b46e442fc41c2b86b821562f.png";
		
		artImg.src = albumArt;
		artImg.alt = `placeholder art`
		title.textContent = "error loading last.fm stats!";
		album.textContent = "???";
		artist.textContent = "???";
		link.textContent = `error: ${err.message}`;

		document.querySelector('.lastfm').appendChild(clone);
	}
}

document.addEventListener('DOMContentLoaded', () => {
	updateLastFM();

    const tags = ['p','h1','h2','h3','h4','h5','h6','span','a','li','td','th','label','button','blockquote','figcaption','legend','dt','dd'];
    const all = [...document.querySelectorAll(tags.join(','))];
    for (let index = 0; index < (all.length / 5); index++) {
        const el = all[Math.floor(Math.random() * all.length)];
        el.style.setProperty('font-family', 'serif', 'important');
    }    
});

