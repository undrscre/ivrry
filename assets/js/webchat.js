const chatbox = document.querySelector("#webchat-chatbox");
const sendbtn = document.querySelector("#webchat-sendbtn");
const mesgbox = document.querySelector("#webchat-mesgbox");
const namebox = document.querySelector("#webchat-namebox");

let endpoint  = "wss://nerds.my.id:2763";
let ws = null;

function displayMessage(name, message) {
	let container = document.createElement("span")
    let namen = document.createElement("b");
	let messagen = document.createElement("span");
	namen.classList.add("splash");
    namen.innerText = `${name || "anon"}:`;
	messagen.innerText = ` ${message}`;
    container.appendChild(namen);
	container.appendChild(messagen);
	chatbox.appendChild(container);
	chatbox.appendChild(document.createElement("br")); // oh my god this code is so stupid
	chatbox.appendChild(document.createElement("br"));
}

sendbtn.addEventListener('click', () => {
    if (!ws || ws.readyState === WebSocket.CLOSED) {
		chatbox.innerHTML = "";
        ws = new WebSocket(endpoint);

        ws.onopen = () => {
            displayMessage("SYSTEM","connected! if you just wait a bit around i'll probably respond :3 if not, you could probably drop your handle or something ^_^ i'll read it later and get back to you!!");
            sendMessage();
        };

        ws.onmessage = (e) => displayMessage("ivy",e.data);
        ws.onerror = (e) => console.error("ws error:", e);
        ws.onclose = () => displayMessage("SYSTEM","disconnected :(");
    } else {
        sendMessage();
    }
});

function sendMessage() {
    if (ws.readyState === WebSocket.OPEN) {
        const message = {
            "name": namebox.value || "anon",
            "mesg": mesgbox.value
        };
        ws.send(JSON.stringify(message));
        displayMessage(namebox.value, mesgbox.value);
        mesgbox.value = "";
    } else {
        console.error("ws not connected yet, waiting...");
    }
}
