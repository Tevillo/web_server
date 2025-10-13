window.addEventListener("load", loadHandler);

function loadHandler(event) {
    add_event_listeners();
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
        const response = await fetch("http://72.181.111.213:3000/server", {
            // Replace with your server's URL
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(data),
        });

        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
    } catch (error) {
        // document.getElementById('response').innerText = 'Error: ' + error.message;
        console.error("Error :(");
    }
}
