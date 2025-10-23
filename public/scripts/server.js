window.addEventListener("load", loadHandler);

function loadHandler(event) {
    loadServers();
}

function loadServers() {
    const serversDiv = document.getElementById("servers");

    fetch("/get_servers", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({}),
    })
        .then((response) => {
            if (!response.ok) {
                throw new Error("Network response was not ok");
            }
            return response.text();
        })
        .then((html) => {
            serversDiv.innerHTML = html;
            add_event_listeners();
        })
        .catch((error) => {
            serversDiv.innerHTML =
                '<p style="color: #ff6b6b;">Error loading servers: ' +
                error.message +
                "</p>";
        });
}
function responseReceivedHandler() {
    //We received something that is healthy
    if (this.status === 200) {
        //Creating a new, empty div
        //Adding the response to this new div
        info.innerHTML = this.response;
        //Appending the div to the subInfo tag
    } else {
        //Handling an unsuccessful database lookup
        info.innerHTML = "Query error";
    }
}

function add_event_listeners() {
    const vals = ["mc_bedrock", "mc_java", "mc_all_the_mods"];
    for (let i = 0; i < vals.length; i++) {
        const server = vals[i];
        const button = document.getElementById(server);
        button.addEventListener("click", async function (event) {
            a = get_action(button);

            const data = {
                server: server,
                action: a,
            };
            post_data(data);
        });
    }
}

function get_action(button) {
    switch (parseInt(button.value)) {
        case 0:
            return "start";
        case 1:
            return "close";
        case 2:
            return "restart";
        default:
            return "other";
    }
}

async function post_data(data) {
    try {
        const response = await fetch("https://72.181.111.213:3000/server", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(data),
        });

        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        const server_data = await response.json();
        if (!server_data.error_msg) {
            loadServers();
        } else {
            handlePopup(server_data);
        }
    } catch (error) {
        // document.getElementById('response').innerText = 'Error: ' + error.message;
        console.error("Error :( " + error.message);
    }
}

function handlePopup(server_data) {
    const popup = document.getElementById("popup");
    const goBackButton = document.getElementById("go-back");
    const status = document.getElementById("server-status");
    const img = document.getElementById("server-image");
    popup.style.display = "block";
    status.innerText =
        "Failed to " +
        server_data.action +
        " " +
        server_data.server +
        ": " +
        server_data.error_msg;

    if (server_data.server == "mc_java" || server_data.server == "mc_bedrock") {
        img.src = "/public/images/minecraft.png";
    } else if (server_data.server == "mc_all_the_mods") {
        img.src = "/public/images/all_the_mods.png";
    } else {
        img.src = "/public/images/error.png";
    }
    goBackButton.addEventListener("click", function () {
        popup.style.display = "none";
    });
}
