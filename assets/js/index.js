async function updateLastFM() {
	const template = document.getElementById('lastfm-template');
	const clone = template.content.cloneNode(true);
	
	const legend = clone.querySelector('legend');
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
		artImg.alt = `album art for ${data.album?.["#text"] || data.name}`
		title.textContent = data.name;
		title.setAttribute("data-title", data.name);
		album.textContent = data.album?.["#text"] || data.name;
		artist.textContent = data.artist?.["#text"];
		
		if (data.url) {
			link.innerHTML = `<a href="${data.url}">listen</a> on lastfm - <span class="splash">${response["@attr"].total}</span> total scrobbles`;
		}
		
		document.querySelector('.lastfm').appendChild(clone);
		
	} catch (err) {

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

async function updateStatus() {
	await fetch("https://status.cafe/users/ivrry/status.json")
		.then( r => r.json() )
		.then( r => {
			document.getElementById("thoughts-feeling").innerHTML = "feeling: " + r.face + " / "
			document.getElementById("thoughts-timestamp").innerHTML = r.timeAgo
			document.getElementById("thoughts-content").innerHTML = r.content
		})
}

document.addEventListener('DOMContentLoaded', () => {
	updateLastFM();
	updateStatus();
});

document.querySelector(".bendy").addEventListener('mousedown', () => {
	const audio = new Audio("/assets/audio/squeak-in.mp3");
	
	audio.volume = .6
	audio.play();
});

document.querySelector(".bendy").addEventListener('mouseup', () => {
	const audio = new Audio("/assets/audio/squeak-out.mp3");
	
	audio.volume = .6
	audio.play();
});