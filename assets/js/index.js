async function updateLastFM() {
	const template = document.getElementById('lastfm-template');
	const clone = template.content.cloneNode(true);
	
	const legend = document.querySelector('#lastfm-status');
	const artImg = clone.querySelector('.listening-art');
	// const bgImg = clone.querySelector('.listening-bg');
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
		
		if (data.url) {
			link.innerHTML = `<span class="splash">${response["@attr"].total}</span> total scrobbles`;
		}
		
		document.querySelector('.lastfm').innerHTML = '';
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

function getOrdinalSuffix(day) {
    if (day > 3 && day < 21) return 'th';
    switch (day % 10) {
        case 1: return 'st';
        case 2: return 'nd';
        case 3: return 'rd';
        default: return 'th';
    }
}

function formatDate(dateObj) {
    const month = dateObj.toLocaleDateString('en-US', { month: 'short' }); // "Jul"
    const day = dateObj.getDate(); // 26
    return `${month} ${day}${getOrdinalSuffix(day)}`; // "Jul 26th"
}

function renderPaddedStat(value, width = 5) {
    const str = value.toString().padStart(width, '0');
    const leadingZeros = str.length - value.toString().length;
    return [...str].map((digit, i) =>
        `<span${i < leadingZeros ? ' class="subtext"' : ''}>${digit}</span>`
    ).join('');
}

async function updateSiteStats() {
	const username = "ivrry";
	try {
		const request = await fetch(`https://nekoweb.org/api/site/info/${username}.nekoweb.org`);
		const response = await request.json();
		const date = formatDate(new Date(response.updated_at));
		console.log(response)
		document.getElementById('visitors').innerHTML = renderPaddedStat(response.views);
		document.getElementById('followers').innerHTML = renderPaddedStat(response.followers)
		document.getElementById('date').textContent = date

	} catch (err) {
		console.error(err)
	}
}
document.addEventListener('DOMContentLoaded', () => {
	updateLastFM();
	updateSiteStats();
});